#[cfg(windows)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use nex_packet::{
        builder::{
            ethernet::EthernetPacketBuilder, ipv4::Ipv4PacketBuilder, udp::UdpPacketBuilder,
        },
        ethernet::EtherType,
        ip::IpNextProtocol,
    };
    let a: Vec<_> = std::env::args().collect();
    if a.len() != 7 && a.len() != 8 {
        return Err("usage: tx_nex_packet IFINDEX SRC_MAC DST_MAC SRC_IPV4 DST_IPV4 FRAME_OUTPUT [--send-isolated]".into());
    }
    let src: std::net::Ipv4Addr = a[4].parse()?;
    let dst: std::net::Ipv4Addr = a[5].parse()?;
    let udp = UdpPacketBuilder::new(src.into(), dst.into())
        .source(49001)
        .destination(49000)
        .payload(b"winxdp isolated test".to_vec().into())
        .to_bytes()?;
    let ip = Ipv4PacketBuilder::new()
        .source(src)
        .destination(dst)
        .protocol(IpNextProtocol::Udp)
        .payload(udp)
        .to_bytes()?;
    let frame = EthernetPacketBuilder::new()
        .source(a[2].parse()?)
        .destination(a[3].parse()?)
        .ethertype(EtherType::Ipv4)
        .payload(ip)
        .to_bytes();
    let mut bytes = frame.to_vec();
    bytes.resize(bytes.len().max(60), 0);
    std::fs::write(&a[6], &bytes)?;
    println!(
        "Wrote {} bytes; checksums computed by nex-packet 0.27.0",
        bytes.len()
    );
    if a.len() == 8 {
        if a[7] != "--send-isolated" {
            return Err("unknown option".into());
        }
        let r = winxdp::Runtime::open()?;
        let mut s = winxdp::Socket::builder(&r)
            .interface(a[1].parse()?)
            .build()?;
        s.send(&bytes)?;
        s.flush_tx()?;
        s.close(std::time::Duration::from_secs(2))?;
    }
    Ok(())
}
#[cfg(not(windows))]
fn main() {
    eprintln!("Windows x64 is required");
}
