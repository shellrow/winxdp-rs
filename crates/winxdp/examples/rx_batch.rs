#[cfg(windows)]
mod common;
#[cfg(windows)]
fn main() -> winxdp::Result<()> {
    let mut steering = common::selected_rx(1)?;
    let mut socket = steering.socket(0).unwrap();
    socket.wait_rx(std::time::Duration::from_secs(5))?;
    socket.with_rx_batch(64, |batch| {
        for packet in batch.iter() {
            println!("{} bytes, {:?}", packet.bytes().len(), packet.metadata());
        }
        Ok(())
    })
}
#[cfg(not(windows))]
fn main() {
    eprintln!("Windows x64 is required");
}
