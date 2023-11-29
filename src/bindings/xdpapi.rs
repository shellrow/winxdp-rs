#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

use windows::Win32::Foundation::HANDLE;
use windows::Win32::Networking::WinSock::IN6_ADDR;
use windows::Win32::Networking::WinSock::IN_ADDR;
use windows::core::HRESULT;


pub type wchar_t = ::std::os::raw::c_ushort;
pub type PVOID = *mut ::std::os::raw::c_void;
pub type CHAR = ::std::os::raw::c_char;
pub type SHORT = ::std::os::raw::c_short;
pub type LONG = ::std::os::raw::c_long;
pub type WCHAR = wchar_t;
pub type PWCHAR = *mut WCHAR;
pub type LPWCH = *mut WCHAR;
pub type PWCH = *mut WCHAR;
pub type LPCWCH = *const WCHAR;
pub type PCWCH = *const WCHAR;
pub type NWPSTR = *mut WCHAR;
pub type LPWSTR = *mut WCHAR;
pub type PWSTR = *mut WCHAR;
pub type PZPWSTR = *mut PWSTR;
pub type PCZPWSTR = *const PWSTR;
pub type LPUWSTR = *mut WCHAR;
pub type PUWSTR = *mut WCHAR;
pub type LPCWSTR = *const WCHAR;
pub type PCWSTR = *const WCHAR;
pub type PZPCWSTR = *mut PCWSTR;
pub type PCZPCWSTR = *const PCWSTR;
pub type LPCUWSTR = *const WCHAR;
pub type PCUWSTR = *const WCHAR;
pub type PZZWSTR = *mut WCHAR;
pub type PCZZWSTR = *const WCHAR;
pub type PUZZWSTR = *mut WCHAR;
pub type PCUZZWSTR = *const WCHAR;
pub type PNZWCH = *mut WCHAR;
pub type PCNZWCH = *const WCHAR;
pub type PUNZWCH = *mut WCHAR;
pub type PCUNZWCH = *const WCHAR;
pub type LPCWCHAR = *const WCHAR;
pub type PCWCHAR = *const WCHAR;
pub type LPCUWCHAR = *const WCHAR;
pub type PCUWCHAR = *const WCHAR;
pub type UCSCHAR = ::std::os::raw::c_ulong;
pub type PUCSCHAR = *mut UCSCHAR;
pub type PCUCSCHAR = *const UCSCHAR;
pub type PUCSSTR = *mut UCSCHAR;
pub type PUUCSSTR = *mut UCSCHAR;
pub type PCUCSSTR = *const UCSCHAR;
pub type PCUUCSSTR = *const UCSCHAR;
pub type PUUCSCHAR = *mut UCSCHAR;
pub type PCUUCSCHAR = *const UCSCHAR;
pub type PCHAR = *mut CHAR;
pub type LPCH = *mut CHAR;
pub type PCH = *mut CHAR;
pub type LPCCH = *const CHAR;
pub type PCCH = *const CHAR;
pub type NPSTR = *mut CHAR;
pub type LPSTR = *mut CHAR;
pub type PSTR = *mut CHAR;
pub type PZPSTR = *mut PSTR;
pub type PCZPSTR = *const PSTR;
pub type LPCSTR = *const CHAR;
pub type PCSTR = *const CHAR;
pub type PZPCSTR = *mut PCSTR;
pub type PCZPCSTR = *const PCSTR;
pub type PZZSTR = *mut CHAR;
pub type PCZZSTR = *const CHAR;
pub type PNZCH = *mut CHAR;
pub type PCNZCH = *const CHAR;
pub type TCHAR = ::std::os::raw::c_char;
pub type PTCHAR = *mut ::std::os::raw::c_char;
pub type TBYTE = ::std::os::raw::c_uchar;
pub type PTBYTE = *mut ::std::os::raw::c_uchar;
pub type LPTCH = LPCH;
pub type PTCH = LPCH;
pub type LPCTCH = LPCCH;
pub type PCTCH = LPCCH;
pub type PTSTR = LPSTR;
pub type LPTSTR = LPSTR;
pub type PUTSTR = LPSTR;
pub type LPUTSTR = LPSTR;
pub type PCTSTR = LPCSTR;
pub type LPCTSTR = LPCSTR;
pub type PCUTSTR = LPCSTR;
pub type LPCUTSTR = LPCSTR;
pub type PZZTSTR = PZZSTR;
pub type PUZZTSTR = PZZSTR;
pub type PCZZTSTR = PCZZSTR;
pub type PCUZZTSTR = PCZZSTR;
pub type PZPTSTR = PZPSTR;
pub type PNZTCH = PNZCH;
pub type PUNZTCH = PNZCH;
pub type PCNZTCH = PCNZCH;
pub type PCUNZTCH = PCNZCH;
pub type PSHORT = *mut SHORT;
pub type PLONG = *mut LONG;
pub type ULONG = ::std::os::raw::c_ulong;
pub type PULONG = *mut ULONG;
pub type USHORT = ::std::os::raw::c_ushort;
pub type PUSHORT = *mut USHORT;
pub type UCHAR = ::std::os::raw::c_uchar;
pub type PUCHAR = *mut UCHAR;
pub type PSZ = *mut ::std::os::raw::c_char;
pub type DWORD = ::std::os::raw::c_ulong;
pub type BOOL = ::std::os::raw::c_int;
pub type BYTE = ::std::os::raw::c_uchar;
pub type WORD = ::std::os::raw::c_ushort;
pub type FLOAT = f32;
pub type PFLOAT = *mut FLOAT;
pub type PBOOL = *mut BOOL;
pub type LPBOOL = *mut BOOL;
pub type PBYTE = *mut BYTE;
pub type LPBYTE = *mut BYTE;
pub type PINT = *mut ::std::os::raw::c_int;
pub type LPINT = *mut ::std::os::raw::c_int;
pub type PWORD = *mut WORD;
pub type LPWORD = *mut WORD;
pub type LPLONG = *mut ::std::os::raw::c_long;
pub type PDWORD = *mut DWORD;
pub type LPDWORD = *mut DWORD;
pub type LPVOID = *mut ::std::os::raw::c_void;
pub type LPCVOID = *const ::std::os::raw::c_void;
pub type INT = ::std::os::raw::c_int;
pub type UINT = ::std::os::raw::c_uint;
pub type PUINT = *mut ::std::os::raw::c_uint;

