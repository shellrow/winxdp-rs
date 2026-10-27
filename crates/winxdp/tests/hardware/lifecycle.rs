#![cfg(windows)]
use winxdp::*;
#[test]
#[ignore = "requires an installed driver and explicit dedicated test interface"]
fn lifecycle() {
    assert_eq!(std::env::var("WINXDP_ISOLATED").as_deref(), Ok("1"));
    let index = std::env::var("WINXDP_IFINDEX")
        .expect("dedicated interface index")
        .parse()
        .unwrap();
    let runtime = Runtime::open().unwrap();
    for (rx, tx) in [(false, true), (true, false), (true, true)] {
        for _ in 0..10 {
            let socket = Socket::builder(&runtime)
                .interface(index)
                .receive(rx)
                .transmit(tx)
                .bind_mode(BindMode::Generic)
                .build()
                .unwrap();
            socket.stats().unwrap();
            socket.close(std::time::Duration::from_secs(1)).unwrap();
        }
    }
}
#[test]
fn interface_enumeration_is_read_only() {
    assert!(!Interface::list().unwrap().is_empty());
}

#[cfg(feature = "async")]
#[test]
#[ignore = "requires an installed driver and explicit dedicated test interface"]
fn overlapped_cancellation() {
    use std::time::Duration;
    use windows_sys::Win32::System::Threading::{GetCurrentProcess, GetProcessHandleCount};
    assert_eq!(std::env::var("WINXDP_ISOLATED").as_deref(), Ok("1"));
    let index = std::env::var("WINXDP_IFINDEX").unwrap().parse().unwrap();
    let runtime = Runtime::open().unwrap();
    let mut socket = Socket::builder(&runtime).interface(index).build().unwrap();
    let handles = || {
        let mut count = 0;
        assert_ne!(
            unsafe { GetProcessHandleCount(GetCurrentProcess(), &mut count) },
            0
        );
        count
    };
    // Warm up the driver's notification path before taking the resource baseline.
    drop(
        socket
            .notify_async(sys::XSK_NOTIFY_FLAG_WAIT_TX as u32)
            .unwrap(),
    );
    let before = handles();
    for _ in 0..100 {
        let mut pending = socket
            .notify_async(sys::XSK_NOTIFY_FLAG_WAIT_TX as u32)
            .unwrap();
        assert!(pending.wait(Duration::ZERO).unwrap().is_none());
        assert!(pending.wait(Duration::from_millis(1)).unwrap().is_none());
        assert!(pending.cancel().unwrap().is_none());
        // Drop must cancel too, and leave the socket reusable.
        drop(
            socket
                .notify_async(sys::XSK_NOTIFY_FLAG_WAIT_TX as u32)
                .unwrap(),
        );
    }
    assert!(handles() <= before + 2, "notification handle leak");
    socket.close(Duration::from_secs(1)).unwrap();
}

