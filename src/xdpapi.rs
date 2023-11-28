#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

use windows::core::HRESULT;
use crate::bindings::xdpapi::{UINT32, XDP_API_TABLE};

#[link(name = "xdpapi")]
extern "C" {
    pub fn XdpOpenApi(XdpApiVersion: UINT32, XdpApiTable: *mut *const XDP_API_TABLE) -> HRESULT;
    pub fn XdpCloseApi(XdpApiTable: *const XDP_API_TABLE);
}

// test for link with xdpapi.dll
#[cfg(test)]
mod tests {
    use super::*;
    //use std::ffi::CString;
    use std::ptr::null_mut;

    #[test]
    fn test_xdp_open_api() {
        unsafe {
            let mut xdp_api_table: *const XDP_API_TABLE = null_mut();

            let result = XdpOpenApi(1, &mut xdp_api_table);
            if result.is_ok() {
                println!("XdpOpenApi is ok");
                // Close the XDP API
                XdpCloseApi(xdp_api_table);
                println!("XdpCloseApi is ok");
            } else {
                println!("XdpOpenApi error: {:?}", result);
            }
            assert_eq!(result, HRESULT(0));
        }
    }
}