pub const XDP_QUIC_MAX_CID_LENGTH: u32 = 20;
pub const XSK_SOCKOPT_UMEM_REG: u32 = 1;
pub const XSK_SOCKOPT_RX_RING_SIZE: u32 = 2;
pub const XSK_SOCKOPT_RX_FILL_RING_SIZE: u32 = 3;
pub const XSK_SOCKOPT_TX_RING_SIZE: u32 = 4;
pub const XSK_SOCKOPT_TX_COMPLETION_RING_SIZE: u32 = 5;
pub const XSK_SOCKOPT_RING_INFO: u32 = 6;
pub const XSK_SOCKOPT_STATISTICS: u32 = 7;
pub const XSK_SOCKOPT_RX_HOOK_ID: u32 = 8;
pub const XSK_SOCKOPT_TX_HOOK_ID: u32 = 9;
pub const XSK_SOCKOPT_RX_ERROR: u32 = 10;
pub const XSK_SOCKOPT_RX_FILL_ERROR: u32 = 11;
pub const XSK_SOCKOPT_TX_ERROR: u32 = 12;
pub const XSK_SOCKOPT_TX_COMPLETION_ERROR: u32 = 13;
pub const XSK_SOCKOPT_RX_PROCESSOR_AFFINITY: u32 = 14;
pub const XSK_SOCKOPT_TX_PROCESSOR_AFFINITY: u32 = 15;
pub const XDP_API_VERSION_1: u32 = 1;
pub const _XDP_HOOK_LAYER_XDP_HOOK_L2: _XDP_HOOK_LAYER = 0;
pub type _XDP_HOOK_LAYER = ::std::os::raw::c_int;
pub use self::_XDP_HOOK_LAYER as XDP_HOOK_LAYER;
pub const _XDP_HOOK_DATAPATH_DIRECTION_XDP_HOOK_RX: _XDP_HOOK_DATAPATH_DIRECTION = 0;
pub const _XDP_HOOK_DATAPATH_DIRECTION_XDP_HOOK_TX: _XDP_HOOK_DATAPATH_DIRECTION = 1;
pub type _XDP_HOOK_DATAPATH_DIRECTION = ::std::os::raw::c_int;
pub use self::_XDP_HOOK_DATAPATH_DIRECTION as XDP_HOOK_DATAPATH_DIRECTION;
pub const _XDP_HOOK_SUBLAYER_XDP_HOOK_INSPECT: _XDP_HOOK_SUBLAYER = 0;
pub const _XDP_HOOK_SUBLAYER_XDP_HOOK_INJECT: _XDP_HOOK_SUBLAYER = 1;
pub type _XDP_HOOK_SUBLAYER = ::std::os::raw::c_int;
pub use self::_XDP_HOOK_SUBLAYER as XDP_HOOK_SUBLAYER;

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct __BindgenBitfieldUnit<Storage> {
    storage: Storage,
}
impl<Storage> __BindgenBitfieldUnit<Storage> {
    #[inline]
    pub const fn new(storage: Storage) -> Self {
        Self { storage }
    }
}
impl<Storage> __BindgenBitfieldUnit<Storage>
where
    Storage: AsRef<[u8]> + AsMut<[u8]>,
{
    #[inline]
    pub fn get_bit(&self, index: usize) -> bool {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = self.storage.as_ref()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        byte & mask == mask
    }
    #[inline]
    pub fn set_bit(&mut self, index: usize, val: bool) {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = &mut self.storage.as_mut()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        if val {
            *byte |= mask;
        } else {
            *byte &= !mask;
        }
    }
    #[inline]
    pub fn get(&self, bit_offset: usize, bit_width: u8) -> u64 {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        let mut val = 0;
        for i in 0..(bit_width as usize) {
            if self.get_bit(i + bit_offset) {
                let index = if cfg!(target_endian = "big") {
                    bit_width as usize - 1 - i
                } else {
                    i
                };
                val |= 1 << index;
            }
        }
        val
    }
    #[inline]
    pub fn set(&mut self, bit_offset: usize, bit_width: u8, val: u64) {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        for i in 0..(bit_width as usize) {
            let mask = 1 << i;
            let val_bit_is_set = val & mask == mask;
            let index = if cfg!(target_endian = "big") {
                bit_width as usize - 1 - i
            } else {
                i
            };
            self.set_bit(index + bit_offset, val_bit_is_set);
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _OVERLAPPED {
    pub Internal: ULONG_PTR,
    pub InternalHigh: ULONG_PTR,
    pub __bindgen_anon_1: _OVERLAPPED__bindgen_ty_1,
    pub hEvent: HANDLE,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union _OVERLAPPED__bindgen_ty_1 {
    pub __bindgen_anon_1: _OVERLAPPED__bindgen_ty_1__bindgen_ty_1,
    pub Pointer: PVOID,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _OVERLAPPED__bindgen_ty_1__bindgen_ty_1 {
    pub Offset: DWORD,
    pub OffsetHigh: DWORD,
}
#[test]
fn bindgen_test_layout__OVERLAPPED__bindgen_ty_1__bindgen_ty_1() {
    const UNINIT: ::std::mem::MaybeUninit<_OVERLAPPED__bindgen_ty_1__bindgen_ty_1> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_OVERLAPPED__bindgen_ty_1__bindgen_ty_1>(),
        8usize,
        concat!(
            "Size of: ",
            stringify!(_OVERLAPPED__bindgen_ty_1__bindgen_ty_1)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<_OVERLAPPED__bindgen_ty_1__bindgen_ty_1>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(_OVERLAPPED__bindgen_ty_1__bindgen_ty_1)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Offset) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_OVERLAPPED__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(Offset)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).OffsetHigh) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(_OVERLAPPED__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(OffsetHigh)
        )
    );
}
#[test]
fn bindgen_test_layout__OVERLAPPED__bindgen_ty_1() {
    const UNINIT: ::std::mem::MaybeUninit<_OVERLAPPED__bindgen_ty_1> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_OVERLAPPED__bindgen_ty_1>(),
        8usize,
        concat!("Size of: ", stringify!(_OVERLAPPED__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<_OVERLAPPED__bindgen_ty_1>(),
        8usize,
        concat!("Alignment of ", stringify!(_OVERLAPPED__bindgen_ty_1))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Pointer) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_OVERLAPPED__bindgen_ty_1),
            "::",
            stringify!(Pointer)
        )
    );
}
#[test]
fn bindgen_test_layout__OVERLAPPED() {
    const UNINIT: ::std::mem::MaybeUninit<_OVERLAPPED> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_OVERLAPPED>(),
        32usize,
        concat!("Size of: ", stringify!(_OVERLAPPED))
    );
    assert_eq!(
        ::std::mem::align_of::<_OVERLAPPED>(),
        8usize,
        concat!("Alignment of ", stringify!(_OVERLAPPED))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Internal) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_OVERLAPPED),
            "::",
            stringify!(Internal)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).InternalHigh) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_OVERLAPPED),
            "::",
            stringify!(InternalHigh)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).hEvent) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(_OVERLAPPED),
            "::",
            stringify!(hEvent)
        )
    );
}
pub type OVERLAPPED = _OVERLAPPED;
pub type LPOVERLAPPED = *mut _OVERLAPPED;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _XDP_HOOK_ID {
    pub Layer: XDP_HOOK_LAYER,
    pub Direction: XDP_HOOK_DATAPATH_DIRECTION,
    pub SubLayer: XDP_HOOK_SUBLAYER,
}
#[test]
fn bindgen_test_layout__XDP_HOOK_ID() {
    const UNINIT: ::std::mem::MaybeUninit<_XDP_HOOK_ID> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_XDP_HOOK_ID>(),
        12usize,
        concat!("Size of: ", stringify!(_XDP_HOOK_ID))
    );
    assert_eq!(
        ::std::mem::align_of::<_XDP_HOOK_ID>(),
        4usize,
        concat!("Alignment of ", stringify!(_XDP_HOOK_ID))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Layer) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_XDP_HOOK_ID),
            "::",
            stringify!(Layer)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Direction) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(_XDP_HOOK_ID),
            "::",
            stringify!(Direction)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).SubLayer) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_XDP_HOOK_ID),
            "::",
            stringify!(SubLayer)
        )
    );
}
pub type XDP_HOOK_ID = _XDP_HOOK_ID;
pub type POINTER_64_INT = ::std::os::raw::c_ulonglong;
pub type INT8 = ::std::os::raw::c_schar;
pub type PINT8 = *mut ::std::os::raw::c_schar;
pub type INT16 = ::std::os::raw::c_short;
pub type PINT16 = *mut ::std::os::raw::c_short;
pub type INT32 = ::std::os::raw::c_int;
pub type PINT32 = *mut ::std::os::raw::c_int;
pub type INT64 = ::std::os::raw::c_longlong;
pub type PINT64 = *mut ::std::os::raw::c_longlong;
pub type UINT8 = ::std::os::raw::c_uchar;
pub type PUINT8 = *mut ::std::os::raw::c_uchar;
pub type UINT16 = ::std::os::raw::c_ushort;
pub type PUINT16 = *mut ::std::os::raw::c_ushort;
pub type UINT32 = ::std::os::raw::c_uint;
pub type PUINT32 = *mut ::std::os::raw::c_uint;
pub type UINT64 = ::std::os::raw::c_ulonglong;
pub type PUINT64 = *mut ::std::os::raw::c_ulonglong;
pub type LONG32 = ::std::os::raw::c_int;
pub type PLONG32 = *mut ::std::os::raw::c_int;
pub type ULONG32 = ::std::os::raw::c_uint;
pub type PULONG32 = *mut ::std::os::raw::c_uint;
pub type DWORD32 = ::std::os::raw::c_uint;
pub type PDWORD32 = *mut ::std::os::raw::c_uint;
pub type INT_PTR = ::std::os::raw::c_longlong;
pub type PINT_PTR = *mut ::std::os::raw::c_longlong;
pub type UINT_PTR = ::std::os::raw::c_ulonglong;
pub type PUINT_PTR = *mut ::std::os::raw::c_ulonglong;
pub type LONG_PTR = ::std::os::raw::c_longlong;
pub type PLONG_PTR = *mut ::std::os::raw::c_longlong;
pub type ULONG_PTR = ::std::os::raw::c_ulonglong;
pub type PULONG_PTR = *mut ::std::os::raw::c_ulonglong;
pub type PHANDLE64 = *mut *mut ::std::os::raw::c_void;
pub type SHANDLE_PTR = ::std::os::raw::c_longlong;
pub type HANDLE_PTR = ::std::os::raw::c_ulonglong;
pub type UHALF_PTR = ::std::os::raw::c_uint;
pub type PUHALF_PTR = *mut ::std::os::raw::c_uint;
pub type HALF_PTR = ::std::os::raw::c_int;
pub type PHALF_PTR = *mut ::std::os::raw::c_int;
pub type SIZE_T = ULONG_PTR;
pub type PSIZE_T = *mut ULONG_PTR;
pub type SSIZE_T = LONG_PTR;
pub type PSSIZE_T = *mut LONG_PTR;
pub type DWORD_PTR = ULONG_PTR;
pub type PDWORD_PTR = *mut ULONG_PTR;
pub type LONG64 = ::std::os::raw::c_longlong;
pub type PLONG64 = *mut ::std::os::raw::c_longlong;
pub type ULONG64 = ::std::os::raw::c_ulonglong;
pub type PULONG64 = *mut ::std::os::raw::c_ulonglong;
pub type DWORD64 = ::std::os::raw::c_ulonglong;
pub type PDWORD64 = *mut ::std::os::raw::c_ulonglong;
pub type KAFFINITY = ULONG_PTR;
pub type PKAFFINITY = *mut KAFFINITY;

