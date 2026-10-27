//! Optional adapter for caller-provided, verified native eBPF artifacts.
//! Requires eBPF for Windows v1.4.0 compatible ebpfapi.dll and XDP provider.
//! Only owned links are destroyed. No interface-wide detach API is called.
use crate::{Error, Result, Socket};
use std::{
    ffi::{CStr, CString, c_char, c_void},
    path::Path,
    ptr,
    rc::Rc,
};
type Obj = *mut c_void;
type Open = unsafe extern "C" fn(*const c_char) -> Obj;
type Load = unsafe extern "C" fn(Obj) -> i32;
type Close = unsafe extern "C" fn(Obj);
type FindMap = unsafe extern "C" fn(Obj, *const c_char) -> i32;
type FindProgram = unsafe extern "C" fn(Obj, *const c_char) -> Obj;
type Update = unsafe extern "C" fn(i32, *const c_void, *const c_void, u64) -> i32;
type MapSize = unsafe extern "C" fn(Obj) -> u32;
type MapType = unsafe extern "C" fn(Obj) -> i32;
type Delete = unsafe extern "C" fn(i32, *const c_void) -> i32;
type Attach = unsafe extern "C" fn(Obj, i32) -> Obj;
type Destroy = unsafe extern "C" fn(Obj) -> i32;
type GetError = unsafe extern "C" fn(*const c_void) -> i32;
struct Api {
    open: Open,
    load: Load,
    close: Close,
    find_map: FindMap,
    find_map_ptr: FindProgram,
    key_size: MapSize,
    value_size: MapSize,
    map_type: MapType,
    find_program: FindProgram,
    update: Update,
    delete: Delete,
    attach: Attach,
    destroy: Destroy,
    error: GetError,
    _library: Option<libloading::Library>,
}
/// Holds the explicitly selected trusted eBPF runtime DLL loaded for this adapter.
pub struct EbpfRuntime {
    api: Rc<Api>,
}
impl EbpfRuntime {
    /// # Safety
    /// The absolute path must name a trusted eBPF for Windows v1.4 compatible DLL.
    /// Loading a DLL executes its initialization code in the caller's process.
    pub unsafe fn load(path: &Path) -> Result<Self> {
        if !path.is_absolute() {
            return Err(Error::InvalidConfiguration(
                "eBPF DLL path must be absolute",
            ));
        }
        let library =
            unsafe { libloading::Library::new(path) }.map_err(|_| Error::UnsupportedFeature {
                context: "load trusted ebpfapi.dll",
                code: 0,
            })?;
        macro_rules! symbol {
            ($name:literal,$ty:ty) => {
                *unsafe { library.get::<$ty>(concat!($name, "\0").as_bytes()) }.map_err(|_| {
                    Error::UnsupportedFeature {
                        context: $name,
                        code: 0,
                    }
                })?
            };
        }
        let api = Api {
            open: symbol!("bpf_object__open", Open),
            load: symbol!("bpf_object__load", Load),
            close: symbol!("bpf_object__close", Close),
            find_map: symbol!("bpf_object__find_map_fd_by_name", FindMap),
            find_map_ptr: symbol!("bpf_object__find_map_by_name", FindProgram),
            key_size: symbol!("bpf_map__key_size", MapSize),
            value_size: symbol!("bpf_map__value_size", MapSize),
            map_type: symbol!("bpf_map__type", MapType),
            find_program: symbol!("bpf_object__find_program_by_name", FindProgram),
            update: symbol!("bpf_map_update_elem", Update),
            delete: symbol!("bpf_map_delete_elem", Delete),
            attach: symbol!("bpf_program__attach_xdp", Attach),
            destroy: symbol!("bpf_link__destroy", Destroy),
            error: symbol!("libbpf_get_error", GetError),
            _library: Some(library),
        };
        Ok(Self { api: Rc::new(api) })
    }
}
fn check(code: i32, context: &'static str) -> Result<()> {
    if code == 0 {
        Ok(())
    } else {
        Err(Error::Io { code, context })
    }
}
struct Program {
    api: Rc<Api>,
    object: Obj,
    link: Obj,
    map: i32,
    keys: Vec<u64>,
}
impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            if !self.link.is_null() {
                (self.api.destroy)(self.link);
            }
            for key in &self.keys {
                (self.api.delete)(self.map, (key as *const u64).cast());
            }
            if !self.object.is_null() {
                (self.api.close)(self.object);
            }
        }
    }
}
/// Owns the program, its XSKMAP entries, and sockets. Program drops before sockets.
pub struct Steering {
    program: Program,
    sockets: Vec<Socket>,
}
impl Steering {
    /// Load an independently verified signed .sys program. The program must redirect
    /// only intended test traffic, pass unmatched packets, and expose `xsk_map` with
    /// u64 queue keys and HANDLE values. This adapter does not verify program policy.
    pub fn redirect_selected(
        runtime: &EbpfRuntime,
        mut sockets: Vec<Socket>,
        artifact: &Path,
        program_name: &CStr,
    ) -> Result<Self> {
        let first = sockets.first().ok_or(Error::InvalidConfiguration(
            "at least one RX socket is required",
        ))?;
        let index = first.configuration().interface;
        if index > i32::MAX as u32
            || sockets
                .iter()
                .any(|s| !s.configuration().receive || s.configuration().interface != index)
        {
            return Err(Error::InvalidConfiguration(
                "RX sockets must share one valid interface",
            ));
        }
        for (i, s) in sockets.iter().enumerate() {
            if sockets[..i]
                .iter()
                .any(|p| p.configuration().queue == s.configuration().queue)
            {
                return Err(Error::InvalidConfiguration("duplicate queue in XSKMAP"));
            }
        }
        if !artifact.is_absolute()
            || artifact
                .extension()
                .is_none_or(|e| !e.eq_ignore_ascii_case("sys"))
        {
            return Err(Error::InvalidConfiguration(
                "provide an absolute path to a verified native .sys artifact",
            ));
        }
        let path = CString::new(
            artifact
                .to_str()
                .ok_or(Error::InvalidConfiguration("artifact path must be UTF-8"))?,
        )
        .map_err(|_| Error::InvalidConfiguration("path contains NUL"))?;
        let api = runtime.api.clone();
        let object = unsafe { (api.open)(path.as_ptr()) };
        if object.is_null() {
            return Err(Error::Io {
                code: unsafe { (api.error)(object) },
                context: "bpf_object__open",
            });
        }
        let mut p = Program {
            api,
            object,
            link: ptr::null_mut(),
            map: -1,
            keys: Vec::with_capacity(sockets.len()),
        };
        unsafe {
            check((p.api.load)(object), "bpf_object__load")?;
            let map = (p.api.find_map_ptr)(object, c"xsk_map".as_ptr());
            if map.is_null()
                || (p.api.map_type)(map) != 16
                || (p.api.key_size)(map) != 8
                || (p.api.value_size)(map) != std::mem::size_of::<crate::sys::HANDLE>() as u32
            {
                return Err(Error::InvalidConfiguration(
                    "xsk_map must be XSKMAP with u64 keys and HANDLE values",
                ));
            }
            p.map = (p.api.find_map)(object, c"xsk_map".as_ptr());
            if p.map < 0 {
                return Err(Error::Io {
                    code: p.map,
                    context: "find xsk_map",
                });
            }
            for s in &sockets {
                let key = s.configuration().queue.0 as u64;
                let handle = s.raw_handle();
                check(
                    (p.api.update)(
                        p.map,
                        (&key as *const u64).cast(),
                        (&handle as *const crate::sys::HANDLE).cast(),
                        0,
                    ),
                    "bpf_map_update_elem(BPF_ANY)",
                )?;
                p.keys.push(key);
            }
            let program = (p.api.find_program)(object, program_name.as_ptr());
            if program.is_null() {
                return Err(Error::Io {
                    code: (p.api.error)(program),
                    context: "find eBPF program",
                });
            }
            p.link = (p.api.attach)(program, index as i32);
            if p.link.is_null() {
                return Err(Error::Io {
                    code: (p.api.error)(p.link),
                    context: "bpf_program__attach_xdp",
                });
            }
        }
        for s in &mut sockets {
            s.acknowledge_external_steering();
        }
        Ok(Self {
            program: p,
            sockets,
        })
    }
    pub fn socket_count(&self) -> usize {
        self.sockets.len()
    }
    /// A restricted borrow prevents replacing a socket while its map entry exists.
    pub fn socket(&mut self, index: usize) -> Option<SteeredSocket<'_>> {
        self.sockets
            .get_mut(index)
            .map(|socket| SteeredSocket { socket })
    }
    /// Explicitly detach the owned link; object/map and socket cleanup follows on drop.
    pub fn detach(mut self) -> Result<()> {
        // Upstream destroys the link even when detach reports an error.
        let link = std::mem::replace(&mut self.program.link, ptr::null_mut());
        let code = unsafe { (self.program.api.destroy)(link) };
        check(code, "bpf_link__destroy")
    }
}

