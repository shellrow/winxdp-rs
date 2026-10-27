#[cfg(windows)]
mod common;
#[cfg(windows)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let destination: u32 = std::env::args()
        .nth(1)
        .ok_or("usage: bridge ISOLATED_TX_IFINDEX; RX configured by WINXDP_* variables")?
        .parse()?;
    let source: u32 = std::env::var("WINXDP_IFINDEX")?.parse()?;
    if source == destination {
        return Err("bridge interfaces must differ".into());
    }
    let mut steering = common::selected_rx(1)?;
    let runtime = winxdp::Runtime::open()?;
    let mut tx = winxdp::Socket::builder(&runtime)
        .interface(destination)
        .build()?;
    let end = std::time::Instant::now() + std::time::Duration::from_secs(10);
    let mut forwarded = 0;
    let mut dropped = 0;
    while std::time::Instant::now() < end {
        steering.socket(0).unwrap().with_rx_batch(64, |batch| {
            for f in batch.iter() {
                match tx.send(f.bytes()) {
                    Ok(_) => forwarded += 1,
                    Err(winxdp::Error::WouldBlock) => dropped += 1,
                    Err(e) => return Err(e),
                }
            }
            Ok(())
        })?;
        tx.flush_tx()?;
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
    tx.close(std::time::Duration::from_secs(2))?;
    println!("One-way copying forwarder: {forwarded} forwarded, {dropped} dropped due to pressure");
    Ok(())
}
#[cfg(not(windows))]
fn main() {
    eprintln!("Windows x64 is required");
}