#[cfg(feature = "legacy-rules")]
#[test]
#[ignore = "requires dedicated NIC and tools/udp-peer.py; legacy steering is test-only"]
#[allow(deprecated)]
fn legacy_udp_roundtrip() {
    use std::{
        ptr,
        time::{Duration, Instant},
    };
    use windows_sys::Win32::Foundation::{CloseHandle, HANDLE};
    assert_eq!(std::env::var("WINXDP_ISOLATED").as_deref(), Ok("1"));
    let index = std::env::var("WINXDP_IFINDEX").unwrap().parse().unwrap();
    let runtime = Runtime::open().unwrap();
    let mut socket = Socket::builder(&runtime)
        .interface(index)
        .receive(true)
        .rx_checksum(std::env::var("WINXDP_TEST_EXTENSIONS").as_deref() == Ok("1"))
        .rx_timestamp(std::env::var("WINXDP_TEST_EXTENSIONS").as_deref() == Ok("1"))
        .tx_timestamp(std::env::var("WINXDP_TEST_EXTENSIONS").as_deref() == Ok("1"))
        .bind_mode(BindMode::Generic)
        .rx_ring_size(64)
        .tx_ring_size(64)
        .frame_count(128)
        .headroom(32)
        .build()
        .unwrap();
    let extensions = socket.configuration().rx_timestamp;
    struct Program(HANDLE);
    impl Drop for Program {
        fn drop(&mut self) {
            unsafe {
                CloseHandle(self.0);
            }
        }
    }
    // Declare AFTER socket: on panic the program closes before its target socket.
    let mut program = Program(ptr::null_mut());
    let mut rule: sys::XDP_RULE = unsafe { std::mem::zeroed() };
    rule.Match = sys::XDP_MATCH_UDP_DST;
    rule.Pattern.Port = 49000u16.to_be();
    rule.Action = sys::XDP_PROGRAM_ACTION_REDIRECT;
    unsafe {
        *rule.__bindgen_anon_1.Redirect.as_mut() = sys::XDP_REDIRECT_PARAMS {
            TargetType: sys::XDP_REDIRECT_TARGET_TYPE_XSK,
            Target: socket.raw_handle(),
        };
    }
    let hook = sys::XDP_HOOK_ID {
        Layer: sys::XDP_HOOK_L2,
        Direction: sys::XDP_HOOK_RX,
        SubLayer: sys::XDP_HOOK_INSPECT,
    };
    let status = unsafe {
        sys::XdpCreateProgram(
            index,
            &hook,
            0,
            sys::XDP_CREATE_PROGRAM_FLAG_GENERIC,
            &rule,
            1,
            &mut program.0,
        )
    };
    assert!(status >= 0, "XdpCreateProgram: {status:#x}");
    socket.acknowledge_external_steering();
    let deadline = Instant::now() + Duration::from_secs(30);
    let mut received = std::collections::BTreeSet::new();
    while received.len() < 256 {
        assert!(
            Instant::now() < deadline,
            "received only {} unique packets",
            received.len()
        );
        #[cfg(feature = "async")]
        {
            let mut pending = socket
                .notify_async((sys::XSK_NOTIFY_FLAG_POKE_RX | sys::XSK_NOTIFY_FLAG_WAIT_RX) as u32)
                .unwrap();
            if let Some(flags) = pending.wait(Duration::from_millis(20)).unwrap() {
                assert_eq!(pending.wait(Duration::ZERO).unwrap().unwrap().0, flags.0);
            }
        }
        let packet = if received.len() % 2 == 0 {
            let mut buffer = [0u8; 2048];
            match socket.recv_into(&mut buffer) {
                Ok(info) => {
                    assert_ne!(info.truncated(), Some(true));
                    if extensions {
                        assert!(info.metadata.timestamp.is_some());
                        assert!(info.metadata.checksum.is_some());
                    }
                    Some(buffer[..info.length].to_vec())
                }
                Err(Error::WouldBlock) => None,
                Err(e) => panic!("copy receive: {e}"),
            }
        } else {
            socket
                .with_rx_batch(8, |batch| {
                    Ok(batch.frame(0).map(|frame| {
                        assert_ne!(frame.truncated(), Some(true));
                        if extensions {
                            assert!(frame.metadata().timestamp.is_some());
                            assert!(frame.metadata().checksum.is_some());
                        }
                        frame.bytes().to_vec()
                    }))
                })
                .unwrap()
        };
        let Some(mut packet) = packet else {
            match socket.wait_rx(Duration::from_millis(20)) {
                Ok(_) | Err(Error::TimedOut) => {}
                Err(e) => panic!("receive notification: {e}"),
            }
            continue;
        };
        assert!(packet.len() >= 50);
        assert_eq!(&packet[12..14], &[8, 0]);
        assert_eq!(packet[14], 0x45);
        assert_eq!(packet[23], 17);
        let sequence = u64::from_be_bytes(packet[42..50].try_into().unwrap());
        assert!(sequence < 256);
        received.insert(sequence);
        // Swapping the endpoints preserves IPv4/UDP one's-complement checksums.
        for i in 0..6 {
            packet.swap(i, i + 6);
        }
        for i in 0..4 {
            packet.swap(26 + i, 30 + i);
        }
        for i in 0..2 {
            packet.swap(34 + i, 36 + i);
        }
        if sequence % 2 == 0 {
            socket.send(&packet).unwrap();
        } else {
            socket
                .with_tx_batch(1, |batch| {
                    let mut frame = batch.next_frame()?.ok_or(Error::WouldBlock)?;
                    frame.bytes_mut()[..packet.len()].copy_from_slice(&packet);
                    frame.submit(packet.len())
                })
                .unwrap();
        }
        if extensions {
            socket
                .notify(
                    (sys::XSK_NOTIFY_FLAG_POKE_TX | sys::XSK_NOTIFY_FLAG_WAIT_TX) as u32,
                    Duration::from_secs(2),
                )
                .unwrap();
            assert_eq!(
                socket
                    .drain_completions_with(|completion| {
                        assert!(completion.timestamp.is_some());
                    })
                    .unwrap(),
                1
            );
        } else {
            socket.flush_tx().unwrap();
        }
    }
    assert_eq!(socket.stats().unwrap().TxInvalidDescriptors, 0);
    drop(program);
    socket.close(Duration::from_secs(2)).unwrap();
    println!("256 unique UDP packets received and echoed; copy/borrow APIs and headroom exercised");
}

#[cfg(feature = "ebpf")]
#[test]
#[ignore = "requires an explicitly trusted installed eBPF runtime DLL"]
fn ebpf_runtime_symbols() {
    let dll = std::env::var("WINXDP_EBPF_DLL").expect("trusted absolute runtime DLL path");
    for _ in 0..10 {
        let runtime = unsafe { ebpf::EbpfRuntime::load(std::path::Path::new(&dll)) }.unwrap();
        drop(runtime);
    }
}

