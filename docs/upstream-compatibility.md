# Compatibility

## Baseline

- XDP for Windows: [v1.4.0](https://github.com/microsoft/xdp-for-windows/tree/v1.4.0),
  commit `d372b52577a724e04fa4c06acb90bbfa4719fc25`.
- eBPF adapter: eBPF for Windows v1.4.0, commit
  `fc0e2f92002e1832fc62e8f68d4b4cc538d57b59`.
- Target: `x86_64-pc-windows-msvc`, Rust 1.88 or later.

API v3 uses inline implementations compiled into the C bridge. The legacy v1/v2
DLL dispatch API is excluded. Older driver versions are not a supported baseline.
Vendored headers retain Microsoft's MIT notices; normalized hashes are recorded
in [upstream-pin.json](upstream-pin.json).

## Support

| Area | v0.1.0 support |
|---|---|
| Windows 11 x64 | Generic RX/TX and signed eBPF tested with a virtual NIC |
| Windows Server | Upstream-supported configurations; not tested by this project |
| Other architectures | No packet-engine ABI support |
| Non-Windows builds | Configuration and error types only |
| Generic mode | NDIS software path; availability checked during bind |
| Native mode | Requires NIC driver support; traffic not yet tested |
| RX | Requires steering; selected packets leave the normal Windows stack |
| TX | Enqueue and completion APIs; success does not prove peer delivery |
| eBPF | Separate runtime, XDP provider and trusted signed native program |
| RSS, QEO, checksum APIs | Upstream experimental interfaces; no additional stability guarantee |
| Legacy rules | Deprecated, available through `legacy-rules` |

Queue IDs select RSS queues, not processors. The default device permissions
require administrator/SYSTEM access. Auto bind mode delegates selection to XDP
and does not establish native or DMA zero-copy support. Extension negotiation is
per socket and depends on the driver and queue state.

The raw API covers the public user-mode declarations in the pinned headers.
Kernel DDIs and implementation-private IOCTLs are excluded. Types use the Windows
x64 ABI, including upstream packing and bitfield layouts; native tests compare
sizes, alignments, offsets and constants. See [API coverage](api-coverage.md).

Upstream references: [usage](https://github.com/microsoft/xdp-for-windows/blob/v1.4.0/docs/usage.md),
[AF_XDP](https://github.com/microsoft/xdp-for-windows/blob/v1.4.0/docs/afxdp.md),
and [eBPF](https://github.com/microsoft/xdp-for-windows/blob/v1.4.0/docs/ebpf.md).
