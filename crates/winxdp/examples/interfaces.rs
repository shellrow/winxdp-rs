#[cfg(windows)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    for interface in winxdp::Interface::list()? {
        println!("{interface:?}");
    }
    match winxdp::Runtime::open() {
        Ok(r) => println!("XDP: {:?}", r.capabilities()?),
        Err(e) => println!("XDP probe: {e}"),
    }
    Ok(())
}
#[cfg(not(windows))]
fn main() {
    eprintln!("Windows x64 is required");
}
