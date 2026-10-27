# winxdp

Packet I/O for Microsoft XDP for Windows, with scoped RX/TX buffers, completion
recycling and optional eBPF steering.

Requires Windows x64, Rust 1.88+, MSVC and a Windows SDK. Install XDP for Windows
1.4.0 separately to run packet I/O. Builds use vendored headers and do not need
Clang or `xdpapi.dll`.

```rust,no_run
use std::time::Duration;
use winxdp::{Result, Runtime, Socket};

fn send_frame(interface_index: u32, frame: &[u8]) -> Result<()> {
    let runtime = Runtime::open()?;
    let mut socket = Socket::builder(&runtime)
        .interface(interface_index)
        .build()?;
    socket.send(frame)?;
    socket.flush_tx()?;
    socket.close(Duration::from_secs(2))
}
```

Sockets default to TX only. RX needs a steering program; redirected packets no
longer follow the normal Windows network path. Use a dedicated adapter when
trying the packet examples.

Optional features (none enabled by default):

- `ebpf`: owned steering using a trusted eBPF runtime DLL and signed native program.
- `async`: owned Windows overlapped notifications, timed waits and cancellation.
  This is a Windows wait API, not a `Future` or Tokio adapter.
- `legacy-rules`: deprecated raw rule-based program creation.

v0.1.0 targets Windows x64 and XDP 1.4.0. Generic RX/TX and eBPF steering are tested
on Windows 11 with a virtual NIC; native-mode traffic and broader hardware
coverage remain unvalidated. The Rust API may change in later releases.

See the [repository](https://github.com/shellrow/winxdp) for examples,
[setup and compatibility](https://github.com/shellrow/winxdp/blob/main/docs/upstream-compatibility.md),
and [safety contracts](https://github.com/shellrow/winxdp/blob/main/docs/safety.md).

Licensed under MIT.
