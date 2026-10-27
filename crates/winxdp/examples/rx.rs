#[cfg(windows)]
mod common;
#[cfg(windows)]
fn main() -> winxdp::Result<()> {
    let mut steering = common::selected_rx(1)?;
    let mut buffer = vec![0; 2048];
    let mut socket = steering.socket(0).unwrap();
    socket.wait_rx(std::time::Duration::from_secs(5))?;
    let received = socket.recv_into(&mut buffer)?;
    println!("{received:?} {:02x?}", &buffer[..received.length]);
    Ok(())
}
#[cfg(not(windows))]
fn main() {
    eprintln!("Windows x64 is required");
}
