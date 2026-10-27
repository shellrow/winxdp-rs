#![cfg(windows)]
use winxdp_sys::*;
unsafe extern "C" {
    fn wx_test_address(base: u64, offset: u16) -> u64;
    fn wx_test_layout(l2: u16, l3: u16, l4: u8, t2: u8, t3: u8, t4: u8) -> u64;
}
#[test]
fn bitfields_match_c() {
    for base in [0, 1, 0x123456789abc, (1 << 48) - 1] {
        for offset in [0, 1, 32768, 65535] {
            assert_eq!(
                unsafe { wx_test_address(base, offset) },
                base | ((offset as u64) << 48)
            );
        }
    }
    let mut l = XDP_FRAME_LAYOUT::default();
    l.set_Layer2HeaderLength(127);
    l.set_Layer3HeaderLength(511);
    l.set_Layer4HeaderLength(255);
    l.set_Layer2Type(2);
    l.set_Layer3Type(6);
    l.set_Layer4Type(4);
    let mut bytes = [0u8; 8];
    unsafe {
        std::ptr::copy_nonoverlapping(
            (&l as *const XDP_FRAME_LAYOUT).cast::<u8>(),
            bytes.as_mut_ptr(),
            5,
        );
    }
    assert_eq!(u64::from_le_bytes(bytes), unsafe {
        wx_test_layout(127, 511, 255, 2, 6, 4)
    });
}
