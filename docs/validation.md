# Validation

The v0.1.0 baseline was tested on Windows 11 x64 (build 26200), XDP for Windows
1.4.0 and eBPF for Windows 1.4.0, using an Intel 82574L virtual NIC in generic mode.
This covers one configuration, not all Windows versions or physical adapters.

## Completed checks

- Default and all-feature workspace tests, including ownership and compile-fail tests.
- Native ABI, bitfield and error-status comparisons against the vendored headers.
- Rust 1.88 minimum-version check, Clippy, formatting, examples and rustdoc.
- RX-only, TX-only and combined socket creation, activation and repeated close.
- Copying and borrowed RX/TX, headroom, recycling and variable-length UDP roundtrips.
- Signed eBPF program load, map update, attach, detach and reopen; unmatched UDP
  continues through the Windows stack.
- Overlapped completion, timeout and cancellation, including Drop and handle counts.
- RX checksum/timestamp metadata and TX completion timestamp fields.
- Short TX pressure tests with no invalid descriptors reported.

Immediate native program reload after detach can fail while upstream cleanup is
still pending. The hardware test uses bounded retries for the observed load error;
see [eBPF lifetime behavior](ebpf.md#detach-and-reload).

## Remaining coverage

Native-mode traffic, physical NICs, Windows Server, multiqueue traffic, requested
TX checksum computation, VLAN/jumbo frames and sustained loss/resource measurements
remain unvalidated. Metadata field presence does not establish timestamp accuracy
or checksum offload correctness. Driver restart, adapter removal and reconfiguration
during I/O also need dedicated testing.

These limits should be considered when deploying v0.1.0. No packet-rate, latency,
loss-free operation or NIC DMA zero-copy guarantee is made.

## Reproducing tests

See [contributing](../CONTRIBUTING.md) for build checks and
[hardware tests](testing.md) for the isolated-network setup. Ordinary `cargo test`
runs do not inject packets or load a steering program.
