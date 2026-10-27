#[cfg(windows)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use std::{mem::zeroed, ptr, time::Duration};
    use windows_sys::Win32::{
        Foundation::{CloseHandle, ERROR_OPERATION_ABORTED, GetLastError},
        System::{
            IO::{CancelIoEx, GetOverlappedResult, OVERLAPPED},
            Threading::{CreateEventW, WaitForSingleObject},
        },
    };
    let a: Vec<_> = std::env::args().collect();
    if a.len() != 3 || a[1] != "--isolated" {
        return Err("usage: notify_async --isolated IFINDEX; demonstrates timed cancellation without injecting traffic".into());
    }
    let runtime = winxdp::Runtime::open()?;
    let socket = winxdp::Socket::builder(&runtime)
        .interface(a[2].parse()?)
        .build()?;
    let event = unsafe { CreateEventW(ptr::null(), 1, 0, ptr::null()) };
    if event.is_null() {
        return Err(std::io::Error::last_os_error().into());
    }
    struct Event(windows_sys::Win32::Foundation::HANDLE);
    impl Drop for Event {
        fn drop(&mut self) {
            unsafe {
                CloseHandle(self.0);
            }
        }
    }
    let event = Event(event);
    let mut overlapped = Box::pin(unsafe { zeroed::<OVERLAPPED>() });
    overlapped.hEvent = event.0;
    // No ring operation occurs concurrently. The socket and pinned OVERLAPPED stay
    // alive until the kernel has acknowledged completion or cancellation.
    let handle = unsafe { socket.raw_handle() };
    let code = unsafe {
        winxdp::sys::XskNotifyAsync(
            handle,
            winxdp::sys::XSK_NOTIFY_FLAG_WAIT_TX,
            &mut *overlapped,
        )
    };
    if code < 0 && code != winxdp::sys::XDP_STATUS_PENDING {
        return Err(winxdp::Error::Io {
            code,
            context: "XskNotifyAsync",
        }
        .into());
    }
    unsafe {
        WaitForSingleObject(event.0, 50);
        CancelIoEx(handle, &*overlapped);
    }
    let mut bytes = 0;
    // This example deliberately waits to observe cancellation. Never free an
    // OVERLAPPED merely because CancelIoEx returned successfully.
    let done = unsafe { GetOverlappedResult(handle, &*overlapped, &mut bytes, 1) };
    if done == 0 {
        let code = unsafe { GetLastError() };
        if code != ERROR_OPERATION_ABORTED {
            return Err(std::io::Error::from_raw_os_error(code as i32).into());
        }
        println!("Notification cancelled and completion observed");
    } else {
        let mut flags = 0;
        let code = unsafe { winxdp::sys::XskGetNotifyAsyncResult(&mut *overlapped, &mut flags) };
        if code < 0 {
            return Err(winxdp::Error::Io {
                code,
                context: "XskGetNotifyAsyncResult",
            }
            .into());
        }
        println!("Notification flags: {flags:#x}");
    }
    socket.close(Duration::ZERO)?;
    Ok(())
}
#[cfg(not(windows))]
fn main() {
    eprintln!("Windows x64 is required");
}
