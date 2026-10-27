# Architecture

`winxdp-sys` compiles 32 small C forwarding functions against 24 vendored
upstream headers. It links ntdll for the inline upstream implementation.
Public types and constants are checked-in bindgen output. A separate C ABI
probe checks layouts against the compiler used by the consumer build.
`native/abi.c` is test support, never an alternative driver implementation.

`winxdp` owns an XSK handle, a page-aligned VirtualAlloc region, four SPSC rings,
and frame-state/free-list vectors. Initialization is create, register UMEM,
set optional hooks, bind, enable extensions, configure ring sizes, activate,
query ring layouts, negotiate extension offsets, then publish the fill ring.
A failure drops the handle before freeing UMEM. No network configuration is
changed by Runtime::open or Interface::list.

RX and TX use disjoint chunks in a common region. RX reserves exactly the fill
ring capacity; remaining chunks are TX's pool. Frame size need not be a power
of two; ring sizes must be. UMEM total size is nonzero and fits UINT32, matching
the pinned implementation. Headroom fits UINT16 and leaves payload capacity.
Scatter/gather buffers are not supported by the safe API.

RX: FreeRx -> DriverRx -> BorrowedRx -> FreeRx. Refill then returns FreeRx to
the driver. TX: FreeTx -> ReservedTx -> DriverTx -> FreeTx. A dropped reservation
returns to FreeTx; a submitted descriptor remains unavailable until completion.
The Rust ring implementation uses acquire loads and release publications, with
wrapping index arithmetic. Unknown flag bits are preserved.

The optional eBPF adapter loads an explicitly selected DLL, validates XSKMAP
key/value sizes and type, loads a native artifact, fills a newly owned map, and
attaches an owned link. It destroys the link, removes entries, closes the object,
and only then drops sockets. Restricted packet-I/O borrows prevent moving
sockets out while map entries reference them.

The optional `async` module owns one pinned OVERLAPPED and a private event per
notification. Its exclusive socket borrow prevents concurrent safe ring/handle
operations. A wait timeout keeps the request alive. Cancellation and Drop observe
final completion before releasing storage; Drop can block on driver latency.
This Windows wait primitive is not tied to an executor and does not implement Future.

Examples/tests/benchmarks live in their owning Cargo packages rather than at
the workspace root so standalone source packages do not reference parent files.
