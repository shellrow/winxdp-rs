# eBPF receive steering

The optional `ebpf` feature uses eBPF for Windows v1.4.0 and the XDP provider.
Install them separately following the
[upstream setup guide](https://github.com/microsoft/xdp-for-windows/blob/v1.4.0/docs/ebpf.md).
The adapter requires a trusted `ebpfapi.dll` and a verified, signed native `.sys`
program. The crate does not distribute a driver, certificate or compiled program.

## Program contract

`Steering::redirect_selected` takes activated RX sockets on one interface, an
absolute native artifact path and an exact program name. The program must expose
`xsk_map` as an XSKMAP with 64-bit queue keys and HANDLE-sized values. The adapter
checks the map layout, inserts the sockets and attaches an owned link.

The program controls which traffic is redirected. It must pass unmatched traffic
and use `bpf_redirect_map(..., XDP_PASS)` for lookup failure. Redirected frames do
not continue to the Windows stack. The adapter does not inspect program policy.

The [source example](../crates/winxdp/examples/bpf/selected.c) selects nonfragmented
IPv4 UDP destination port 49000. Compile it against the pinned eBPF headers and XDP
headers, then verify, convert and sign it using the
[upstream native workflow](https://github.com/microsoft/ebpf-for-windows/blob/v1.4.0/docs/NativeCodeGeneration.md).
Compiling an ELF `.o` alone is not sufficient for the native runtime.

## Run an example

Use a dedicated adapter and a peer on the same isolated network. Replace the
interface index and paths with those for your machine:

```powershell
$env:WINXDP_ISOLATED = '1'
$env:WINXDP_IFINDEX = '<interface-index>'
$env:WINXDP_EBPF_DLL = 'C:\path\to\ebpfapi.dll'
$env:WINXDP_EBPF_PROGRAM = 'C:\path\to\selected.sys'
$env:WINXDP_EBPF_NAME = 'winxdp_selected'
cargo run -p winxdp --features ebpf --example rx_batch
```

Send UDP traffic from the peer to port 49000. The example waits up to five seconds
and exits after processing a batch. A timeout means no matching frame arrived.
`EbpfRuntime::load` is unsafe because loading a DLL executes its initialization code.

## Ownership

`Steering` owns its link, map entries, object and sockets. It detaches the link,
removes entries and closes the object before releasing sockets. It never detaches
by interface index or modifies another process's map. `SteeredSocket` lends packet
I/O without allowing a socket to be moved out of its owner.

For externally managed steering, use the unsafe raw-handle API and then call
`acknowledge_external_steering`. The external owner must release its references
before the socket is dropped; the acknowledgment does not verify external state.

## Detach and reload

Native driver unload is asynchronous upstream. An immediate reload after detach
can fail with `Io { code: -2, context: "bpf_object__load" }`. The hardware test
retries this observed post-detach error at 25 ms intervals for at most two seconds,
recreating sockets on each attempt. The library returns the original error without
hidden retries. Missing files and initial-load failures need separate diagnosis.
