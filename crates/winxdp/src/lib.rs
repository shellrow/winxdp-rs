//! Scoped, single-thread-owned XDP packet I/O.
//!
//! RX requires explicit external steering; redirect is not passive capture.
//! Borrowed batches avoid extra user-space copies, without claiming NIC DMA zero-copy.
//!
//! RX views cannot escape their batch:
//! ```compile_fail
//! fn escape(s: &mut winxdp::Socket) -> winxdp::Result<&[u8]> {
//!     s.with_rx_batch(1, |b| Ok(b.frame(0).unwrap().bytes()))
//! }
//! ```
//! A submitted TX frame cannot still be mutated:
//! ```compile_fail
//! fn modify_after_submit(mut f: winxdp::TxFrame<'_, '_>) {
//!     let bytes = f.bytes_mut();
//!     f.submit(60).unwrap();
//!     bytes[0] = 1;
//! }
//! ```
//! Sockets do not implement Send:
//! ```compile_fail
//! fn require_send<T: Send>() {}
//! require_send::<winxdp::Socket>();
//! ```
pub use winxdp_sys as sys;
mod config;
mod error;
pub use config::*;
pub use error::*;
#[cfg(all(windows, feature = "async"))]
mod notification;
#[cfg(all(windows, feature = "async"))]
pub use notification::Notification;
#[cfg(windows)]
mod engine;
#[cfg(windows)]
pub use engine::*;

#[cfg(windows)]
mod interface;
#[cfg(windows)]
mod ring;
#[cfg(windows)]
pub use interface::*;

#[cfg(all(windows, feature = "ebpf"))]
pub mod ebpf;
