# Safety

Memory safety depends on XDP's mapped-memory and ownership contracts. Safe Rust
access to rings and buffers is restricted to the socket and its borrowed batches.

- Socket and interface handles are RAII-owned. Socket fields drop in declaration
  order: handle first, then UMEM. Partial construction uses the same ordering.
- VirtualAlloc commits zero-initialized, page-aligned memory. TX frames are zeroed
  again on reservation. No uninitialized payload bytes become safe slices.
- Safe APIs require exclusive socket access. Socket has an explicit Rc marker
  and raw mappings, making it neither Send nor Sync. No unsafe Send impl exists.
- Ring metadata is validated for nonzero power-of-two size, minimum stride,
  aligned shared indices and arithmetic overflow. Mapping extent is trusted
  to the driver; offsets cannot be independently verified against an OS mapping
  length supplied by the XSK API because none is supplied.
- RX descriptors must identify a chunk assigned to DriverRx, have an aligned
  base, and fit their offset/length in that chunk. Each transitions to BorrowedRx
  before any slice is lent; duplicate references poison the socket.
- RX slices borrow the batch. The batch borrows the socket. Drop releases and
  replenishes all validated descriptors, including callback errors and panics.
- One TX frame can be lent at a time. submit consumes it; its mutable slice cannot
  survive submission. Batch drop publishes the exact submitted count.
- Completion offsets must identify DriverTx chunks. Duplicate/out-of-range
  completions poison the socket. Completion publication is consumed before an
  observer callback, so a panic cannot recycle a frame twice.
- Extension offsets are queried, checked against negotiated strides, and read
  unaligned into owned values. TX timestamps belong to the completion ring,
  not the TX submission ring. RX checksum bits are evaluations, not checksums.
- No ring vector allocation happens in ordinary packet loops. Free-list capacity
  is established at initialization and bounded by validated ownership states.
- Invalid frames exceeding a NIC's runtime maximum can be rejected by the driver
  without a usable completion. Check TxInvalidDescriptors and reopen. The caller
  must respect NIC frame-size limits. Explicit close has a finite completion deadline.
- Raw handle/ring escape hatches are unsafe. Callers must not close/reconfigure
  handles or violate pool ownership and must release external map references
  before the safe socket is destroyed. Raw overlapped state must remain pinned
  until cancellation/completion is observed; no safe async future is offered.
- With feature `async`, Notification exclusively borrows the socket and owns a
  pinned OVERLAPPED and private event. Timeout preserves the pending request.
  Explicit cancellation and Drop wait for the final I/O status before closing
  the event or releasing storage. Forgetting the guard leaks its allocation,
  rather than leaving a dangling kernel pointer. Safe callers cannot close the
  borrowed handle or mutate its rings while a notification is pending.

Drop does not wait for TX completion. It closes handles and releases memory after
handle close. Owned eBPF cleanup uses the runtime's synchronous resource-release
functions; their internal driver latency is outside Rust's control. Explicit
Steering::detach reports link errors. Upstream bpf_link__destroy frees a link even
when detach fails, so the pointer is cleared before the call to prevent double-free.

See [validation](validation.md) for tests of these invariants and the remaining
driver/hardware coverage.
