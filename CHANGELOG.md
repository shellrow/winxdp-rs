# Changelog

## 0.1.0 (unreleased)

Initial release.

- Windows x64 bindings for XDP for Windows 1.4.0 / API v3.
- Socket configuration, UMEM allocation, copying and scoped RX/TX batches.
- Completion recycling, packet metadata, notifications and bounded TX drain.
- Optional owned eBPF steering and overlapped notification APIs.
- Examples for batch I/O, forwarding, multiple queues and `nex-packet` integration.
- Native ABI checks, ownership tests and opt-in hardware tests.

See [compatibility](docs/upstream-compatibility.md) for supported configurations
and [validation](docs/validation.md) for tested behavior and remaining coverage.