#[cfg(all(feature = "ebpf", feature = "async"))]
#[test]
#[ignore = "requires a trusted signed native program, isolated NIC and tools/udp-peer.py"]
fn ebpf_udp_roundtrip() {
    use std::{
        path::Path,
        time::{Duration, Instant},
    };
    assert_eq!(std::env::var("WINXDP_ISOLATED").as_deref(), Ok("1"));
    let index = std::env::var("WINXDP_IFINDEX").unwrap().parse().unwrap();
    let dll = std::env::var("WINXDP_EBPF_DLL").unwrap();
    let artifact = std::env::var("WINXDP_EBPF_PROGRAM").unwrap();
    let os_socket = std::net::UdpSocket::bind(format!(
        "{}:49003",
        std::env::var("WINXDP_TEST_IP").unwrap()
    ))
    .unwrap();
    os_socket
        .connect(std::env::var("WINXDP_PEER").unwrap())
        .unwrap();
    os_socket.set_nonblocking(true).unwrap();
    let ebpf = unsafe { ebpf::EbpfRuntime::load(Path::new(&dll)) }.unwrap();
    let runtime = Runtime::open().unwrap();
    let extensions = std::env::var("WINXDP_TEST_EXTENSIONS").as_deref() == Ok("1");
    let deadline = Instant::now() + Duration::from_secs(30);
    for cycle in 0..4u64 {
        let reopen_deadline = Instant::now() + Duration::from_secs(2);
        let mut retries = 0;
        let mut steering = loop {
            let socket = Socket::builder(&runtime)
                .interface(index)
                .receive(true)
                .bind_mode(BindMode::Generic)
                .rx_ring_size(64)
                .tx_ring_size(64)
                .frame_count(128)
                .headroom(32)
                .rx_checksum(extensions)
                .rx_timestamp(extensions)
                .tx_timestamp(extensions)
                .build()
                .unwrap();
            match ebpf::Steering::redirect_selected(
                &ebpf,
                vec![socket],
                Path::new(&artifact),
                c"winxdp_selected",
            ) {
                Ok(owner) => break owner,
                Err(Error::Io {
                    code: -2,
                    context: "bpf_object__load",
                }) if cycle > 0 && Instant::now() < reopen_deadline => {
                    // Native module unload is queued by the runtime. Retry only
                    // the observed post-detach error, with fresh sockets and a deadline.
                    retries += 1;
                    std::thread::sleep(Duration::from_millis(25));
                }
                Err(error) => panic!("cycle {cycle} load: {error}"),
            }
        };
        println!("eBPF attach cycle {cycle}, post-detach load retries: {retries}");
        let mut os_received = false;
        let mut socket = steering.socket(0).unwrap();
        let mut received = std::collections::BTreeSet::new();
        while received.len() < 64 {
            os_socket.send(b"winxdp-os-pass").unwrap();
            let mut probe = [0u8; 64];
            match os_socket.recv(&mut probe) {
                Ok(length) => {
                    assert_eq!(&probe[..length], b"winxdp-os-pass");
                    os_received = true;
                }
                Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {}
                Err(error) => panic!("OS UDP probe: {error}"),
            }
            assert!(
                Instant::now() < deadline,
                "cycle {cycle}: only {} packets",
                received.len()
            );
            let packet = socket
                .with_rx_batch(1, |batch| {
                    Ok(batch.frame(0).map(|frame| {
                        assert_ne!(frame.truncated(), Some(true));
                        if extensions {
                            assert!(frame.metadata().timestamp.is_some());
                            assert!(frame.metadata().checksum.is_some());
                        }
                        frame.bytes().to_vec()
                    }))
                })
                .unwrap();
            let Some(mut packet) = packet else {
                match socket.wait_rx(Duration::from_millis(20)) {
                    Ok(_) | Err(Error::TimedOut) => {}
                    Err(error) => panic!("receive wait: {error}"),
                }
                continue;
            };
            assert!(packet.len() >= 50);
            assert_eq!(packet[14], 0x45);
            assert_eq!(packet[23], 17);
            let sequence = u64::from_be_bytes(packet[42..50].try_into().unwrap());
            assert!(sequence < 256);
            if sequence / 64 != cycle {
                continue;
            }
            received.insert(sequence);
            for i in 0..6 {
                packet.swap(i, i + 6);
            }
            for i in 0..4 {
                packet.swap(26 + i, 30 + i);
            }
            for i in 0..2 {
                packet.swap(34 + i, 36 + i);
            }
            socket
                .with_tx_batch(1, |batch| {
                    let mut frame = batch.next_frame()?.ok_or(Error::WouldBlock)?;
                    frame.bytes_mut()[..packet.len()].copy_from_slice(&packet);
                    frame.submit(packet.len())
                })
                .unwrap();
            {
                let mut pending = socket
                    .notify_async(
                        (sys::XSK_NOTIFY_FLAG_POKE_TX | sys::XSK_NOTIFY_FLAG_WAIT_TX) as u32,
                    )
                    .unwrap();
                assert!(pending.wait(Duration::from_secs(2)).unwrap().is_some());
            }
            assert_eq!(socket.drain_completions().unwrap(), 1);
        }
        assert_eq!(socket.stats().unwrap().TxInvalidDescriptors, 0);
        assert!(
            os_received,
            "unmatched UDP did not reach OS in cycle {cycle}"
        );
        steering.detach().unwrap();
    }
    println!("Modern eBPF: 256 UDP payloads echoed across four owned attach/detach cycles");
}
