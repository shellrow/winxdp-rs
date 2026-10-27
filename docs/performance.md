# Benchmarks

The ring microbenchmark measures reserve/submit/release helpers in process memory:

```powershell
cargo bench -p winxdp-sys --bench rings
```

It does not measure driver or NIC throughput.

For TX measurements, `packet_bench` accepts an isolated interface index, raw
Ethernet frame file, batch size, duration in seconds, and `copy` or `borrow`:

```powershell
cargo run -p winxdp --release --example packet_bench -- --isolated <interface-index> frame.bin 64 30 borrow
```

The template must be at least 60 bytes with experimental EtherType `0x88b5`.
Both modes send the same header and a `0x5a` payload. Copy mode copies a prepared
frame; borrow mode fills the UMEM frame directly. Both include reservation zeroing.

JSON output reports enqueued frames, explicitly observed completions, backpressure
iterations and invalid TX descriptors. Automatic recycling may consume additional
completions. A successful final drain proves reclamation, not peer delivery.

For throughput or loss measurements, use a peer that counts received frames.
Record the NIC and driver, OS/XDP versions, frame and batch sizes, queue count,
CPU affinity, warm-up and duration. Compare repeated runs and include CPU use,
loss and latency distributions. Keep wire overhead distinct from payload bytes.
No representative hardware performance results are published for v0.1.0.
