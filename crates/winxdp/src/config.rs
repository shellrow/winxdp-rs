use crate::{Error, Result};
/// Hardware/software RSS queue index, not a processor index.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct QueueId(pub u32);
impl QueueId {
    pub const fn new(id: u32) -> Self {
        Self(id)
    }
}
/// Auto delegates selection to the driver's documented bind behavior.
#[derive(Clone, Copy, Debug, Default)]
pub enum BindMode {
    #[default]
    Auto,
    Generic,
    Native,
}
#[derive(Clone, Debug)]
pub struct Config {
    pub interface: u32,
    pub queue: QueueId,
    pub receive: bool,
    pub transmit: bool,
    pub bind_mode: BindMode,
    pub frame_size: u32,
    pub frame_count: u32,
    pub headroom: u32,
    pub rx_ring_size: u32,
    pub tx_ring_size: u32,
    pub rx_checksum: bool,
    pub tx_checksum: bool,
    pub rx_timestamp: bool,
    pub tx_timestamp: bool,
    pub original_length: bool,
    pub rx_hook: Option<Hook>,
    pub tx_hook: Option<Hook>,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            interface: 0,
            queue: QueueId(0),
            receive: false,
            transmit: true,
            bind_mode: BindMode::Auto,
            frame_size: 2048,
            frame_count: 4096,
            headroom: 0,
            rx_ring_size: 1024,
            tx_ring_size: 1024,
            rx_checksum: false,
            tx_checksum: false,
            rx_timestamp: false,
            tx_timestamp: false,
            original_length: true,
            rx_hook: None,
            tx_hook: None,
        }
    }
}
impl Config {
    pub fn validate(&self) -> Result<()> {
        let bad = |s| Err(Error::InvalidConfiguration(s));
        if self.interface == 0 {
            return bad("interface index must be nonzero");
        }
        if !self.receive && !self.transmit {
            return bad("enable RX or TX");
        }
        if self.frame_size == 0
            || self.headroom >= self.frame_size
            || self.headroom > u16::MAX as u32
        {
            return bad("invalid frame size or headroom");
        }
        if self.frame_count == 0 || self.frame_size.checked_mul(self.frame_count).is_none() {
            return bad("UMEM must contain frames and fit in UINT32");
        }
        for (enabled, size) in [
            (self.receive, self.rx_ring_size),
            (self.transmit, self.tx_ring_size),
        ] {
            if enabled && (!size.is_power_of_two() || size > (1 << 30)) {
                return bad("ring size must be a power of two at most 2^30");
            }
        }
        if self.receive
            && self.frame_count < self.rx_ring_size.saturating_add(u32::from(self.transmit))
        {
            return bad("pool must cover the fill ring and at least one TX frame");
        }
        if (!self.receive && (self.rx_checksum || self.rx_timestamp))
            || (!self.transmit && (self.tx_checksum || self.tx_timestamp))
        {
            return bad("extension requires its direction");
        }
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn configuration_boundaries() {
        let mut c = Config {
            interface: 1,
            ..Config::default()
        };
        assert!(c.validate().is_ok());
        c.tx_ring_size = 3;
        assert!(c.validate().is_err());
        c.tx_ring_size = 4;
        c.frame_count = u32::MAX;
        assert!(c.validate().is_err());
        c.frame_count = 4096;
        c.headroom = c.frame_size;
        assert!(c.validate().is_err());
    }
}
/// Optional L2 hook selection. Driver bind validates actual hook availability.
#[derive(Clone, Copy, Debug)]
pub enum Hook {
    ReceiveInspect,
    ReceiveInject,
    TransmitInspect,
    TransmitInject,
}