pub const _XDP_MATCH_TYPE_XDP_MATCH_ALL: _XDP_MATCH_TYPE = 0;
pub const _XDP_MATCH_TYPE_XDP_MATCH_UDP: _XDP_MATCH_TYPE = 1;
pub const _XDP_MATCH_TYPE_XDP_MATCH_UDP_DST: _XDP_MATCH_TYPE = 2;
pub const _XDP_MATCH_TYPE_XDP_MATCH_IPV4_DST_MASK: _XDP_MATCH_TYPE = 3;
pub const _XDP_MATCH_TYPE_XDP_MATCH_IPV6_DST_MASK: _XDP_MATCH_TYPE = 4;
pub const _XDP_MATCH_TYPE_XDP_MATCH_QUIC_FLOW_SRC_CID: _XDP_MATCH_TYPE = 5;
pub const _XDP_MATCH_TYPE_XDP_MATCH_QUIC_FLOW_DST_CID: _XDP_MATCH_TYPE = 6;
pub const _XDP_MATCH_TYPE_XDP_MATCH_IPV4_UDP_TUPLE: _XDP_MATCH_TYPE = 7;
pub const _XDP_MATCH_TYPE_XDP_MATCH_IPV6_UDP_TUPLE: _XDP_MATCH_TYPE = 8;
pub const _XDP_MATCH_TYPE_XDP_MATCH_UDP_PORT_SET: _XDP_MATCH_TYPE = 9;
pub const _XDP_MATCH_TYPE_XDP_MATCH_IPV4_UDP_PORT_SET: _XDP_MATCH_TYPE = 10;
pub const _XDP_MATCH_TYPE_XDP_MATCH_IPV6_UDP_PORT_SET: _XDP_MATCH_TYPE = 11;
pub const _XDP_MATCH_TYPE_XDP_MATCH_IPV4_TCP_PORT_SET: _XDP_MATCH_TYPE = 12;
pub const _XDP_MATCH_TYPE_XDP_MATCH_IPV6_TCP_PORT_SET: _XDP_MATCH_TYPE = 13;
pub const _XDP_MATCH_TYPE_XDP_MATCH_TCP_DST: _XDP_MATCH_TYPE = 14;
pub const _XDP_MATCH_TYPE_XDP_MATCH_TCP_QUIC_FLOW_SRC_CID: _XDP_MATCH_TYPE = 15;
pub const _XDP_MATCH_TYPE_XDP_MATCH_TCP_QUIC_FLOW_DST_CID: _XDP_MATCH_TYPE = 16;
pub const _XDP_MATCH_TYPE_XDP_MATCH_TCP_CONTROL_DST: _XDP_MATCH_TYPE = 17;
pub type _XDP_MATCH_TYPE = ::std::os::raw::c_int;

