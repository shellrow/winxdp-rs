# winxdp

Rust bindings and packet I/O for [XDP for Windows](https://github.com/microsoft/xdp-for-windows).

`winxdp` provides socket configuration, scoped RX/TX buffers, completion handling,
and optional eBPF steering. `winxdp-sys` exposes the underlying user-mode API.
Both crates target Windows x64 and XDP 1.4.0 (API v3).

## Requirements

- Rust 1.88 or later, with the `x86_64-pc-windows-msvc` toolchain.
- MSVC build tools and a Windows SDK.
- XDP for Windows 1.4.0 installed separately to run packet I/O.
- Administrator access, or access granted through XDP's device permissions.

Builds use vendored headers; Clang and an installed XDP SDK are not required.
The crates do not install drivers or depend on `xdpapi.dll`.

## Usage

```toml
[dependencies]
winxdp = "0.1.0"
```

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

Sockets default to TX only. `send` enqueues a complete Ethernet frame; successful
completion does not confirm delivery to a peer. `recv_into` copies a received
frame. `with_rx_batch` and `with_tx_batch` lend buffers within a callback, avoiding
an additional user-space copy. Sockets are neither `Send` nor `Sync`; create each
socket on the thread that will use it.

RX requires a steering program. The optional eBPF adapter owns its program link,
map entries and sockets. Redirected traffic is removed from the normal Windows
network path; this is not passive capture. See [eBPF setup](docs/ebpf.md).

## Features

All features are disabled by default.

| Feature | Purpose |
|---|---|
| `ebpf` | Steering through a separately installed eBPF runtime and signed native program |
| `async` | Owned Windows overlapped notifications with polling, timed waits and cancellation; no executor dependency or `Future` implementation |
| `legacy-rules` | Deprecated `XdpCreateProgram` binding for compatibility |

## Examples

```powershell
cargo run -p winxdp --example interfaces
cargo run -p winxdp --example capabilities -- --isolated <interface-index>
cargo run -p winxdp --example tx -- --isolated <interface-index> frame.bin
```

Use a dedicated test adapter for packet examples. See
[examples](crates/winxdp/examples) for batch I/O, forwarding, multiple queues,
notification cancellation and packet construction/parsing with `nex-packet`.
`nex-packet` is a development dependency only.

## Status

v0.1.0 is an initial release; the Rust API may change in subsequent versions.
Generic RX/TX, eBPF steering, cancellation and buffer recycling have been tested
on Windows 11 x64 with a virtual NIC. Native-mode traffic, physical NIC coverage
and sustained performance remain to be validated. See the
[validation summary](docs/validation.md) and [compatibility guide](docs/upstream-compatibility.md).

Further documentation: [safety](docs/safety.md), [architecture](docs/architecture.md),
[troubleshooting](docs/troubleshooting.md), [benchmarks](docs/performance.md),
[API coverage](docs/api-coverage.md), and [contributing](CONTRIBUTING.md).

## License

MIT. Vendored XDP headers retain their [upstream license](crates/winxdp-sys/vendor/LICENSE).
