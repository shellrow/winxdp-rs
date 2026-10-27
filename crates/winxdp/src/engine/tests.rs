use super::*;
use std::panic::{AssertUnwindSafe, catch_unwind};
use windows_sys::Win32::System::Threading::CreateEventW;
struct Fixture {
    s: Socket,
    _memory: Vec<Vec<u64>>,
}
fn fixture() -> Fixture {
    let c = Config {
        interface: 1,
        receive: true,
        transmit: true,
        frame_size: 256,
        frame_count: 8,
        rx_ring_size: 4,
        tx_ring_size: 4,
        headroom: 16,
        ..Config::default()
    };
    let mut memory = vec![vec![0u64; 32]; 4];
    let mut ring = |i: usize, stride: u32| unsafe {
        Ring::new(
            sys::XSK_RING_INFO {
                Ring: memory[i].as_mut_ptr().cast(),
                DescriptorsOffset: 16,
                ProducerIndexOffset: 0,
                ConsumerIndexOffset: 4,
                FlagsOffset: 8,
                Size: 4,
                ElementStride: stride,
                Reserved: 0,
            },
            stride as usize,
        )
        .unwrap()
    };
    let fill = Some(ring(0, 8));
    let rx = Some(ring(1, 32));
    let tx = Some(ring(2, 32));
    let completion = Some(ring(3, 16));
    let handle = unsafe { CreateEventW(ptr::null(), 0, 0, ptr::null()) };
    assert!(!handle.is_null());
    let mut s = Socket {
        handle: Handle(handle),
        umem: Umem::new(2048).unwrap(),
        config: c,
        fill,
        rx,
        tx,
        completion,
        states: vec![State::FreeRx; 4]
            .into_iter()
            .chain(vec![State::FreeTx; 4])
            .collect(),
        rx_free: (0..4).rev().collect(),
        tx_free: (4..8).rev().collect(),
        poisoned: false,
        steering: true,
        extensions: Extensions::default(),
        _thread: PhantomData,
    };
    s.refill().unwrap();
    Fixture { s, _memory: memory }
}
fn inject(s: &mut Socket, bytes: &[u8]) {
    let fill = s.fill.as_mut().unwrap();
    let (p, n) = fill.reserve(false, 1).unwrap();
    assert_eq!(n, 1);
    let base: u64 = unsafe { fill.read(p) };
    fill.advance(false, 1);
    unsafe {
        ptr::copy_nonoverlapping(
            bytes.as_ptr(),
            s.umem.ptr.add(base as usize + 16),
            bytes.len(),
        );
    }
    let rx = s.rx.as_mut().unwrap();
    let (p, n) = rx.reserve(true, 1).unwrap();
    assert_eq!(n, 1);
    unsafe {
        rx.write(
            p,
            sys::XSK_BUFFER_DESCRIPTOR {
                Address: sys::XSK_BUFFER_ADDRESS {
                    AddressAndOffset: base | (16u64 << 48),
                },
                Length: bytes.len() as u32,
                Reserved: 0,
            },
        );
    }
    rx.advance(true, 1);
}
fn complete(s: &mut Socket) {
    let tx = s.tx.as_mut().unwrap();
    let (p, n) = tx.reserve(false, 4).unwrap();
    let comp = s.completion.as_mut().unwrap();
    let (cp, cn) = comp.reserve(true, n).unwrap();
    assert_eq!(cn, n);
    for i in 0..n {
        let d: sys::XSK_BUFFER_DESCRIPTOR = unsafe { tx.read(p.wrapping_add(i)) };
        let address = unsafe { d.Address.AddressAndOffset } & ((1u64 << 48) - 1);
        unsafe {
            comp.write(cp.wrapping_add(i), address);
        }
    }
    tx.advance(false, n);
    comp.advance(true, n);
}
#[test]
fn tx_pressure_and_completion_recycling() {
    let mut f = fixture();
    for _ in 0..4 {
        assert_eq!(f.s.send(&[1; 60]).unwrap(), 60);
    }
    assert_eq!(f.s.send(&[1; 60]), Err(Error::WouldBlock));
    complete(&mut f.s);
    assert_eq!(f.s.drain_completions().unwrap(), 4);
    assert_eq!(f.s.tx_free.len(), 4);
    assert!(f.s.send(&[1; 60]).is_ok());
}
#[test]
fn rx_copy_and_continuous_refill() {
    let mut f = fixture();
    for i in 0..1000 {
        let bytes = [i as u8; 60];
        inject(&mut f.s, &bytes);
        let mut out = [0; 256];
        let info = f.s.recv_into(&mut out).unwrap();
        assert_eq!(info.length, 60);
        assert_eq!(&out[..60], &bytes);
    }
    assert_eq!(f.s.fill.as_ref().unwrap().reserve(false, 9).unwrap().1, 4);
}
#[test]
fn rx_short_buffer_and_panic_release() {
    let mut f = fixture();
    inject(&mut f.s, &[1; 80]);
    assert_eq!(
        f.s.recv_into(&mut [0; 16]).unwrap_err(),
        Error::BufferTooSmall {
            needed: 80,
            provided: 16
        }
    );
    inject(&mut f.s, &[2; 80]);
    assert!(
        catch_unwind(AssertUnwindSafe(|| {
            let _: Result<()> = f.s.with_rx_batch(4, |b| {
                assert_eq!(b.len(), 1);
                panic!("test panic")
            });
        }))
        .is_err()
    );
    assert_eq!(f.s.fill.as_ref().unwrap().reserve(false, 9).unwrap().1, 4);
    assert!(!f.s.states.contains(&State::BorrowedRx));
}
#[test]
fn tx_dropped_and_partial_panic_batches() {
    let mut f = fixture();
    assert!(
        catch_unwind(AssertUnwindSafe(|| {
            let _: Result<()> = f.s.with_tx_batch(4, |b| {
                let frame = b.next_frame()?.unwrap();
                frame.submit(60)?;
                let _dropped = b.next_frame()?.unwrap();
                panic!("test panic")
            });
        }))
        .is_err()
    );
    assert_eq!(f.s.tx_free.len(), 3);
    assert_eq!(f.s.tx.as_ref().unwrap().reserve(false, 4).unwrap().1, 1);
    complete(&mut f.s);
    f.s.drain_completions().unwrap();
    assert_eq!(f.s.tx_free.len(), 4);
}
#[test]
fn duplicate_rx_quarantines_socket() {
    let mut f = fixture();
    inject(&mut f.s, &[1; 60]);
    let r = f.s.rx.as_mut().unwrap();
    let d: sys::XSK_BUFFER_DESCRIPTOR = unsafe { r.read(0) };
    unsafe {
        r.write(1, d);
    }
    r.advance(true, 1);
    assert!(matches!(
        f.s.with_rx_batch(4, |_| Ok(())),
        Err(Error::CorruptDescriptor(_))
    ));
    assert!(f.s.poisoned);
    assert!(f.s.send(&[0; 60]).is_err());
}
#[test]
fn invalid_completion_and_callback_panic() {
    let mut f = fixture();
    f.s.send(&[1; 60]).unwrap();
    complete(&mut f.s);
    assert!(
        catch_unwind(AssertUnwindSafe(|| {
            f.s.drain_completions_with(|_| panic!("observer")).unwrap();
        }))
        .is_err()
    );
    assert_eq!(f.s.drain_completions().unwrap(), 0);
    assert_eq!(f.s.tx_free.len(), 4);
    let r = f.s.completion.as_mut().unwrap();
    unsafe {
        r.write(1, 1024u64);
    }
    r.advance(true, 1);
    assert!(matches!(
        f.s.drain_completions(),
        Err(Error::CorruptDescriptor(_))
    ));
}
#[test]
fn metadata_and_completion_timestamp() {
    let mut f = fixture();
    inject(&mut f.s, &[3; 60]);
    f.s.extensions.original_length = Some(16);
    f.s.extensions.rx_timestamp = Some(24);
    unsafe {
        let p = f.s.rx.as_ref().unwrap().element(0);
        ptr::write_unaligned(p.add(16).cast::<u32>(), 80);
        ptr::write_unaligned(p.add(24).cast::<u64>(), 123);
    }
    f.s.with_rx_batch(1, |b| {
        let m = b.frame(0).unwrap().metadata();
        assert_eq!(m.original_length, Some(80));
        assert_eq!(m.timestamp, Some(123));
        Ok(())
    })
    .unwrap();
    f.s.send(&[1; 60]).unwrap();
    complete(&mut f.s);
    f.s.extensions.tx_timestamp = Some(8);
    unsafe {
        ptr::write_unaligned(
            f.s.completion
                .as_ref()
                .unwrap()
                .element(0)
                .add(8)
                .cast::<u64>(),
            456,
        );
    }
    f.s.drain_completions_with(|c| assert_eq!(c.timestamp, Some(456)))
        .unwrap();
}
#[test]
fn malformed_descriptor_property() {
    let c = Config {
        interface: 1,
        frame_size: 256,
        frame_count: 8,
        ..Config::default()
    };
    let mut rng = 0x12345678u64;
    for _ in 0..100000 {
        rng = rng.wrapping_mul(6364136223846793005).wrapping_add(1);
        let address = rng;
        let d = sys::XSK_BUFFER_DESCRIPTOR {
            Address: sys::XSK_BUFFER_ADDRESS {
                AddressAndOffset: address,
            },
            Length: (rng >> 32) as u32,
            Reserved: 0,
        };
        if let Ok((id, start, len)) = validate_descriptor(&c, &d) {
            assert!(id < 8);
            assert!(start + len <= 2048);
            assert!(start + len <= ((id + 1) * 256));
        }
    }
    for id in 0..8 {
        for off in [0, 1, 16, 255] {
            for len in [0, 1, 60, 256, u32::MAX] {
                let d = sys::XSK_BUFFER_DESCRIPTOR {
                    Address: sys::XSK_BUFFER_ADDRESS {
                        AddressAndOffset: (id * 256) | ((off as u64) << 48),
                    },
                    Length: len,
                    Reserved: 0,
                };
                assert_eq!(
                    validate_descriptor(&c, &d).is_ok(),
                    off as u64 + len as u64 <= 256
                );
            }
        }
    }
}
#[test]
fn errors_and_steering_are_explicit() {
    assert!(matches!(
        status(0x80070002u32 as i32, "XskCreate"),
        Err(Error::DriverNotInstalled { .. })
    ));
    assert!(matches!(
        status(0x80070002u32 as i32, "XskBind"),
        Err(Error::Io { .. })
    ));
    let mut f = fixture();
    f.s.steering = false;
    assert_eq!(
        f.s.recv_into(&mut [0; 64]).unwrap_err(),
        Error::SteeringNotConfigured
    );
}
#[test]
fn offload_metadata_rejection_returns_reservation() {
    let mut f = fixture();
    f.s.extensions.tx_layout = Some(16);
    f.s.extensions.tx_checksum = Some(21);
    let error = f.s.with_tx_batch(1, |b| {
        let mut frame = b.next_frame()?.unwrap();
        let mut layout = sys::XDP_FRAME_LAYOUT::default();
        layout.set_Layer3HeaderLength(511);
        frame.checksum(layout, sys::XDP_FRAME_CHECKSUM::default())?;
        frame.submit(60)
    });
    assert!(matches!(error, Err(Error::InvalidConfiguration(_))));
    assert_eq!(f.s.tx_free.len(), 4);
    assert_eq!(f.s.tx.as_ref().unwrap().reserve(false, 4).unwrap().1, 0);
}
