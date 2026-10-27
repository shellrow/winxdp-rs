use crate::{BindMode, Config, Error, QueueId, Result, error::status, ring::Ring, sys};
use std::{
    marker::PhantomData,
    mem::{size_of, zeroed},
    ptr,
    rc::Rc,
    time::{Duration, Instant},
};
use windows_sys::Win32::{
    Foundation::{CloseHandle, GetLastError, HANDLE},
    System::Memory::{
        MEM_COMMIT, MEM_RELEASE, MEM_RESERVE, PAGE_READWRITE, VirtualAlloc, VirtualFree,
    },
};

struct Handle(HANDLE);
impl Handle {
    fn create() -> Result<Self> {
        let mut h = ptr::null_mut();
        status(unsafe { sys::XskCreate(&mut h) }, "XskCreate")?;
        Ok(Self(h))
    }
}
impl Drop for Handle {
    fn drop(&mut self) {
        unsafe {
            CloseHandle(self.0);
        }
    }
}
struct Umem {
    ptr: *mut u8,
}
impl Umem {
    fn new(size: usize) -> Result<Self> {
        let ptr: *mut u8 =
            unsafe { VirtualAlloc(ptr::null(), size, MEM_COMMIT | MEM_RESERVE, PAGE_READWRITE) }
                .cast();
        if ptr.is_null() {
            return Err(Error::Io {
                code: unsafe { GetLastError() } as i32,
                context: "VirtualAlloc",
            });
        }
        Ok(Self { ptr })
    }
}
impl Drop for Umem {
    fn drop(&mut self) {
        unsafe {
            VirtualFree(self.ptr.cast(), 0, MEM_RELEASE);
        }
    }
}
/// A fact from probing, never inferred from a NIC name.
#[derive(Debug, Clone, Copy)]
pub enum Capability {
    Supported,
    Unsupported,
    NotQueryable,
}
#[derive(Debug)]
pub struct Capabilities {
    pub api_version: u32,
    pub native: Capability,
    pub passive_capture: Capability,
    pub ebpf: Capability,
}
/// Opening only probes socket creation and closes the probe handle.
pub struct Runtime;
impl Runtime {
    pub fn open() -> Result<Self> {
        drop(Handle::create()?);
        Ok(Self)
    }
    pub fn capabilities(&self) -> Result<Capabilities> {
        Ok(Capabilities {
            api_version: 3,
            native: Capability::NotQueryable,
            passive_capture: Capability::Unsupported,
            ebpf: Capability::NotQueryable,
        })
    }
    pub fn interfaces(&self) -> Result<Vec<crate::Interface>> {
        crate::interface::interfaces()
    }
}
/// Socket construction defaults to TX only. RX must have explicit steering.
pub struct SocketBuilder {
    config: Config,
}
macro_rules! setters { ($($name:ident:$ty:ty),* $(,)?) => { $(pub fn $name(mut self,value:$ty)->Self {self.config.$name=value;self})* }; }
impl SocketBuilder {
    setters!(interface:u32,queue:QueueId,receive:bool,transmit:bool,bind_mode:BindMode,frame_size:u32,frame_count:u32,headroom:u32,rx_ring_size:u32,tx_ring_size:u32,rx_checksum:bool,tx_checksum:bool,rx_timestamp:bool,tx_timestamp:bool,original_length:bool,rx_hook:Option<crate::Hook>,tx_hook:Option<crate::Hook>);
    pub fn config(mut self, config: Config) -> Self {
        self.config = config;
        self
    }
    pub fn build(self) -> Result<Socket> {
        Socket::create(self.config)
    }
}
/// A socket and its pool are confined to their creating thread (neither Send nor Sync).
/// The handle closes before UMEM is released, including on partial construction failure.
pub struct Socket {
    handle: Handle,
    umem: Umem,
    config: Config,
    rx: Option<Ring>,
    fill: Option<Ring>,
    tx: Option<Ring>,
    completion: Option<Ring>,
    states: Vec<State>,
    rx_free: Vec<u32>,
    tx_free: Vec<u32>,
    poisoned: bool,
    steering: bool,
    extensions: Extensions,
    _thread: PhantomData<Rc<()>>,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum State {
    FreeRx,
    DriverRx,
    BorrowedRx,
    FreeTx,
    ReservedTx,
    DriverTx,
}
#[derive(Default, Clone, Copy, Debug)]
struct Extensions {
    rx_checksum: Option<u16>,
    rx_layout: Option<u16>,
    rx_timestamp: Option<u16>,
    original_length: Option<u16>,
    tx_checksum: Option<u16>,
    tx_layout: Option<u16>,
    tx_timestamp: Option<u16>,
}
#[derive(Debug, Default, Clone, Copy)]
pub struct PacketMetadata {
    pub original_length: Option<u32>,
    pub timestamp: Option<u64>,
    /// Raw upstream checksum evaluation bits; not a computed packet checksum.
    pub checksum: Option<sys::XDP_FRAME_CHECKSUM>,
    pub layout: Option<sys::XDP_FRAME_LAYOUT>,
}
#[derive(Debug, Clone, Copy)]
pub struct ReceivedInfo {
    pub length: usize,
    pub metadata: PacketMetadata,
}
/// Unknown flag bits are preserved.
#[derive(Debug, Clone, Copy)]
pub struct NotifyResult(pub u32);
pub type SocketStats = sys::XSK_STATISTICS;
#[derive(Debug, Clone, Copy)]
pub struct TxCompletion {
    pub frame_id: u32,
    pub timestamp: Option<u64>,
}
unsafe fn set<T>(h: HANDLE, opt: u32, v: &T) -> Result<()> {
    status(
        unsafe { sys::XskSetSockopt(h, opt, (v as *const T).cast(), size_of::<T>() as u32) },
        "XskSetSockopt",
    )
}
unsafe fn get<T: Copy>(h: HANDLE, opt: u32) -> Result<T> {
    let mut v: T = unsafe { zeroed() };
    let mut n = size_of::<T>() as u32;
    status(
        unsafe { sys::XskGetSockopt(h, opt, (&mut v as *mut T).cast(), &mut n) },
        "XskGetSockopt",
    )?;
    if n != size_of::<T>() as u32 {
        return Err(Error::CorruptDescriptor("unexpected socket option size"));
    }
    Ok(v)
}
impl Socket {
    pub fn builder(_runtime: &Runtime) -> SocketBuilder {
        SocketBuilder {
            config: Config::default(),
        }
    }
    fn create(config: Config) -> Result<Self> {
        config.validate()?;
        // Allocation precedes handle creation so failure unwinds handle before UMEM.
        let umem = Umem::new(config.frame_size as usize * config.frame_count as usize)?;
        let handle = Handle::create()?;
        let mut s = Self {
            handle,
            umem,
            config,
            rx: None,
            fill: None,
            tx: None,
            completion: None,
            states: Vec::new(),
            rx_free: Vec::new(),
            tx_free: Vec::new(),
            poisoned: false,
            steering: false,
            extensions: Extensions::default(),
            _thread: PhantomData,
        };
        unsafe {
            let c = &s.config;
            set(
                s.handle.0,
                sys::XSK_SOCKOPT_UMEM_REG,
                &sys::XSK_UMEM_REG {
                    TotalSize: c.frame_size as u64 * c.frame_count as u64,
                    ChunkSize: c.frame_size,
                    Headroom: c.headroom,
                    Address: s.umem.ptr.cast(),
                },
            )?;
            for (hook, opt) in [
                (c.rx_hook, sys::XSK_SOCKOPT_RX_HOOK_ID),
                (c.tx_hook, sys::XSK_SOCKOPT_TX_HOOK_ID),
            ] {
                if let Some(hook) = hook {
                    let (direction, sub) = match hook {
                        crate::Hook::ReceiveInspect => (sys::XDP_HOOK_RX, sys::XDP_HOOK_INSPECT),
                        crate::Hook::ReceiveInject => (sys::XDP_HOOK_RX, sys::XDP_HOOK_INJECT),
                        crate::Hook::TransmitInspect => (sys::XDP_HOOK_TX, sys::XDP_HOOK_INSPECT),
                        crate::Hook::TransmitInject => (sys::XDP_HOOK_TX, sys::XDP_HOOK_INJECT),
                    };
                    set(
                        s.handle.0,
                        opt,
                        &sys::XDP_HOOK_ID {
                            Layer: sys::XDP_HOOK_L2,
                            Direction: direction,
                            SubLayer: sub,
                        },
                    )?;
                }
            }
            let flags = if c.receive { sys::XSK_BIND_FLAG_RX } else { 0 }
                | if c.transmit { sys::XSK_BIND_FLAG_TX } else { 0 }
                | match c.bind_mode {
                    BindMode::Auto => 0,
                    BindMode::Generic => sys::XSK_BIND_FLAG_GENERIC,
                    BindMode::Native => sys::XSK_BIND_FLAG_NATIVE,
                };
            status(
                sys::XskBind(s.handle.0, c.interface, c.queue.0, flags),
                "XskBind",
            )?;
            for (on, opt) in [
                (c.rx_checksum, sys::XSK_SOCKOPT_RX_OFFLOAD_CHECKSUM),
                (c.tx_checksum, sys::XSK_SOCKOPT_TX_OFFLOAD_CHECKSUM),
                (c.rx_timestamp, sys::XSK_SOCKOPT_RX_OFFLOAD_TIMESTAMP),
                (c.tx_timestamp, sys::XSK_SOCKOPT_TX_OFFLOAD_TIMESTAMP),
                (
                    c.receive && c.original_length,
                    sys::XSK_SOCKOPT_RX_ORIGINAL_LENGTH,
                ),
            ] {
                if on {
                    set(s.handle.0, opt, &1u32)?;
                }
            }
            if c.receive {
                set(s.handle.0, sys::XSK_SOCKOPT_RX_RING_SIZE, &c.rx_ring_size)?;
                set(
                    s.handle.0,
                    sys::XSK_SOCKOPT_RX_FILL_RING_SIZE,
                    &c.rx_ring_size,
                )?;
            }
            if c.transmit {
                set(s.handle.0, sys::XSK_SOCKOPT_TX_RING_SIZE, &c.tx_ring_size)?;
                set(
                    s.handle.0,
                    sys::XSK_SOCKOPT_TX_COMPLETION_RING_SIZE,
                    &c.tx_ring_size,
                )?;
            }
            status(sys::XskActivate(s.handle.0, 0), "XskActivate")?;
            let info: sys::XSK_RING_INFO_SET = get(s.handle.0, sys::XSK_SOCKOPT_RING_INFO)?;
            if c.receive {
                s.rx = Some(Ring::new(info.Rx, 16)?);
                s.fill = Some(Ring::new(info.Fill, 8)?);
            }
            if c.transmit {
                s.tx = Some(Ring::new(info.Tx, 16)?);
                s.completion = Some(Ring::new(info.Completion, 8)?);
            }
            s.query_extensions()?;
        }
        let rx_count = if s.config.receive {
            s.config.rx_ring_size
        } else {
            0
        };
        let allocation_error = |_| Error::Io {
            code: sys::XDP_STATUS_OUT_OF_MEMORY,
            context: "allocate frame ownership tables",
        };
        s.rx_free
            .try_reserve_exact(rx_count as usize)
            .map_err(allocation_error)?;
        s.tx_free
            .try_reserve_exact((s.config.frame_count - rx_count) as usize)
            .map_err(allocation_error)?;
        s.states
            .try_reserve_exact(s.config.frame_count as usize)
            .map_err(allocation_error)?;
        s.rx_free.extend((0..rx_count).rev());
        s.tx_free.extend((rx_count..s.config.frame_count).rev());
        s.states.extend((0..s.config.frame_count).map(|i| {
            if i < rx_count {
                State::FreeRx
            } else {
                State::FreeTx
            }
        }));
        s.refill()?;
        Ok(s)
    }
    pub(crate) fn healthy(&self) -> Result<()> {
        if self.poisoned {
            Err(Error::CorruptDescriptor("socket quarantined; close it"))
        } else {
            Ok(())
        }
    }
    pub fn configuration(&self) -> &Config {
        &self.config
    }
    /// # Safety
    /// Do not close, reconfigure, register memory, or operate rings concurrently with this socket.
    /// An external steering owner must detach and release references before the socket is dropped.
    pub unsafe fn raw_handle(&self) -> HANDLE {
        self.handle.0
    }
    /// Acknowledge externally owned RX steering. This does not attach any program.
    /// External steering must use this socket on its configured interface and queue.
    pub fn acknowledge_external_steering(&mut self) {
        self.steering = true;
    }
    fn refill(&mut self) -> Result<()> {
        self.healthy()?;
        if let Some(r) = self.fill.as_mut() {
            let (start, n) = r.reserve(true, self.rx_free.len() as u32)?;
            for j in 0..n {
                let id = self.rx_free.pop().unwrap();
                debug_assert_eq!(self.states[id as usize], State::FreeRx);
                self.states[id as usize] = State::DriverRx;
                unsafe {
                    r.write(
                        start.wrapping_add(j),
                        id as u64 * self.config.frame_size as u64,
                    );
                }
            }
            r.advance(true, n);
        }
        Ok(())
    }
    pub fn drain_completions(&mut self) -> Result<usize> {
        self.drain_completions_with(|_| {})
    }
    /// Observe each reclaimed frame's timestamp before recycling. A callback panic cannot double-recycle.
    pub fn drain_completions_with(
        &mut self,
        mut observer: impl FnMut(TxCompletion),
    ) -> Result<usize> {
        self.healthy()?;
        let Some(r) = self.completion.as_mut() else {
            return Ok(0);
        };
        let (start, n) = r.reserve(false, u32::MAX)?;
        for j in 0..n {
            let address: u64 = unsafe { r.read(start.wrapping_add(j)) };
            let id = address / self.config.frame_size as u64;
            if !address.is_multiple_of(self.config.frame_size as u64)
                || id >= self.states.len() as u64
                || self.states[id as usize] != State::DriverTx
            {
                self.poisoned = true;
                return Err(Error::CorruptDescriptor(
                    "invalid or duplicate TX completion",
                ));
            }
            let timestamp = self.extensions.tx_timestamp.map(|off| unsafe {
                ptr::read_unaligned(
                    r.element(start.wrapping_add(j))
                        .add(off as usize)
                        .cast::<sys::XDP_FRAME_TIMESTAMP>(),
                )
                .Timestamp
            });
            self.states[id as usize] = State::FreeTx;
            self.tx_free.push(id as u32);
            r.advance(false, 1);
            observer(TxCompletion {
                frame_id: id as u32,
                timestamp,
            });
        }
        Ok(n as usize)
    }
    /// Nonblocking enqueue; success means queued, not transmitted. Call flush_tx to kick.
    pub fn send(&mut self, frame: &[u8]) -> Result<usize> {
        self.with_tx_batch(1, |b| {
            let mut f = b.next_frame()?.ok_or(Error::WouldBlock)?;
            if frame.len() > f.bytes_mut().len() {
                return Err(Error::BufferTooSmall {
                    needed: frame.len(),
                    provided: f.bytes_mut().len(),
                });
            }
            f.bytes_mut()[..frame.len()].copy_from_slice(frame);
            f.submit(frame.len())?;
            Ok(frame.len())
        })
    }
    pub fn try_send(&mut self, frame: &[u8]) -> Result<usize> {
        self.send(frame)
    }
    /// Returns the number enqueued before backpressure; no implicit blocking.
    pub fn send_batch(&mut self, frames: &[&[u8]]) -> Result<usize> {
        for frame in frames {
            if frame.is_empty()
                || frame.len() > (self.config.frame_size - self.config.headroom) as usize
            {
                return Err(Error::InvalidConfiguration(
                    "all TX frames must be nonempty and fit one chunk",
                ));
            }
        }
        self.with_tx_batch(frames.len().min(u32::MAX as usize) as u32, |b| {
            let mut sent = 0;
            for bytes in frames {
                let Some(mut f) = b.next_frame()? else { break };
                if bytes.len() > f.bytes_mut().len() {
                    return Err(Error::BufferTooSmall {
                        needed: bytes.len(),
                        provided: f.bytes_mut().len(),
                    });
                }
                f.bytes_mut()[..bytes.len()].copy_from_slice(bytes);
                f.submit(bytes.len())?;
                sent += 1;
            }
            Ok(sent)
        })
    }
    pub fn flush_tx(&mut self) -> Result<()> {
        self.healthy()?;
        self.drain_completions()?;
        if self
            .tx
            .as_ref()
            .is_some_and(|r| r.flags() & sys::XSK_RING_FLAG_NEED_POKE as u32 != 0)
        {
            self.notify(sys::XSK_NOTIFY_FLAG_POKE_TX as u32, Duration::ZERO)?;
        }
        Ok(())
    }
    pub fn notify(&mut self, flags: u32, timeout: Duration) -> Result<NotifyResult> {
        self.healthy()?;
        if flags & !15 != 0 {
            return Err(Error::InvalidConfiguration("unknown notify flags"));
        }
        let ms = timeout.as_millis().min(u32::MAX as u128 - 1) as u32;
        let mut result = 0;
        status(
            unsafe { sys::XskNotifySocket(self.handle.0, flags as _, ms, &mut result) },
            "XskNotifySocket",
        )?;
        Ok(NotifyResult(result as u32))
    }
    pub fn wait_rx(&mut self, timeout: Duration) -> Result<NotifyResult> {
        if !self.steering {
            return Err(Error::SteeringNotConfigured);
        }
        self.refill()?;
        self.notify(
            (sys::XSK_NOTIFY_FLAG_POKE_RX | sys::XSK_NOTIFY_FLAG_WAIT_RX) as u32,
            timeout,
        )
    }
    pub fn stats(&self) -> Result<SocketStats> {
        unsafe { get(self.handle.0, sys::XSK_SOCKOPT_STATISTICS) }
    }
    /// Closes after attempting completion drain within a finite deadline. Drop always closes.
    pub fn close(mut self, timeout: Duration) -> Result<()> {
        let start = Instant::now();
        loop {
            self.flush_tx()?;
            if !self.states.contains(&State::DriverTx) {
                return Ok(());
            }
            let remaining = timeout
                .checked_sub(start.elapsed())
                .ok_or(Error::TimedOut)?;
            self.notify(
                (sys::XSK_NOTIFY_FLAG_POKE_TX | sys::XSK_NOTIFY_FLAG_WAIT_TX) as u32,
                remaining,
            )?;
        }
    }
    /// Copies one packet. BufferTooSmall consumes and recycles that packet without copying.
    /// Original wire-length truncation is reported separately in metadata.
    pub fn recv_into(&mut self, out: &mut [u8]) -> Result<ReceivedInfo> {
        self.with_rx_batch(1, |b| {
            let f = b.frame(0).ok_or(Error::WouldBlock)?;
            if out.len() < f.bytes().len() {
                return Err(Error::BufferTooSmall {
                    needed: f.bytes().len(),
                    provided: out.len(),
                });
            }
            out[..f.bytes().len()].copy_from_slice(f.bytes());
            Ok(ReceivedInfo {
                length: f.bytes().len(),
                metadata: f.metadata(),
            })
        })
    }
    pub fn with_rx_batch<T>(
        &mut self,
        max: u32,
        f: impl FnOnce(&RxBatch<'_>) -> Result<T>,
    ) -> Result<T> {
        self.healthy()?;
        if !self.steering {
            return Err(Error::SteeringNotConfigured);
        }
        self.refill()?;
        let r = self
            .rx
            .as_ref()
            .ok_or(Error::InvalidConfiguration("RX disabled"))?;
        let (start, count) = r.reserve(false, max)?;
        for j in 0..count {
            let d: sys::XSK_BUFFER_DESCRIPTOR = unsafe { r.read(start.wrapping_add(j)) };
            let (id, _, _) = match validate_descriptor(&self.config, &d) {
                Ok(v) => v,
                Err(e) => {
                    self.poisoned = true;
                    return Err(e);
                }
            };
            if self.states[id] != State::DriverRx {
                self.poisoned = true;
                return Err(Error::CorruptDescriptor("duplicate or unowned RX frame"));
            }
            self.states[id] = State::BorrowedRx;
        }
        let batch = RxBatch {
            socket: self,
            start,
            count,
        };
        f(&batch)
    }
    pub fn with_tx_batch<T>(
        &mut self,
        max: u32,
        f: impl FnOnce(&mut TxBatch<'_>) -> Result<T>,
    ) -> Result<T> {
        self.healthy()?;
        self.drain_completions()?;
        let r = self
            .tx
            .as_ref()
            .ok_or(Error::InvalidConfiguration("TX disabled"))?;
        let (start, count) = r.reserve(true, max.min(self.tx_free.len() as u32))?;
        let mut batch = TxBatch {
            socket: self,
            start,
            count,
            submitted: 0,
        };
        f(&mut batch)
    }
}
fn validate_descriptor(
    c: &Config,
    d: &sys::XSK_BUFFER_DESCRIPTOR,
) -> Result<(usize, usize, usize)> {
    let packed = unsafe { d.Address.AddressAndOffset };
    let base = packed & ((1u64 << 48) - 1);
    let offset = (packed >> 48) as usize;
    let id = base / c.frame_size as u64;
    if !base.is_multiple_of(c.frame_size as u64)
        || id >= c.frame_count as u64
        || offset
            .checked_add(d.Length as usize)
            .is_none_or(|end| end > c.frame_size as usize)
    {
        return Err(Error::CorruptDescriptor(
            "frame lies outside its owned chunk",
        ));
    }
    Ok((id as usize, base as usize + offset, d.Length as usize))
}
/// Borrowed RX descriptors. Dropping releases all reserved packets, even after an error or panic.
pub struct RxBatch<'a> {
    socket: &'a mut Socket,
    start: u32,
    count: u32,
}
impl RxBatch<'_> {
    pub fn len(&self) -> usize {
        self.count as usize
    }
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }
    pub fn frame(&self, index: usize) -> Option<RxFrame<'_>> {
        if index >= self.count as usize {
            return None;
        }
        let r = self.socket.rx.as_ref().unwrap();
        let pos = self.start.wrapping_add(index as u32);
        let d: sys::XSK_BUFFER_DESCRIPTOR = unsafe { r.read(pos) };
        // All descriptors were validated before constructing this batch. The driver no longer owns them.
        let (_, offset, len) =
            validate_descriptor(&self.socket.config, &d).expect("validated descriptor");
        Some(RxFrame {
            bytes: unsafe { std::slice::from_raw_parts(self.socket.umem.ptr.add(offset), len) },
            metadata: unsafe { self.socket.rx_metadata(pos) },
        })
    }
    pub fn iter(&self) -> impl Iterator<Item = RxFrame<'_>> {
        (0..self.len()).map(|i| self.frame(i).unwrap())
    }
}
impl Drop for RxBatch<'_> {
    fn drop(&mut self) {
        let r = self.socket.rx.as_mut().unwrap();
        for j in 0..self.count {
            let d: sys::XSK_BUFFER_DESCRIPTOR = unsafe { r.read(self.start.wrapping_add(j)) };
            let base = unsafe { d.Address.AddressAndOffset } & ((1u64 << 48) - 1);
            let id = (base / self.socket.config.frame_size as u64) as usize;
            self.socket.states[id] = State::FreeRx;
            self.socket.rx_free.push(id as u32);
        }
        r.advance(false, self.count);
        if self.socket.refill().is_err() {
            self.socket.poisoned = true;
        }
    }
}
pub struct RxFrame<'a> {
    bytes: &'a [u8],
    metadata: PacketMetadata,
}
impl RxFrame<'_> {
    pub fn bytes(&self) -> &[u8] {
        self.bytes
    }
    pub fn metadata(&self) -> PacketMetadata {
        self.metadata
    }
}
/// TX frames submitted inside the callback are published together when the batch drops.
pub struct TxBatch<'a> {
    socket: &'a mut Socket,
    start: u32,
    count: u32,
    submitted: u32,
}
impl<'b> TxBatch<'b> {
    pub fn capacity(&self) -> usize {
        (self.count - self.submitted) as usize
    }
    pub fn next_frame(&mut self) -> Result<Option<TxFrame<'_, 'b>>> {
        if self.submitted == self.count {
            return Ok(None);
        }
        let Some(id) = self.socket.tx_free.pop() else {
            return Ok(None);
        };
        self.socket.states[id as usize] = State::ReservedTx;
        let pos = self.start.wrapping_add(self.submitted);
        unsafe {
            ptr::write_bytes(
                self.socket.tx.as_ref().unwrap().element(pos),
                0,
                self.socket.tx.as_ref().unwrap().info.ElementStride as usize,
            );
        }
        // Zero recycled bytes: a caller may submit without writing every byte.
        unsafe {
            ptr::write_bytes(
                self.socket
                    .umem
                    .ptr
                    .add(id as usize * self.socket.config.frame_size as usize),
                0,
                self.socket.config.frame_size as usize,
            );
        }
        Ok(Some(TxFrame {
            batch: self,
            id,
            submitted: false,
        }))
    }
}
impl Drop for TxBatch<'_> {
    fn drop(&mut self) {
        self.socket
            .tx
            .as_mut()
            .unwrap()
            .advance(true, self.submitted);
    }
}
pub struct TxFrame<'a, 'b> {
    batch: &'a mut TxBatch<'b>,
    id: u32,
    submitted: bool,
}
impl TxFrame<'_, '_> {
    pub fn bytes_mut(&mut self) -> &mut [u8] {
        let s = &mut self.batch.socket;
        let off = self.id as usize * s.config.frame_size as usize + s.config.headroom as usize;
        unsafe {
            std::slice::from_raw_parts_mut(
                s.umem.ptr.add(off),
                (s.config.frame_size - s.config.headroom) as usize,
            )
        }
    }
    pub fn submit(mut self, length: usize) -> Result<()> {
        let s = &mut self.batch.socket;
        if length == 0 || length > (s.config.frame_size - s.config.headroom) as usize {
            return Err(Error::InvalidConfiguration(
                "TX length must fit one nonempty frame",
            ));
        }
        if let (Some(l), Some(c)) = (s.extensions.tx_layout, s.extensions.tx_checksum) {
            let p = unsafe {
                s.tx.as_ref()
                    .unwrap()
                    .element(self.batch.start.wrapping_add(self.batch.submitted))
            };
            let layout =
                unsafe { ptr::read_unaligned(p.add(l as usize).cast::<sys::XDP_FRAME_LAYOUT>()) };
            let checksum =
                unsafe { ptr::read_unaligned(p.add(c as usize).cast::<sys::XDP_FRAME_CHECKSUM>()) };
            let headers = layout.Layer2HeaderLength() as usize
                + layout.Layer3HeaderLength() as usize
                + layout.Layer4HeaderLength() as usize;
            if headers > length
                || layout.Layer2Type() > 2
                || layout.Layer3Type() > 6
                || layout.Layer4Type() > 4
                || layout.Reserved0() != 0
                || checksum.Layer3() > 1
                || checksum.Layer4() > 1
                || checksum.Reserved() != 0
            {
                return Err(Error::InvalidConfiguration(
                    "invalid TX offload metadata or header lengths",
                ));
            }
        }
        let d = sys::XSK_BUFFER_DESCRIPTOR {
            Address: sys::XSK_BUFFER_ADDRESS {
                AddressAndOffset: (self.id as u64 * s.config.frame_size as u64)
                    | ((s.config.headroom as u64) << 48),
            },
            Length: length as u32,
            Reserved: 0,
        };
        unsafe {
            s.tx.as_mut()
                .unwrap()
                .write(self.batch.start.wrapping_add(self.batch.submitted), d);
        }
        s.states[self.id as usize] = State::DriverTx;
        self.batch.submitted += 1;
        self.submitted = true;
        Ok(())
    }
}
impl Drop for TxFrame<'_, '_> {
    fn drop(&mut self) {
        if !self.submitted {
            self.batch.socket.states[self.id as usize] = State::FreeTx;
            self.batch.socket.tx_free.push(self.id);
        }
    }
}
impl Socket {
    unsafe fn query_extensions(&mut self) -> Result<()> {
        fn query<T>(h: HANDLE, opt: u32, r: &Ring) -> Result<Option<u16>> {
            let off: u16 = unsafe { get(h, opt) }?;
            if (off as usize)
                < if opt == sys::XSK_SOCKOPT_TX_FRAME_TIMESTAMP_EXTENSION {
                    8
                } else {
                    size_of::<sys::XSK_FRAME_DESCRIPTOR>()
                }
                || off as usize + size_of::<T>() > r.info.ElementStride as usize
            {
                return Err(Error::CorruptDescriptor(
                    "extension exceeds descriptor stride",
                ));
            }
            Ok(Some(off))
        }
        if let Some(r) = &self.rx {
            if self.config.rx_checksum {
                self.extensions.rx_checksum = query::<sys::XDP_FRAME_CHECKSUM>(
                    self.handle.0,
                    sys::XSK_SOCKOPT_RX_FRAME_CHECKSUM_EXTENSION,
                    r,
                )?;
                self.extensions.rx_layout = query::<sys::XDP_FRAME_LAYOUT>(
                    self.handle.0,
                    sys::XSK_SOCKOPT_RX_FRAME_LAYOUT_EXTENSION,
                    r,
                )?;
            }
            if self.config.rx_timestamp {
                self.extensions.rx_timestamp = query::<sys::XDP_FRAME_TIMESTAMP>(
                    self.handle.0,
                    sys::XSK_SOCKOPT_RX_FRAME_TIMESTAMP_EXTENSION,
                    r,
                )?;
            }
            if self.config.original_length {
                self.extensions.original_length = query::<sys::XSK_FRAME_ORIGINAL_LENGTH>(
                    self.handle.0,
                    sys::XSK_SOCKOPT_RX_FRAME_ORIGINAL_LENGTH_EXTENSION,
                    r,
                )?;
            }
        }
        if let Some(r) = &self.tx {
            if self.config.tx_checksum {
                self.extensions.tx_checksum = query::<sys::XDP_FRAME_CHECKSUM>(
                    self.handle.0,
                    sys::XSK_SOCKOPT_TX_FRAME_CHECKSUM_EXTENSION,
                    r,
                )?;
                self.extensions.tx_layout = query::<sys::XDP_FRAME_LAYOUT>(
                    self.handle.0,
                    sys::XSK_SOCKOPT_TX_FRAME_LAYOUT_EXTENSION,
                    r,
                )?;
            }
            if self.config.tx_timestamp {
                self.extensions.tx_timestamp = query::<sys::XDP_FRAME_TIMESTAMP>(
                    self.handle.0,
                    sys::XSK_SOCKOPT_TX_FRAME_TIMESTAMP_EXTENSION,
                    self.completion.as_ref().unwrap(),
                )?;
            }
        }
        Ok(())
    }
    unsafe fn rx_metadata(&self, index: u32) -> PacketMetadata {
        unsafe fn read<T: Copy>(r: &Ring, index: u32, offset: Option<u16>) -> Option<T> {
            offset.map(|o| unsafe { ptr::read_unaligned(r.element(index).add(o as usize).cast()) })
        }
        let r = self.rx.as_ref().unwrap();
        let e = self.extensions;
        unsafe {
            PacketMetadata {
                original_length: read::<sys::XSK_FRAME_ORIGINAL_LENGTH>(
                    r,
                    index,
                    e.original_length,
                )
                .map(|v| v.OriginalLength),
                timestamp: read::<sys::XDP_FRAME_TIMESTAMP>(r, index, e.rx_timestamp)
                    .map(|v| v.Timestamp),
                checksum: read(r, index, e.rx_checksum),
                layout: read(r, index, e.rx_layout),
            }
        }
    }
    /// Ring flags retain unknown bits. No direct ownership transfer is performed.
    pub fn ring_flags(&self) -> [Option<u32>; 4] {
        [&self.fill, &self.rx, &self.tx, &self.completion].map(|r| r.as_ref().map(Ring::flags))
    }
    /// Advanced control must preserve every pool, ring and descriptor ownership invariant.
    /// # Safety
    /// Do not use this mapping concurrently with the safe API. Restore all producer,
    /// consumer and pool ownership before invoking any safe method. Never unmap it.
    pub unsafe fn raw_ring_info(&self) -> sys::XSK_RING_INFO_SET {
        sys::XSK_RING_INFO_SET {
            Fill: self.fill.as_ref().map_or_else(Default::default, |r| r.info),
            Rx: self.rx.as_ref().map_or_else(Default::default, |r| r.info),
            Tx: self.tx.as_ref().map_or_else(Default::default, |r| r.info),
            Completion: self
                .completion
                .as_ref()
                .map_or_else(Default::default, |r| r.info),
        }
    }
}
impl TxFrame<'_, '_> {
    /// Request verified upstream checksum offload; header lengths/types must match the packet.
    pub fn checksum(
        &mut self,
        layout: sys::XDP_FRAME_LAYOUT,
        checksum: sys::XDP_FRAME_CHECKSUM,
    ) -> Result<()> {
        let s = &mut self.batch.socket;
        let (Some(l), Some(c)) = (s.extensions.tx_layout, s.extensions.tx_checksum) else {
            return Err(Error::UnsupportedFeature {
                context: "TX checksum was not enabled",
                code: 0,
            });
        };
        unsafe {
            let p =
                s.tx.as_ref()
                    .unwrap()
                    .element(self.batch.start.wrapping_add(self.batch.submitted));
            ptr::write_unaligned(p.add(l as usize).cast(), layout);
            ptr::write_unaligned(p.add(c as usize).cast(), checksum);
        }
        Ok(())
    }
}
#[cfg(test)]
mod tests;

impl ReceivedInfo {
    /// None means original-length reporting was explicitly disabled.
    pub fn truncated(&self) -> Option<bool> {
        self.metadata
            .original_length
            .map(|n| n as usize > self.length)
    }
}
impl RxFrame<'_> {
    pub fn truncated(&self) -> Option<bool> {
        self.metadata
            .original_length
            .map(|n| n as usize > self.bytes.len())
    }
}
