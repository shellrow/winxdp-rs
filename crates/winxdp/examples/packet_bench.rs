#[cfg(windows)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use std::time::{Duration, Instant};
    let a: Vec<_> = std::env::args().collect();
    if a.len() != 7 || a[1] != "--isolated" {
        return Err(
            "usage: packet_bench --isolated IFINDEX FRAME_FILE BATCH SECONDS copy|borrow".into(),
        );
    }
    let mut bytes = std::fs::read(&a[3])?;
    if bytes.len() < 60 || bytes[12..14] != [0x88, 0xb5] {
        return Err(
            "benchmark requires a >=60-byte experimental EtherType 0x88b5 frame template".into(),
        );
    }
    bytes[14..].fill(0x5a);
    let batch: u32 = a[4].parse()?;
    let seconds: u64 = a[5].parse()?;
    if batch == 0 || batch > 4096 || seconds == 0 || !["copy", "borrow"].contains(&a[6].as_str()) {
        return Err("invalid benchmark parameters".into());
    }
    let runtime = winxdp::Runtime::open()?;
    let mut socket = winxdp::Socket::builder(&runtime)
        .interface(a[2].parse()?)
        .build()?;
    let frames = vec![bytes.as_slice(); batch as usize];
    let start = Instant::now();
    let mut enqueued = 0u64;
    let mut completed = 0u64;
    let mut pressure = 0u64;
    while start.elapsed() < Duration::from_secs(seconds) {
        let n = if a[6] == "copy" {
            socket.send_batch(&frames)?
        } else {
            socket.with_tx_batch(batch, |b| {
                let mut n = 0;
                while let Some(mut f) = b.next_frame()? {
                    if bytes.len() > f.bytes_mut().len() {
                        return Err(winxdp::Error::InvalidConfiguration("frame exceeds chunk"));
                    }
                    f.bytes_mut()[..14].copy_from_slice(&bytes[..14]);
                    f.bytes_mut()[14..bytes.len()].fill(0x5a);
                    f.submit(bytes.len())?;
                    n += 1;
                }
                Ok(n)
            })?
        };
        enqueued += n as u64;
        if n == 0 {
            pressure += 1;
        }
        completed += socket.drain_completions()? as u64;
        socket.flush_tx()?;
    }
    let stats = socket.stats()?;
    let elapsed = start.elapsed().as_secs_f64();
    socket.close(Duration::from_secs(5))?;
    println!(
        "{{\"mode\":\"{}\",\"frame_bytes\":{},\"batch\":{},\"elapsed_seconds\":{},\"enqueued\":{},\"observed_completions\":{},\"pressure_iterations\":{},\"invalid_tx\":{}}}",
        a[6],
        bytes.len(),
        batch,
        elapsed,
        enqueued,
        completed,
        pressure,
        stats.TxInvalidDescriptors
    );
    Ok(())
}
#[cfg(not(windows))]
fn main() {
    eprintln!("Windows x64 is required");
}
