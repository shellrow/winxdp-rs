#[cfg(windows)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use std::time::Duration;
    use winxdp::{BindMode, Config, InterfaceHandle, Runtime, Socket};
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 3 || args[1] != "--isolated" {
        return Err(
            "usage: capabilities --isolated IFINDEX; opens probe sockets, no traffic injection"
                .into(),
        );
    }
    let index = args[2].parse()?;
    let runtime = Runtime::open()?;
    match InterfaceHandle::open(index).and_then(|interface| interface.rss_capabilities()) {
        Ok(caps) => println!("RSS capabilities: {caps:?}"),
        Err(error) => println!("RSS query failed: {error}"),
    }
    for name in [
        "generic",
        "native",
        "rx_checksum",
        "tx_checksum",
        "rx_timestamp",
        "tx_timestamp",
    ] {
        let config = Config {
            interface: index,
            receive: true,
            bind_mode: if name == "native" {
                BindMode::Native
            } else {
                BindMode::Generic
            },
            rx_checksum: name == "rx_checksum",
            tx_checksum: name == "tx_checksum",
            rx_timestamp: name == "rx_timestamp",
            tx_timestamp: name == "tx_timestamp",
            ..Config::default()
        };
        match Socket::builder(&runtime).config(config).build() {
            Ok(socket) => {
                socket.close(Duration::from_secs(1))?;
                println!("{name}: negotiated successfully (packet semantics not tested)");
            }
            Err(error) => println!("{name}: negotiation failed: {error}"),
        }
    }
    Ok(())
}
#[cfg(not(windows))]
fn main() {
    eprintln!("Windows x64 is required");
}
