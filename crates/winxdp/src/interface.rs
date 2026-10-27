use crate::{Error, Result, error::status, sys};
use std::{mem::size_of, ptr};
use windows_sys::Win32::{
    Foundation::{CloseHandle, HANDLE},
    NetworkManagement::IpHelper::{FreeMibTable, GetIfTable2, MIB_IF_TABLE2},
};
#[derive(Debug, Clone)]
pub struct Interface {
    pub index: u32,
    pub name: String,
    pub description: String,
    pub physical_address: Vec<u8>,
    pub operational: bool,
}
pub(crate) fn interfaces() -> Result<Vec<Interface>> {
    let mut table: *mut MIB_IF_TABLE2 = ptr::null_mut();
    let code = unsafe { GetIfTable2(&mut table) };
    if code != 0 {
        return Err(Error::Io {
            code: code as i32,
            context: "GetIfTable2",
        });
    }
    struct Table(*mut MIB_IF_TABLE2);
    impl Drop for Table {
        fn drop(&mut self) {
            unsafe {
                FreeMibTable(self.0.cast());
            }
        }
    }
    let guard = Table(table);
    let rows = unsafe {
        std::slice::from_raw_parts((*guard.0).Table.as_ptr(), (*guard.0).NumEntries as usize)
    };
    fn wide(s: &[u16]) -> String {
        String::from_utf16_lossy(&s[..s.iter().position(|v| *v == 0).unwrap_or(s.len())])
    }
    Ok(rows
        .iter()
        .map(|r| Interface {
            index: r.InterfaceIndex,
            name: wide(&r.Alias),
            description: wide(&r.Description),
            physical_address: r.PhysicalAddress
                [..(r.PhysicalAddressLength as usize).min(r.PhysicalAddress.len())]
                .to_vec(),
            operational: r.OperStatus == 1,
        })
        .collect())
}
/// An interface handle; RSS configuration is experimental and can affect other traffic.
pub struct InterfaceHandle {
    handle: HANDLE,
}
impl InterfaceHandle {
    pub fn open(index: u32) -> Result<Self> {
        let mut handle = ptr::null_mut();
        status(
            unsafe { sys::XdpInterfaceOpen(index, &mut handle) },
            "XdpInterfaceOpen",
        )?;
        Ok(Self { handle })
    }
    pub fn rss_capabilities(&self) -> Result<sys::XDP_RSS_CAPABILITIES> {
        let mut caps = sys::XDP_RSS_CAPABILITIES::default();
        let mut len = size_of::<sys::XDP_RSS_CAPABILITIES>() as u32;
        status(
            unsafe { sys::XdpRssGetCapabilities(self.handle, &mut caps, &mut len) },
            "XdpRssGetCapabilities",
        )?;
        Ok(caps)
    }
    /// # Safety
    /// Caller must preserve handle ownership and validate variable-length RSS/QEO buffers.
    pub unsafe fn raw_handle(&self) -> HANDLE {
        self.handle
    }
}
impl Drop for InterfaceHandle {
    fn drop(&mut self) {
        unsafe {
            CloseHandle(self.handle);
        }
    }
}

impl Interface {
    /// Enumerate OS interfaces without requiring an installed XDP driver.
    pub fn list() -> Result<Vec<Self>> {
        interfaces()
    }
}
