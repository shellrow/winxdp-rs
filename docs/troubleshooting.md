# Troubleshooting

- DriverNotInstalled: XskCreate could not open the driver object. Install the
  supported runtime manually following upstream usage documentation. This library
  never installs drivers or changes certificate/signing policy.
- AccessDenied: the XDP device defaults to administrators/SYSTEM. Use the intended
  account and upstream access-control configuration; the library does not elevate.
- An unknown HRESULT remains Io with its function context. A bind error is not
  automatically relabeled as a missing driver, NIC, queue or incompatible version.
- SteeringNotConfigured: RX needs a program/map association. Acknowledge only
  after the external owner has installed one, or use the owned eBPF adapter.
- No RX packets: verify exact IfIndex, RSS queue ID, map key/type/value sizes,
  matching traffic, native/generic provider mode and successful program attach.
  Unmatched frames should pass to the OS. A kernel debug adapter is not evidence
  of usable XDP support.
- WouldBlock: TX ring/pool is exhausted or RX has no packets. Drain completions,
  flush and use bounded notification. Never retry indefinitely without a deadline.
- TxInvalidDescriptors: the NIC may reject lengths/offload metadata even when
  they fit UMEM. Invalid TX frames can lack completions; close and reopen rather
  than reusing an in-flight buffer. Inspect NIC limits and raw error options.
- CorruptDescriptor: ownership or bounds validation failed. The socket is
  quarantined; close it. Do not try to repair indices from safe code.
- UnsupportedFeature: retain the native code/context and check the compatibility
  matrix. Enabling an extension can fail if an existing queue was activated
  without that offload; it is not automatically reconfigured globally.
- A timeout is finite and reported. Drop releases resources without draining
  packet completions; use close with a deadline when completion matters.

Inspect interfaces without requiring a driver using the `interfaces` example.
Runtime capabilities intentionally report NotQueryable for per-NIC native and
runtime eBPF support. Use raw APIs only when their pointer/lifetime contracts are
understood. For installation and tracing consult the pinned upstream
`docs/usage.md`, `docs/ebpf.md`, and `docs/api/`.