/// A packet-I/O borrow that cannot move a socket out of its steering owner.
pub struct SteeredSocket<'a> {
    socket: &'a mut Socket,
}
impl SteeredSocket<'_> {
    pub fn send(&mut self, frame: &[u8]) -> Result<usize> {
        self.socket.send(frame)
    }
    pub fn recv_into(&mut self, out: &mut [u8]) -> Result<crate::ReceivedInfo> {
        self.socket.recv_into(out)
    }
    pub fn flush_tx(&mut self) -> Result<()> {
        self.socket.flush_tx()
    }
    pub fn drain_completions(&mut self) -> Result<usize> {
        self.socket.drain_completions()
    }
    pub fn stats(&self) -> Result<crate::SocketStats> {
        self.socket.stats()
    }
    pub fn wait_rx(&mut self, timeout: std::time::Duration) -> Result<crate::NotifyResult> {
        self.socket.wait_rx(timeout)
    }
    /// Borrows the steering-owned socket until overlapped completion/cancellation.
    #[cfg(feature = "async")]
    pub fn notify_async(&mut self, flags: u32) -> Result<crate::Notification<'_>> {
        self.socket.notify_async(flags)
    }
    pub fn with_rx_batch<T>(
        &mut self,
        max: u32,
        f: impl FnOnce(&crate::RxBatch<'_>) -> Result<T>,
    ) -> Result<T> {
        self.socket.with_rx_batch(max, f)
    }
    pub fn with_tx_batch<T>(
        &mut self,
        max: u32,
        f: impl FnOnce(&mut crate::TxBatch<'_>) -> Result<T>,
    ) -> Result<T> {
        self.socket.with_tx_batch(max, f)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    thread_local! {static EVENTS:RefCell<Vec<&'static str>>=const {RefCell::new(Vec::new())};}
    unsafe extern "C" fn open(_: *const c_char) -> Obj {
        ptr::null_mut()
    }
    unsafe extern "C" fn load(_: Obj) -> i32 {
        0
    }
    unsafe extern "C" fn close(_: Obj) {
        EVENTS.with(|e| e.borrow_mut().push("close"));
    }
    unsafe extern "C" fn find_map(_: Obj, _: *const c_char) -> i32 {
        1
    }
    unsafe extern "C" fn find(_: Obj, _: *const c_char) -> Obj {
        ptr::null_mut()
    }
    unsafe extern "C" fn update(_: i32, _: *const c_void, _: *const c_void, _: u64) -> i32 {
        0
    }
    unsafe extern "C" fn delete(_: i32, _: *const c_void) -> i32 {
        EVENTS.with(|e| e.borrow_mut().push("delete"));
        0
    }
    unsafe extern "C" fn attach(_: Obj, _: i32) -> Obj {
        ptr::null_mut()
    }
    unsafe extern "C" fn destroy(_: Obj) -> i32 {
        EVENTS.with(|e| e.borrow_mut().push("destroy"));
        -1
    }
    unsafe extern "C" fn error(_: *const c_void) -> i32 {
        -1
    }
    unsafe extern "C" fn size(_: Obj) -> u32 {
        8
    }
    unsafe extern "C" fn typ(_: Obj) -> i32 {
        16
    }
    fn program() -> Program {
        EVENTS.with(|e| e.borrow_mut().clear());
        let api = Api {
            open,
            load,
            close,
            find_map,
            find_map_ptr: find,
            key_size: size,
            value_size: size,
            map_type: typ,
            find_program: find,
            update,
            delete,
            attach,
            destroy,
            error,
            _library: None,
        };
        Program {
            api: Rc::new(api),
            object: ptr::dangling_mut::<c_void>(),
            link: ptr::dangling_mut::<c_void>(),
            map: 1,
            keys: vec![0, 1],
        }
    }
    #[test]
    fn owned_objects_close_in_order() {
        drop(program());
        EVENTS.with(|e| assert_eq!(*e.borrow(), ["destroy", "delete", "delete", "close"]));
    }
    #[test]
    fn detach_error_does_not_destroy_link_twice() {
        let steering = Steering {
            program: program(),
            sockets: Vec::new(),
        };
        assert!(steering.detach().is_err());
        EVENTS.with(|e| assert_eq!(*e.borrow(), ["destroy", "delete", "delete", "close"]));
    }
}
