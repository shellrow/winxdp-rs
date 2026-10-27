#[cfg(windows)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 4 || args[1] != "--isolated" {
        return Err("usage: tx --isolated IFINDEX FRAME_FILE; sends one raw L2 frame".into());
    }
    let bytes = std::fs::read(&args[3])?;
    let r = winxdp::Runtime::open()?;
    let mut s = winxdp::Socket::builder(&r)
        .interface(args[2].parse()?)
        .build()?;
    s.send(&bytes)?;
    s.flush_tx()?;
    s.close(std::time::Duration::from_secs(2))?;
    Ok(())
}
#[cfg(not(windows))]
fn main() {
    eprintln!("Windows x64 is required");
}
