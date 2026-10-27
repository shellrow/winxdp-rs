#[cfg(windows)]
fn main() {
    use std::{hint::black_box, time::Instant};
    use winxdp_sys::*;
    let mut memory = vec![0u64; 1040];
    let info = XSK_RING_INFO {
        Ring: memory.as_mut_ptr().cast(),
        DescriptorsOffset: 16,
        ProducerIndexOffset: 0,
        ConsumerIndexOffset: 4,
        FlagsOffset: 8,
        Size: 1024,
        ElementStride: 8,
        Reserved: 0,
    };
    let mut producer = XSK_RING::default();
    let mut consumer = XSK_RING::default();
    unsafe {
        XskRingInitialize(&mut producer, &info);
        XskRingInitialize(&mut consumer, &info);
    }
    for batch in [1, 16, 32, 64, 128] {
        let start = Instant::now();
        let iterations = 1_000_000;
        for _ in 0..iterations {
            let mut index = 0;
            unsafe {
                assert_eq!(
                    XskRingProducerReserve(&mut producer, batch, &mut index),
                    batch
                );
                black_box(XskRingGetElement(&producer, index));
                XskRingProducerSubmit(&mut producer, batch);
                assert_eq!(
                    XskRingConsumerReserve(&mut consumer, batch, &mut index),
                    batch
                );
                XskRingConsumerRelease(&mut consumer, batch);
            }
        }
        println!(
            "{{\"benchmark\":\"C_ring_helpers_only\",\"batch\":{batch},\"iterations\":{iterations},\"elapsed_ns\":{}}}",
            start.elapsed().as_nanos()
        );
    }
}
#[cfg(not(windows))]
fn main() {}
