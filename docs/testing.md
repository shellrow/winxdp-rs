# Hardware tests

Hardware tests are ignored by default. They require administrator access, an
installed XDP runtime, and a dedicated test adapter. Do not select an adapter
used for management or normal application traffic.

## Socket and notification tests

On the Windows test machine, set its adapter index:

```powershell
$env:WINXDP_ISOLATED = '1'
$env:WINXDP_IFINDEX = '<interface-index>'
cargo test -p winxdp --all-features --test hardware lifecycle -- --ignored --exact
cargo test -p winxdp --all-features --test hardware overlapped_cancellation -- --ignored --exact
```

The first test creates/closes RX, TX and combined sockets. The second checks
overlapped timeouts, cancellation, Drop and process handle counts.

## eBPF roundtrip test

Prepare the signed [selective program](ebpf.md). Set these additional variables
on the test machine, using the IPv4 addresses of the isolated network:

```powershell
$env:WINXDP_EBPF_DLL = 'C:\path\to\ebpfapi.dll'
$env:WINXDP_EBPF_PROGRAM = 'C:\path\to\selected.sys'
$env:WINXDP_TEST_IP = '<test-adapter-ipv4>'
$env:WINXDP_PEER = '<peer-ipv4>:49002'
cargo test -p winxdp --all-features --test hardware --no-run
```

On a separate peer with Python, start:

```text
python tools/udp-peer.py --bind <peer-ipv4> --target <test-adapter-ipv4> --os-probe
```

Then, within 25 seconds, run on the Windows test machine:

```powershell
cargo test -p winxdp --all-features --test hardware ebpf_udp_roundtrip -- --ignored --exact --nocapture
```

The test exchanges 256 payloads across four attach/detach cycles. Port 49000 is
redirected to XDP; port 49003 checks the ordinary OS path. Port 49002 belongs to
the peer. Use an isolated network whose firewall policy permits these test flows;
the tests do not change firewall settings. Stop other traffic generators using
these ports before running the test.

Set `WINXDP_TEST_EXTENSIONS=1` to require RX checksum/timestamp and TX timestamp
extensions. This validates metadata availability, not clock calibration or
hardware checksum computation. Native reload uses the bounded retry described
in [eBPF lifetime behavior](ebpf.md#detach-and-reload).

`legacy_udp_roundtrip` provides a separate test of the deprecated rule API.
Run it with the same peer command without `--os-probe`. It does not require a
signed program and does not replace eBPF acceptance testing. Run only one
roundtrip test and one peer at a time.

For a test machine without Rust, build the test executable on a compatible
Windows host with `cargo test --no-run`, copy it to the test machine, set the
same environment variables, and pass `--ignored --exact <test-name>` to the
executable. Retain logs outside the repository and summarize relevant results
in [validation](validation.md).
