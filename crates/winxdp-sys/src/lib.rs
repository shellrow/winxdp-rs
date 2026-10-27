//! Raw XDP v1.4.0 API v3 bindings. Every pointer-based call is unsafe.
//! Build requires the MSVC C++ toolchain and Windows SDK, but not installed XDP headers.
//! Experimental interfaces retain upstream semantics and stability limitations.
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
#![allow(clippy::all, unsafe_op_in_unsafe_fn)]
#[cfg(all(windows, not(target_arch = "x86_64")))]
compile_error!("The checked-in ABI is validated for Windows x64 only.");
#[cfg(windows)]
pub use windows_sys::Win32::{Foundation::HANDLE, System::IO::OVERLAPPED};
#[cfg(windows)]
pub use windows_sys::core::GUID;
#[cfg(windows)]
include!("bindings.rs");
/// User-mode XDP status is an HRESULT.
pub type XDP_STATUS = i32;
/// Equivalent to the public XDP_FAILED macro in user mode.
pub const fn XDP_FAILED(status: XDP_STATUS) -> bool {
    status < 0
}
/// Equivalent to the public XDP_SUCCEEDED macro in user mode.
pub const fn XDP_SUCCEEDED(status: XDP_STATUS) -> bool {
    status >= 0
}
