#[cfg(windows)]
mod common;
#[cfg(windows)]
fn main() -> winxdp::Result<()> {
    let count = std::env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(1);
    let mut steering = common::selected_rx(count)?;
    let end = std::time::Instant::now() + std::time::Duration::from_secs(10);
    let mut counts = vec![0u64; count as usize];
    while std::time::Instant::now() < end {
        for (q, total) in counts.iter_mut().enumerate() {
            steering.socket(q).unwrap().with_rx_batch(64, |b| {
                *total += b.len() as u64;
                Ok(())
            })?;
        }
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
    println!("Per-queue packets: {counts:?}");
    Ok(())
}
#[cfg(not(windows))]
fn main() {
    eprintln!("Windows x64 is required");
}