pub use self::_XDP_MATCH_TYPE as XDP_MATCH_TYPE;
#[repr(C)]
#[derive(Copy, Clone)]
pub union _XDP_INET_ADDR {
    pub Ipv4: IN_ADDR,
    pub Ipv6: IN6_ADDR,
}
#[test]
fn bindgen_test_layout__XDP_INET_ADDR() {
    const UNINIT: ::std::mem::MaybeUninit<_XDP_INET_ADDR> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_XDP_INET_ADDR>(),
        16usize,
        concat!("Size of: ", stringify!(_XDP_INET_ADDR))
    );
    assert_eq!(
        ::std::mem::align_of::<_XDP_INET_ADDR>(),
        4usize,
        concat!("Alignment of ", stringify!(_XDP_INET_ADDR))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Ipv4) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_XDP_INET_ADDR),
            "::",
            stringify!(Ipv4)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Ipv6) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_XDP_INET_ADDR),
            "::",
            stringify!(Ipv6)
        )
    );
}
pub type XDP_INET_ADDR = _XDP_INET_ADDR;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _XDP_IP_ADDRESS_MASK {
    pub Mask: XDP_INET_ADDR,
    pub Address: XDP_INET_ADDR,
}
#[test]
fn bindgen_test_layout__XDP_IP_ADDRESS_MASK() {
    const UNINIT: ::std::mem::MaybeUninit<_XDP_IP_ADDRESS_MASK> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_XDP_IP_ADDRESS_MASK>(),
        32usize,
        concat!("Size of: ", stringify!(_XDP_IP_ADDRESS_MASK))
    );
    assert_eq!(
        ::std::mem::align_of::<_XDP_IP_ADDRESS_MASK>(),
        4usize,
        concat!("Alignment of ", stringify!(_XDP_IP_ADDRESS_MASK))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Mask) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_XDP_IP_ADDRESS_MASK),
            "::",
            stringify!(Mask)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Address) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_XDP_IP_ADDRESS_MASK),
            "::",
            stringify!(Address)
        )
    );
}
pub type XDP_IP_ADDRESS_MASK = _XDP_IP_ADDRESS_MASK;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _XDP_TUPLE {
    pub SourceAddress: XDP_INET_ADDR,
    pub DestinationAddress: XDP_INET_ADDR,
    pub SourcePort: UINT16,
    pub DestinationPort: UINT16,
}
#[test]
fn bindgen_test_layout__XDP_TUPLE() {
    const UNINIT: ::std::mem::MaybeUninit<_XDP_TUPLE> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_XDP_TUPLE>(),
        36usize,
        concat!("Size of: ", stringify!(_XDP_TUPLE))
    );
    assert_eq!(
        ::std::mem::align_of::<_XDP_TUPLE>(),
        4usize,
        concat!("Alignment of ", stringify!(_XDP_TUPLE))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).SourceAddress) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_XDP_TUPLE),
            "::",
            stringify!(SourceAddress)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).DestinationAddress) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_XDP_TUPLE),
            "::",
            stringify!(DestinationAddress)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).SourcePort) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(_XDP_TUPLE),
            "::",
            stringify!(SourcePort)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).DestinationPort) as usize - ptr as usize },
        34usize,
        concat!(
            "Offset of field: ",
            stringify!(_XDP_TUPLE),
            "::",
            stringify!(DestinationPort)
        )
    );
}
pub type XDP_TUPLE = _XDP_TUPLE;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _XDP_QUIC_FLOW {
    pub UdpPort: UINT16,
    pub CidLength: UCHAR,
    pub CidOffset: UCHAR,
    pub CidData: [UCHAR; 20usize],
}
#[test]
fn bindgen_test_layout__XDP_QUIC_FLOW() {
    const UNINIT: ::std::mem::MaybeUninit<_XDP_QUIC_FLOW> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_XDP_QUIC_FLOW>(),
        24usize,
        concat!("Size of: ", stringify!(_XDP_QUIC_FLOW))
    );
    assert_eq!(
        ::std::mem::align_of::<_XDP_QUIC_FLOW>(),
        2usize,
        concat!("Alignment of ", stringify!(_XDP_QUIC_FLOW))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).UdpPort) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_XDP_QUIC_FLOW),
            "::",
            stringify!(UdpPort)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).CidLength) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(_XDP_QUIC_FLOW),
            "::",
            stringify!(CidLength)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).CidOffset) as usize - ptr as usize },
        3usize,
        concat!(
            "Offset of field: ",
            stringify!(_XDP_QUIC_FLOW),
            "::",
            stringify!(CidOffset)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).CidData) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(_XDP_QUIC_FLOW),
            "::",
            stringify!(CidData)
        )
    );
}
pub type XDP_QUIC_FLOW = _XDP_QUIC_FLOW;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _XDP_PORT_SET {
    pub PortSet: *const UINT8,
    pub Reserved: *mut ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout__XDP_PORT_SET() {
    const UNINIT: ::std::mem::MaybeUninit<_XDP_PORT_SET> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_XDP_PORT_SET>(),
        16usize,
        concat!("Size of: ", stringify!(_XDP_PORT_SET))
    );
    assert_eq!(
        ::std::mem::align_of::<_XDP_PORT_SET>(),
        8usize,
        concat!("Alignment of ", stringify!(_XDP_PORT_SET))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).PortSet) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_XDP_PORT_SET),
            "::",
            stringify!(PortSet)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Reserved) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_XDP_PORT_SET),
            "::",
            stringify!(Reserved)
        )
    );
}
pub type XDP_PORT_SET = _XDP_PORT_SET;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _XDP_IP_PORT_SET {
    pub Address: XDP_INET_ADDR,
    pub PortSet: XDP_PORT_SET,
}
#[test]
fn bindgen_test_layout__XDP_IP_PORT_SET() {
    const UNINIT: ::std::mem::MaybeUninit<_XDP_IP_PORT_SET> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_XDP_IP_PORT_SET>(),
        32usize,
        concat!("Size of: ", stringify!(_XDP_IP_PORT_SET))
    );
    assert_eq!(
        ::std::mem::align_of::<_XDP_IP_PORT_SET>(),
        8usize,
        concat!("Alignment of ", stringify!(_XDP_IP_PORT_SET))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Address) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_XDP_IP_PORT_SET),
            "::",
            stringify!(Address)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).PortSet) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_XDP_IP_PORT_SET),
            "::",
            stringify!(PortSet)
        )
    );
}
pub type XDP_IP_PORT_SET = _XDP_IP_PORT_SET;
#[repr(C)]
#[derive(Copy, Clone)]
pub union _XDP_MATCH_PATTERN {
    pub Port: UINT16,
    pub IpMask: XDP_IP_ADDRESS_MASK,
    pub Tuple: XDP_TUPLE,
    pub QuicFlow: XDP_QUIC_FLOW,
    pub PortSet: XDP_PORT_SET,
    pub IpPortSet: XDP_IP_PORT_SET,
}
#[test]
fn bindgen_test_layout__XDP_MATCH_PATTERN() {
    const UNINIT: ::std::mem::MaybeUninit<_XDP_MATCH_PATTERN> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_XDP_MATCH_PATTERN>(),
        40usize,
        concat!("Size of: ", stringify!(_XDP_MATCH_PATTERN))
    );
    assert_eq!(
        ::std::mem::align_of::<_XDP_MATCH_PATTERN>(),
        8usize,
        concat!("Alignment of ", stringify!(_XDP_MATCH_PATTERN))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Port) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_XDP_MATCH_PATTERN),
            "::",
            stringify!(Port)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).IpMask) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_XDP_MATCH_PATTERN),
            "::",
            stringify!(IpMask)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Tuple) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_XDP_MATCH_PATTERN),
            "::",
            stringify!(Tuple)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).QuicFlow) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_XDP_MATCH_PATTERN),
            "::",
            stringify!(QuicFlow)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).PortSet) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_XDP_MATCH_PATTERN),
            "::",
            stringify!(PortSet)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).IpPortSet) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_XDP_MATCH_PATTERN),
            "::",
            stringify!(IpPortSet)
        )
    );
}
pub type XDP_MATCH_PATTERN = _XDP_MATCH_PATTERN;
pub const _XDP_RULE_ACTION_XDP_PROGRAM_ACTION_DROP: _XDP_RULE_ACTION = 0;
pub const _XDP_RULE_ACTION_XDP_PROGRAM_ACTION_PASS: _XDP_RULE_ACTION = 1;
pub const _XDP_RULE_ACTION_XDP_PROGRAM_ACTION_REDIRECT: _XDP_RULE_ACTION = 2;
pub const _XDP_RULE_ACTION_XDP_PROGRAM_ACTION_L2FWD: _XDP_RULE_ACTION = 3;
pub const _XDP_RULE_ACTION_XDP_PROGRAM_ACTION_EBPF: _XDP_RULE_ACTION = 4;
pub type _XDP_RULE_ACTION = ::std::os::raw::c_int;
pub use self::_XDP_RULE_ACTION as XDP_RULE_ACTION;
pub const _XDP_REDIRECT_TARGET_TYPE_XDP_REDIRECT_TARGET_TYPE_XSK: _XDP_REDIRECT_TARGET_TYPE = 0;
pub type _XDP_REDIRECT_TARGET_TYPE = ::std::os::raw::c_int;
pub use self::_XDP_REDIRECT_TARGET_TYPE as XDP_REDIRECT_TARGET_TYPE;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _XDP_REDIRECT_PARAMS {
    pub TargetType: XDP_REDIRECT_TARGET_TYPE,
    pub Target: HANDLE,
}
#[test]
fn bindgen_test_layout__XDP_REDIRECT_PARAMS() {
    const UNINIT: ::std::mem::MaybeUninit<_XDP_REDIRECT_PARAMS> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_XDP_REDIRECT_PARAMS>(),
        16usize,
        concat!("Size of: ", stringify!(_XDP_REDIRECT_PARAMS))
    );
    assert_eq!(
        ::std::mem::align_of::<_XDP_REDIRECT_PARAMS>(),
        8usize,
        concat!("Alignment of ", stringify!(_XDP_REDIRECT_PARAMS))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).TargetType) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_XDP_REDIRECT_PARAMS),
            "::",
            stringify!(TargetType)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Target) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_XDP_REDIRECT_PARAMS),
            "::",
            stringify!(Target)
        )
    );
}
pub type XDP_REDIRECT_PARAMS = _XDP_REDIRECT_PARAMS;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _XDP_EBPF_PARAMS {
    pub Target: HANDLE,
}
#[test]
fn bindgen_test_layout__XDP_EBPF_PARAMS() {
    const UNINIT: ::std::mem::MaybeUninit<_XDP_EBPF_PARAMS> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_XDP_EBPF_PARAMS>(),
        8usize,
        concat!("Size of: ", stringify!(_XDP_EBPF_PARAMS))
    );
    assert_eq!(
        ::std::mem::align_of::<_XDP_EBPF_PARAMS>(),
        8usize,
        concat!("Alignment of ", stringify!(_XDP_EBPF_PARAMS))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Target) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_XDP_EBPF_PARAMS),
            "::",
            stringify!(Target)
        )
    );
}
pub type XDP_EBPF_PARAMS = _XDP_EBPF_PARAMS;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _XDP_RULE {
    pub Match: XDP_MATCH_TYPE,
    pub Pattern: XDP_MATCH_PATTERN,
    pub Action: XDP_RULE_ACTION,
    pub __bindgen_anon_1: _XDP_RULE__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union _XDP_RULE__bindgen_ty_1 {
    pub Redirect: XDP_REDIRECT_PARAMS,
    pub Ebpf: XDP_EBPF_PARAMS,
}
#[test]
fn bindgen_test_layout__XDP_RULE__bindgen_ty_1() {
    const UNINIT: ::std::mem::MaybeUninit<_XDP_RULE__bindgen_ty_1> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_XDP_RULE__bindgen_ty_1>(),
        16usize,
        concat!("Size of: ", stringify!(_XDP_RULE__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<_XDP_RULE__bindgen_ty_1>(),
        8usize,
        concat!("Alignment of ", stringify!(_XDP_RULE__bindgen_ty_1))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Redirect) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_XDP_RULE__bindgen_ty_1),
            "::",
            stringify!(Redirect)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Ebpf) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_XDP_RULE__bindgen_ty_1),
            "::",
            stringify!(Ebpf)
        )
    );
}
#[test]
fn bindgen_test_layout__XDP_RULE() {
    const UNINIT: ::std::mem::MaybeUninit<_XDP_RULE> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_XDP_RULE>(),
        72usize,
        concat!("Size of: ", stringify!(_XDP_RULE))
    );
    assert_eq!(
        ::std::mem::align_of::<_XDP_RULE>(),
        8usize,
        concat!("Alignment of ", stringify!(_XDP_RULE))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Match) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_XDP_RULE),
            "::",
            stringify!(Match)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Pattern) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_XDP_RULE),
            "::",
            stringify!(Pattern)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Action) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(_XDP_RULE),
            "::",
            stringify!(Action)
        )
    );
}
pub type XDP_RULE = _XDP_RULE;
pub const _XDP_CREATE_PROGRAM_FLAGS_XDP_CREATE_PROGRAM_FLAG_NONE: _XDP_CREATE_PROGRAM_FLAGS = 0;
pub const _XDP_CREATE_PROGRAM_FLAGS_XDP_CREATE_PROGRAM_FLAG_GENERIC: _XDP_CREATE_PROGRAM_FLAGS = 1;
pub const _XDP_CREATE_PROGRAM_FLAGS_XDP_CREATE_PROGRAM_FLAG_NATIVE: _XDP_CREATE_PROGRAM_FLAGS = 2;
pub const _XDP_CREATE_PROGRAM_FLAGS_XDP_CREATE_PROGRAM_FLAG_ALL_QUEUES: _XDP_CREATE_PROGRAM_FLAGS =
    4;
