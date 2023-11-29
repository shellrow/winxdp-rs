/// XDP hook layer
pub use crate::bindings::xdpapi::XDP_HOOK_LAYER;
pub const XDP_HOOK_L2: XDP_HOOK_LAYER = 0;

/// XDP hook direction
pub use crate::bindings::xdpapi::XDP_HOOK_DATAPATH_DIRECTION;
pub const XDP_HOOK_RX: XDP_HOOK_DATAPATH_DIRECTION = 0;
pub const XDP_HOOK_TX: XDP_HOOK_DATAPATH_DIRECTION = 1;

/// XDP hook sublayer
pub use crate::bindings::xdpapi::XDP_HOOK_SUBLAYER;
pub const XDP_HOOK_INSPECT: XDP_HOOK_SUBLAYER = 0;
pub const XDP_HOOK_INJECT: XDP_HOOK_SUBLAYER = 1;

/// XDP hook ID
pub use crate::bindings::xdpapi::XDP_HOOK_ID;
