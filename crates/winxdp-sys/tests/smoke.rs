#![cfg(windows)]
use winxdp_sys::*;
#[test]
fn invalid_handles_preserve_failure() {
    let invalid = windows_sys::Win32::Foundation::INVALID_HANDLE_VALUE;
    let mut length = 4;
    let mut value = 0u32;
    unsafe {
        assert!(XskBind(invalid, 1, 0, XSK_BIND_FLAG_TX) < 0);
        assert!(XskActivate(invalid, 0) < 0);
        assert!(
            XskGetSockopt(
                invalid,
                XSK_SOCKOPT_TX_ERROR,
                (&mut value as *mut u32).cast(),
                &mut length
            ) < 0
        );
    }
}
#[test]
fn status_semantics() {
    assert!(XDP_FAILED(XDP_STATUS_PENDING));
    assert!(XDP_FAILED(XDP_STATUS_TIMEOUT));
    assert!(XDP_SUCCEEDED(XDP_STATUS_SUCCESS));
}