pub type _XDP_CREATE_PROGRAM_FLAGS = ::std::os::raw::c_int;
pub use self::_XDP_CREATE_PROGRAM_FLAGS as XDP_CREATE_PROGRAM_FLAGS;
pub type XDP_CREATE_PROGRAM_FN = ::std::option::Option<
    unsafe extern "C" fn(
        InterfaceIndex: UINT32,
        HookId: *const XDP_HOOK_ID,
        QueueId: UINT32,
        Flags: XDP_CREATE_PROGRAM_FLAGS,
        Rules: *const XDP_RULE,
        RuleCount: UINT32,
        Program: *mut HANDLE,
    ) -> HRESULT,
>;
pub type XDP_INTERFACE_OPEN_FN = ::std::option::Option<
    unsafe extern "C" fn(InterfaceIndex: UINT32, InterfaceHandle: *mut HANDLE) -> HRESULT,
>;
#[repr(C)]
#[derive(Copy, Clone)]
pub union _XSK_BUFFER_ADDRESS {
    pub __bindgen_anon_1: _XSK_BUFFER_ADDRESS__bindgen_ty_1,
    pub AddressAndOffset: UINT64,
}
#[repr(C)]
#[repr(align(8))]
#[derive(Debug, Copy, Clone)]
pub struct _XSK_BUFFER_ADDRESS__bindgen_ty_1 {
    pub _bitfield_align_1: [u64; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 8usize]>,
}
#[test]
fn bindgen_test_layout__XSK_BUFFER_ADDRESS__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<_XSK_BUFFER_ADDRESS__bindgen_ty_1>(),
        8usize,
        concat!("Size of: ", stringify!(_XSK_BUFFER_ADDRESS__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<_XSK_BUFFER_ADDRESS__bindgen_ty_1>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(_XSK_BUFFER_ADDRESS__bindgen_ty_1)
        )
    );
}
impl _XSK_BUFFER_ADDRESS__bindgen_ty_1 {
    #[inline]
    pub fn BaseAddress(&self) -> UINT64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 48u8) as u64) }
    }
    #[inline]
    pub fn set_BaseAddress(&mut self, val: UINT64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 48u8, val as u64)
        }
    }
    #[inline]
    pub fn Offset(&self) -> UINT64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(48usize, 16u8) as u64) }
    }
    #[inline]
    pub fn set_Offset(&mut self, val: UINT64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(48usize, 16u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        BaseAddress: UINT64,
        Offset: UINT64,
    ) -> __BindgenBitfieldUnit<[u8; 8usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 8usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 48u8, {
            let BaseAddress: u64 = unsafe { ::std::mem::transmute(BaseAddress) };
            BaseAddress as u64
        });
        __bindgen_bitfield_unit.set(48usize, 16u8, {
            let Offset: u64 = unsafe { ::std::mem::transmute(Offset) };
            Offset as u64
        });
        __bindgen_bitfield_unit
    }
}
#[test]
fn bindgen_test_layout__XSK_BUFFER_ADDRESS() {
    const UNINIT: ::std::mem::MaybeUninit<_XSK_BUFFER_ADDRESS> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_XSK_BUFFER_ADDRESS>(),
        8usize,
        concat!("Size of: ", stringify!(_XSK_BUFFER_ADDRESS))
    );
    assert_eq!(
        ::std::mem::align_of::<_XSK_BUFFER_ADDRESS>(),
        8usize,
        concat!("Alignment of ", stringify!(_XSK_BUFFER_ADDRESS))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).AddressAndOffset) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_XSK_BUFFER_ADDRESS),
            "::",
            stringify!(AddressAndOffset)
        )
    );
}
pub type XSK_BUFFER_ADDRESS = _XSK_BUFFER_ADDRESS;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _XSK_BUFFER_DESCRIPTOR {
    pub Address: XSK_BUFFER_ADDRESS,
    pub Length: UINT32,
    pub Reserved: UINT32,
}
#[test]
fn bindgen_test_layout__XSK_BUFFER_DESCRIPTOR() {
    const UNINIT: ::std::mem::MaybeUninit<_XSK_BUFFER_DESCRIPTOR> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_XSK_BUFFER_DESCRIPTOR>(),
        16usize,
        concat!("Size of: ", stringify!(_XSK_BUFFER_DESCRIPTOR))
    );
    assert_eq!(
        ::std::mem::align_of::<_XSK_BUFFER_DESCRIPTOR>(),
        8usize,
        concat!("Alignment of ", stringify!(_XSK_BUFFER_DESCRIPTOR))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Address) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_XSK_BUFFER_DESCRIPTOR),
            "::",
            stringify!(Address)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Length) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_XSK_BUFFER_DESCRIPTOR),
            "::",
            stringify!(Length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Reserved) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(_XSK_BUFFER_DESCRIPTOR),
            "::",
            stringify!(Reserved)
        )
    );
}
pub type XSK_BUFFER_DESCRIPTOR = _XSK_BUFFER_DESCRIPTOR;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _XSK_FRAME_DESCRIPTOR {
    pub Buffer: XSK_BUFFER_DESCRIPTOR,
}
#[test]
fn bindgen_test_layout__XSK_FRAME_DESCRIPTOR() {
    const UNINIT: ::std::mem::MaybeUninit<_XSK_FRAME_DESCRIPTOR> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_XSK_FRAME_DESCRIPTOR>(),
        16usize,
        concat!("Size of: ", stringify!(_XSK_FRAME_DESCRIPTOR))
    );
    assert_eq!(
        ::std::mem::align_of::<_XSK_FRAME_DESCRIPTOR>(),
        8usize,
        concat!("Alignment of ", stringify!(_XSK_FRAME_DESCRIPTOR))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Buffer) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_XSK_FRAME_DESCRIPTOR),
            "::",
            stringify!(Buffer)
        )
    );
}
pub type XSK_FRAME_DESCRIPTOR = _XSK_FRAME_DESCRIPTOR;
pub const _XSK_RING_FLAGS_XSK_RING_FLAG_NONE: _XSK_RING_FLAGS = 0;
pub const _XSK_RING_FLAGS_XSK_RING_FLAG_ERROR: _XSK_RING_FLAGS = 1;
pub const _XSK_RING_FLAGS_XSK_RING_FLAG_NEED_POKE: _XSK_RING_FLAGS = 2;
pub const _XSK_RING_FLAGS_XSK_RING_FLAG_AFFINITY_CHANGED: _XSK_RING_FLAGS = 4;
pub type _XSK_RING_FLAGS = ::std::os::raw::c_int;
pub use self::_XSK_RING_FLAGS as XSK_RING_FLAGS;
pub type XSK_CREATE_FN =
    ::std::option::Option<unsafe extern "C" fn(Socket: *mut HANDLE) -> HRESULT>;
