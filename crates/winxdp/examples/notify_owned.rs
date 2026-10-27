#[cfg(windows)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use std::time::Duration;
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 3 || args[1] != "--isolated" {
        return Err("usage: notify_owned --isolated IFINDEX; no packets are injected".into());
    }
    let runtime = winxdp::Runtime::open()?;
    let mut socket = winxdp::Socket::builder(&runtime)
        .interface(args[2].parse()?)
        .build()?;
    let mut pending = socket.notify_async(winxdp::sys::XSK_NOTIFY_FLAG_WAIT_TX as u32)?;
    match pending.wait(Duration::from_millis(50))? {
        Some(result) => println!("Notification flags: {:#x}", result.0),
        None => println!("Wait timed out; cancelling the owned request"),
    }
    pending.cancel()?;
    socket.close(Duration::from_secs(1))?;
    Ok(())
}
#[cfg(not(windows))]
fn main() {
    eprintln!("Windows x64 is required");
}
