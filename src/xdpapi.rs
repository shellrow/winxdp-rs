#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

use std::ptr::null_mut;

use windows::core::HRESULT;
use crate::bindings::xdpapi::{XdpOpenApi, XdpCloseApi};

// constants
pub use crate::bindings::xdpapi::XDP_API_VERSION_1;
pub use crate::bindings::xdpapi::XDP_QUIC_MAX_CID_LENGTH;

// Type alias
pub use crate::bindings::xdpapi::XDP_API_TABLE;
pub use crate::bindings::xdpapi::XDP_CREATE_PROGRAM_FLAGS;
pub use crate::bindings::xdpapi::XDP_EBPF_PARAMS;
pub use crate::bindings::xdpapi::OVERLAPPED;
pub use crate::bindings::xdpapi::LPOVERLAPPED;
pub use crate::bindings::xdpapi::XDP_MATCH_TYPE;
pub use crate::bindings::xdpapi::XDP_INET_ADDR;
pub use crate::bindings::xdpapi::XDP_IP_ADDRESS_MASK;
pub use crate::bindings::xdpapi::XDP_TUPLE;
pub use crate::bindings::xdpapi::XDP_QUIC_FLOW;
pub use crate::bindings::xdpapi::XDP_PORT_SET;
pub use crate::bindings::xdpapi::XDP_IP_PORT_SET;
pub use crate::bindings::xdpapi::XDP_RULE_ACTION;
pub use crate::bindings::xdpapi::XDP_MATCH_PATTERN;
pub use crate::bindings::xdpapi::XDP_REDIRECT_PARAMS;
pub use crate::bindings::xdpapi::XDP_LOAD_API_CONTEXT;
pub use crate::bindings::xdpapi::XDP_REDIRECT_TARGET_TYPE;

// Function pointers
pub use crate::bindings::xdpapi::XDP_OPEN_API_FN;
pub use crate::bindings::xdpapi::XDP_CLOSE_API_FN;
pub use crate::bindings::xdpapi::XDP_GET_ROUTINE_FN;
pub use crate::bindings::xdpapi::XDP_CREATE_PROGRAM_FN;
pub use crate::bindings::xdpapi::XDP_INTERFACE_OPEN_FN;

pub struct XdpSocket {
    xdp_api_table: *const XDP_API_TABLE,
}

impl XdpSocket {
    pub fn new() -> Self {
        unsafe {
            let mut xdp_api_table: *const XDP_API_TABLE = null_mut();
            let result = XdpOpenApi(1, &mut xdp_api_table);
            if result.is_ok() {
                println!("XdpOpenApi is ok");
            } else {
                println!("XdpOpenApi error: {:?}", result);
            }
            assert_eq!(result, HRESULT(0));
            XdpSocket { xdp_api_table }
        }
    }
    pub fn close(&self) {
        unsafe {
            XdpCloseApi(self.xdp_api_table);
            println!("XdpCloseApi is ok");
        }
    }
    pub fn get_routine(&self, routine_name: &str) -> *const std::ffi::c_void {
        unsafe {
            let routine_name = std::ffi::CString::new(routine_name).unwrap();
            let routine = (*self.xdp_api_table).XdpGetRoutine.unwrap();
            let result = routine(routine_name.as_ptr());
            if result.is_null() {
                println!("XdpGetRoutine error: {:?}", result);
            } else {
                println!("XdpGetRoutine is ok");
            }
            result
        }
    }
}