pub const _XSK_BIND_FLAGS_XSK_BIND_FLAG_NONE: _XSK_BIND_FLAGS = 0;
pub const _XSK_BIND_FLAGS_XSK_BIND_FLAG_RX: _XSK_BIND_FLAGS = 1;
pub const _XSK_BIND_FLAGS_XSK_BIND_FLAG_TX: _XSK_BIND_FLAGS = 2;
pub const _XSK_BIND_FLAGS_XSK_BIND_FLAG_GENERIC: _XSK_BIND_FLAGS = 4;
pub const _XSK_BIND_FLAGS_XSK_BIND_FLAG_NATIVE: _XSK_BIND_FLAGS = 8;
pub type _XSK_BIND_FLAGS = ::std::os::raw::c_int;
pub use self::_XSK_BIND_FLAGS as XSK_BIND_FLAGS;
pub type XSK_BIND_FN = ::std::option::Option<
    unsafe extern "C" fn(
        Socket: HANDLE,
        IfIndex: UINT32,
        QueueId: UINT32,
        Flags: XSK_BIND_FLAGS,
    ) -> HRESULT,
>;
pub const _XSK_ACTIVATE_FLAGS_XSK_ACTIVATE_FLAG_NONE: _XSK_ACTIVATE_FLAGS = 0;
pub type _XSK_ACTIVATE_FLAGS = ::std::os::raw::c_int;
pub use self::_XSK_ACTIVATE_FLAGS as XSK_ACTIVATE_FLAGS;
pub type XSK_ACTIVATE_FN = ::std::option::Option<
    unsafe extern "C" fn(Socket: HANDLE, Flags: XSK_ACTIVATE_FLAGS) -> HRESULT,
