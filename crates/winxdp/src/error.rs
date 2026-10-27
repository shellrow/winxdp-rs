use std::fmt;
/// Errors retain native HRESULTs when supplied by the driver.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    DriverNotInstalled { code: i32 },
    AccessDenied { code: i32 },
    UnsupportedFeature { context: &'static str, code: i32 },
    InvalidConfiguration(&'static str),
    WouldBlock,
    TimedOut,
    BufferTooSmall { needed: usize, provided: usize },
    DeviceRemoved,
    RingError { flags: u32 },
    SteeringNotConfigured,
    CorruptDescriptor(&'static str),
    Io { code: i32, context: &'static str },
}
pub type Result<T> = std::result::Result<T, Error>;
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for Error {}
#[cfg(windows)]
pub(crate) fn status(code: i32, context: &'static str) -> Result<()> {
    if code >= 0 {
        return Ok(());
    }
    Err(match code as u32 {
        0x80070005 => Error::AccessDenied { code },
        0x80070002 | 0x80070003 if context == "XskCreate" => Error::DriverNotInstalled { code },
        0x80070032 => Error::UnsupportedFeature { context, code },
        0x800705B4 => Error::TimedOut,
        _ => Error::Io { code, context },
    })
}
