#[cfg(windows)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 4 || args[1] != "--isolated" {
        return Err("usage: tx_batch --isolated IFINDEX FRAME_FILE".into());
    }
    let bytes = std::fs::read(&args[3])?;
    let r = winxdp::Runtime::open()?;
    let mut s = winxdp::Socket::builder(&r)
        .interface(args[2].parse()?)
        .build()?;
    s.with_tx_batch(64, |batch| {
        while let Some(mut frame) = batch.next_frame()? {
            if bytes.len() > frame.bytes_mut().len() {
                return Err(winxdp::Error::InvalidConfiguration("frame too large"));
            }
            frame.bytes_mut()[..bytes.len()].copy_from_slice(&bytes);
            frame.submit(bytes.len())?;
        }
        Ok(())
    })?;
    s.flush_tx()?;
    s.close(std::time::Duration::from_secs(2))?;
    Ok(())
}
#[cfg(not(windows))]
fn main() {
    eprintln!("Windows x64 is required");
}