>;
pub const _XSK_NOTIFY_FLAGS_XSK_NOTIFY_FLAG_NONE: _XSK_NOTIFY_FLAGS = 0;
pub const _XSK_NOTIFY_FLAGS_XSK_NOTIFY_FLAG_POKE_RX: _XSK_NOTIFY_FLAGS = 1;
pub const _XSK_NOTIFY_FLAGS_XSK_NOTIFY_FLAG_POKE_TX: _XSK_NOTIFY_FLAGS = 2;
pub const _XSK_NOTIFY_FLAGS_XSK_NOTIFY_FLAG_WAIT_RX: _XSK_NOTIFY_FLAGS = 4;
pub const _XSK_NOTIFY_FLAGS_XSK_NOTIFY_FLAG_WAIT_TX: _XSK_NOTIFY_FLAGS = 8;
pub type _XSK_NOTIFY_FLAGS = ::std::os::raw::c_int;
pub use self::_XSK_NOTIFY_FLAGS as XSK_NOTIFY_FLAGS;
pub const _XSK_NOTIFY_RESULT_FLAGS_XSK_NOTIFY_RESULT_FLAG_NONE: _XSK_NOTIFY_RESULT_FLAGS = 0;
pub const _XSK_NOTIFY_RESULT_FLAGS_XSK_NOTIFY_RESULT_FLAG_RX_AVAILABLE: _XSK_NOTIFY_RESULT_FLAGS =
    1;
pub const _XSK_NOTIFY_RESULT_FLAGS_XSK_NOTIFY_RESULT_FLAG_TX_COMP_AVAILABLE:
    _XSK_NOTIFY_RESULT_FLAGS = 2;
pub type _XSK_NOTIFY_RESULT_FLAGS = ::std::os::raw::c_int;
pub use self::_XSK_NOTIFY_RESULT_FLAGS as XSK_NOTIFY_RESULT_FLAGS;
pub type XSK_NOTIFY_SOCKET_FN = ::std::option::Option<
    unsafe extern "C" fn(
        Socket: HANDLE,
        Flags: XSK_NOTIFY_FLAGS,
        WaitTimeoutMilliseconds: UINT32,
        Result: *mut XSK_NOTIFY_RESULT_FLAGS,
    ) -> HRESULT,
>;
pub type XSK_NOTIFY_ASYNC_FN = ::std::option::Option<
    unsafe extern "C" fn(
        Socket: HANDLE,
        Flags: XSK_NOTIFY_FLAGS,
        Overlapped: *mut OVERLAPPED,
    ) -> HRESULT,
>;
pub type XSK_GET_NOTIFY_ASYNC_RESULT_FN = ::std::option::Option<
    unsafe extern "C" fn(
        Overlapped: *mut OVERLAPPED,
        Result: *mut XSK_NOTIFY_RESULT_FLAGS,
    ) -> HRESULT,
>;
pub type XSK_SET_SOCKOPT_FN = ::std::option::Option<
    unsafe extern "C" fn(
        Socket: HANDLE,
        OptionName: UINT32,
        OptionValue: *const ::std::os::raw::c_void,
        OptionLength: UINT32,
    ) -> HRESULT,
>;
pub type XSK_GET_SOCKOPT_FN = ::std::option::Option<
    unsafe extern "C" fn(
        Socket: HANDLE,
        OptionName: UINT32,
        OptionValue: *mut ::std::os::raw::c_void,
        OptionLength: *mut UINT32,
    ) -> HRESULT,
>;
pub type XSK_IOCTL_FN = ::std::option::Option<
    unsafe extern "C" fn(
        Socket: HANDLE,
        OptionName: UINT32,
        InputValue: *const ::std::os::raw::c_void,
        InputLength: UINT32,
        OutputValue: *mut ::std::os::raw::c_void,
        OutputLength: *mut UINT32,
    ) -> HRESULT,
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _XSK_UMEM_REG {
    pub TotalSize: UINT64,
    pub ChunkSize: UINT32,
    pub Headroom: UINT32,
    pub Address: *mut ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout__XSK_UMEM_REG() {
    const UNINIT: ::std::mem::MaybeUninit<_XSK_UMEM_REG> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_XSK_UMEM_REG>(),
        24usize,
        concat!("Size of: ", stringify!(_XSK_UMEM_REG))
    );
    assert_eq!(
        ::std::mem::align_of::<_XSK_UMEM_REG>(),
        8usize,
        concat!("Alignment of ", stringify!(_XSK_UMEM_REG))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).TotalSize) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_XSK_UMEM_REG),
            "::",
            stringify!(TotalSize)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ChunkSize) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_XSK_UMEM_REG),
            "::",
            stringify!(ChunkSize)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Headroom) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(_XSK_UMEM_REG),
            "::",
            stringify!(Headroom)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Address) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_XSK_UMEM_REG),
            "::",
            stringify!(Address)
        )
    );
}
pub type XSK_UMEM_REG = _XSK_UMEM_REG;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _XSK_RING_INFO {
    pub Ring: *mut BYTE,
    pub DescriptorsOffset: UINT32,
    pub ProducerIndexOffset: UINT32,
    pub ConsumerIndexOffset: UINT32,
    pub FlagsOffset: UINT32,
    pub Size: UINT32,
    pub ElementStride: UINT32,
    pub Reserved: UINT32,
}
#[test]
fn bindgen_test_layout__XSK_RING_INFO() {
    const UNINIT: ::std::mem::MaybeUninit<_XSK_RING_INFO> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_XSK_RING_INFO>(),
        40usize,
        concat!("Size of: ", stringify!(_XSK_RING_INFO))
    );
    assert_eq!(
        ::std::mem::align_of::<_XSK_RING_INFO>(),
        8usize,
        concat!("Alignment of ", stringify!(_XSK_RING_INFO))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Ring) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_XSK_RING_INFO),
            "::",
            stringify!(Ring)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).DescriptorsOffset) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_XSK_RING_INFO),
            "::",
            stringify!(DescriptorsOffset)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ProducerIndexOffset) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(_XSK_RING_INFO),
            "::",
            stringify!(ProducerIndexOffset)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ConsumerIndexOffset) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_XSK_RING_INFO),
            "::",
            stringify!(ConsumerIndexOffset)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).FlagsOffset) as usize - ptr as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(_XSK_RING_INFO),
            "::",
            stringify!(FlagsOffset)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Size) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(_XSK_RING_INFO),
            "::",
            stringify!(Size)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ElementStride) as usize - ptr as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(_XSK_RING_INFO),
            "::",
            stringify!(ElementStride)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Reserved) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(_XSK_RING_INFO),
            "::",
            stringify!(Reserved)
        )
    );
}
pub type XSK_RING_INFO = _XSK_RING_INFO;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _XSK_RING_INFO_SET {
    pub Fill: XSK_RING_INFO,
    pub Completion: XSK_RING_INFO,
    pub Rx: XSK_RING_INFO,
    pub Tx: XSK_RING_INFO,
}
#[test]
fn bindgen_test_layout__XSK_RING_INFO_SET() {
    const UNINIT: ::std::mem::MaybeUninit<_XSK_RING_INFO_SET> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_XSK_RING_INFO_SET>(),
        160usize,
        concat!("Size of: ", stringify!(_XSK_RING_INFO_SET))
    );
    assert_eq!(
        ::std::mem::align_of::<_XSK_RING_INFO_SET>(),
        8usize,
        concat!("Alignment of ", stringify!(_XSK_RING_INFO_SET))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Fill) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_XSK_RING_INFO_SET),
            "::",
            stringify!(Fill)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Completion) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(_XSK_RING_INFO_SET),
            "::",
            stringify!(Completion)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Rx) as usize - ptr as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(_XSK_RING_INFO_SET),
            "::",
            stringify!(Rx)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Tx) as usize - ptr as usize },
        120usize,
        concat!(
            "Offset of field: ",
            stringify!(_XSK_RING_INFO_SET),
            "::",
            stringify!(Tx)
        )
    );
}
pub type XSK_RING_INFO_SET = _XSK_RING_INFO_SET;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _XSK_STATISTICS {
    pub RxDropped: UINT64,
    pub RxTruncated: UINT64,
    pub RxInvalidDescriptors: UINT64,
    pub TxInvalidDescriptors: UINT64,
}
#[test]
fn bindgen_test_layout__XSK_STATISTICS() {
    const UNINIT: ::std::mem::MaybeUninit<_XSK_STATISTICS> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_XSK_STATISTICS>(),
        32usize,
        concat!("Size of: ", stringify!(_XSK_STATISTICS))
    );
    assert_eq!(
        ::std::mem::align_of::<_XSK_STATISTICS>(),
        8usize,
        concat!("Alignment of ", stringify!(_XSK_STATISTICS))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).RxDropped) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_XSK_STATISTICS),
            "::",
            stringify!(RxDropped)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).RxTruncated) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_XSK_STATISTICS),
            "::",
            stringify!(RxTruncated)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).RxInvalidDescriptors) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_XSK_STATISTICS),
            "::",
            stringify!(RxInvalidDescriptors)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).TxInvalidDescriptors) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(_XSK_STATISTICS),
            "::",
            stringify!(TxInvalidDescriptors)
        )
    );
}
pub type XSK_STATISTICS = _XSK_STATISTICS;
pub const _XSK_ERROR_XSK_NO_ERROR: _XSK_ERROR = 0;
pub const _XSK_ERROR_XSK_ERROR_INTERFACE_DETACH: _XSK_ERROR = -2147483648;
pub const _XSK_ERROR_XSK_ERROR_INVALID_RING: _XSK_ERROR = -1073741824;
pub type _XSK_ERROR = ::std::os::raw::c_int;
pub use self::_XSK_ERROR as XSK_ERROR;
pub type XDP_API_TABLE = _XDP_API_TABLE;
pub type XDP_OPEN_API_FN = ::std::option::Option<
    unsafe extern "C" fn(XdpApiVersion: UINT32, XdpApiTable: *mut *const XDP_API_TABLE) -> HRESULT,
>;
pub type XDP_CLOSE_API_FN =
    ::std::option::Option<unsafe extern "C" fn(XdpApiTable: *const XDP_API_TABLE)>;

pub type XDP_GET_ROUTINE_FN = ::std::option::Option<
    unsafe extern "C" fn(RoutineName: *const CHAR) -> *mut ::std::os::raw::c_void,
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _XDP_API_TABLE {
    pub XdpOpenApi: XDP_OPEN_API_FN,
    pub XdpCloseApi: XDP_CLOSE_API_FN,
    pub XdpGetRoutine: XDP_GET_ROUTINE_FN,
    pub XdpCreateProgram: XDP_CREATE_PROGRAM_FN,
    pub XdpInterfaceOpen: XDP_INTERFACE_OPEN_FN,
    pub XskCreate: XSK_CREATE_FN,
    pub XskBind: XSK_BIND_FN,
    pub XskActivate: XSK_ACTIVATE_FN,
    pub XskNotifySocket: XSK_NOTIFY_SOCKET_FN,
    pub XskNotifyAsync: XSK_NOTIFY_ASYNC_FN,
    pub XskGetNotifyAsyncResult: XSK_GET_NOTIFY_ASYNC_RESULT_FN,
    pub XskSetSockopt: XSK_SET_SOCKOPT_FN,
    pub XskGetSockopt: XSK_GET_SOCKOPT_FN,
    pub XskIoctl: XSK_IOCTL_FN,
}
#[test]
fn bindgen_test_layout__XDP_API_TABLE() {
    const UNINIT: ::std::mem::MaybeUninit<_XDP_API_TABLE> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_XDP_API_TABLE>(),
        112usize,
        concat!("Size of: ", stringify!(_XDP_API_TABLE))
    );
    assert_eq!(
        ::std::mem::align_of::<_XDP_API_TABLE>(),
        8usize,
        concat!("Alignment of ", stringify!(_XDP_API_TABLE))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).XdpOpenApi) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_XDP_API_TABLE),
            "::",
            stringify!(XdpOpenApi)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).XdpCloseApi) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_XDP_API_TABLE),
            "::",
            stringify!(XdpCloseApi)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).XdpGetRoutine) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_XDP_API_TABLE),
            "::",
            stringify!(XdpGetRoutine)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).XdpCreateProgram) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(_XDP_API_TABLE),
            "::",
            stringify!(XdpCreateProgram)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).XdpInterfaceOpen) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(_XDP_API_TABLE),
            "::",
            stringify!(XdpInterfaceOpen)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).XskCreate) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(_XDP_API_TABLE),
            "::",
            stringify!(XskCreate)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).XskBind) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(_XDP_API_TABLE),
            "::",
            stringify!(XskBind)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).XskActivate) as usize - ptr as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(_XDP_API_TABLE),
            "::",
            stringify!(XskActivate)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).XskNotifySocket) as usize - ptr as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(_XDP_API_TABLE),
            "::",
            stringify!(XskNotifySocket)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).XskNotifyAsync) as usize - ptr as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(_XDP_API_TABLE),
            "::",
            stringify!(XskNotifyAsync)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).XskGetNotifyAsyncResult) as usize - ptr as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(_XDP_API_TABLE),
            "::",
            stringify!(XskGetNotifyAsyncResult)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).XskSetSockopt) as usize - ptr as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(_XDP_API_TABLE),
            "::",
            stringify!(XskSetSockopt)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).XskGetSockopt) as usize - ptr as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(_XDP_API_TABLE),
            "::",
            stringify!(XskGetSockopt)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).XskIoctl) as usize - ptr as usize },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(_XDP_API_TABLE),
            "::",
            stringify!(XskIoctl)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _XDP_LOAD_CONTEXT {
    _unused: [u8; 0],
}
pub type XDP_LOAD_API_CONTEXT = *mut _XDP_LOAD_CONTEXT;
pub type __builtin_va_list = *mut ::std::os::raw::c_char;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __crt_locale_data {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __crt_multibyte_data {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _ACTIVATION_CONTEXT {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct NET_ADDRESS_INFO_ {
    pub _address: u8,
}

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
