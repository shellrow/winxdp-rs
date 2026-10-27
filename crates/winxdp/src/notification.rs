use crate::{Error, NotifyResult, Result, Socket, error::status, sys};
use std::{mem::zeroed, pin::Pin, ptr, time::Duration};
use windows_sys::Win32::{
    Foundation::{
        CloseHandle, ERROR_OPERATION_ABORTED, GetLastError, HANDLE, WAIT_OBJECT_0, WAIT_TIMEOUT,
    },
    System::{
        IO::{CancelIoEx, GetOverlappedResult, OVERLAPPED},
        Threading::{CreateEventW, WaitForSingleObject},
    },
};

/// An owned overlapped notification borrowing its socket exclusively.
///
/// `wait(Duration::ZERO)` polls without blocking. A timeout leaves the operation
/// pending; `cancel` or Drop waits for cancellation acknowledgement before freeing
/// native storage. Drop can therefore block on the driver's cancellation latency.
/// This Windows wait primitive has no executor dependency and is not a Future.
///
/// ```compile_fail
/// # fn example(s: &mut winxdp::Socket) -> winxdp::Result<()> {
/// let pending = s.notify_async(winxdp::sys::XSK_NOTIFY_FLAG_WAIT_TX as u32)?;
/// s.flush_tx()?;
/// drop(pending);
/// # Ok(()) }
/// ```
pub struct Notification<'a> {
    socket: &'a mut Socket,
    overlapped: Pin<Box<OVERLAPPED>>,
    event: HANDLE,
    pending: bool,
    result: Option<Result<Option<NotifyResult>>>,
}

impl Socket {
    /// Starts one overlapped operation. Only upstream notify flag bits are accepted.
    /// While pending, no other safe operation can access this socket.
    pub fn notify_async(&mut self, flags: u32) -> Result<Notification<'_>> {
        self.healthy()?;
        if flags & !15 != 0 {
            return Err(Error::InvalidConfiguration("unknown notify flags"));
        }
        let event = unsafe { CreateEventW(ptr::null(), 1, 0, ptr::null()) };
        if event.is_null() {
            return Err(last_error("CreateEventW"));
        }
        let mut operation = Notification {
            socket: self,
            overlapped: Box::pin(unsafe { zeroed::<OVERLAPPED>() }),
            event,
            pending: false,
            result: None,
        };
        operation.overlapped.hEvent = event;
        // The heap allocation stays pinned until completion, even if the guard moves.
        let code = unsafe {
            sys::XskNotifyAsync(
                operation.socket.raw_handle(),
                flags as _,
                &mut *operation.overlapped,
            )
        };
        if code != sys::XDP_STATUS_PENDING {
            status(code, "XskNotifyAsync")?;
        }
        operation.pending = true;
        Ok(operation)
    }
}

fn last_error(context: &'static str) -> Error {
    Error::Io {
        code: unsafe { GetLastError() } as i32,
        context,
    }
}

impl Notification<'_> {
    /// Returns None on timeout; the request remains alive and can be waited again.
    pub fn wait(&mut self, timeout: Duration) -> Result<Option<NotifyResult>> {
        if let Some(result) = &self.result {
            return result.clone();
        }
        let milliseconds = timeout.as_millis().min(u32::MAX as u128 - 1) as u32;
        match unsafe { WaitForSingleObject(self.event, milliseconds) } {
            WAIT_TIMEOUT => Ok(None),
            WAIT_OBJECT_0 => self.finish(),
            _ => Err(last_error("WaitForSingleObject")),
        }
    }

    /// Cancels and observes completion. None means cancellation was acknowledged;
    /// Some means the request completed before cancellation won the race.
    pub fn cancel(mut self) -> Result<Option<NotifyResult>> {
        self.cancel_inner()
    }

    fn cancel_inner(&mut self) -> Result<Option<NotifyResult>> {
        if self.pending {
            // ERROR_NOT_FOUND means completion won the race. In either case observe
            // completion before releasing memory; cancellation alone is not enough.
            unsafe {
                CancelIoEx(self.socket.raw_handle(), &*self.overlapped);
            }
            self.finish()
        } else {
            self.result.clone().unwrap_or(Ok(None))
        }
    }

    fn finish(&mut self) -> Result<Option<NotifyResult>> {
        let mut bytes = 0;
        let done = unsafe {
            GetOverlappedResult(self.socket.raw_handle(), &*self.overlapped, &mut bytes, 1)
        };
        // The live handle and private event cannot be closed by safe callers.
        // A blocking GetOverlappedResult has now observed the final I/O status.
        let result = if done == 0 {
            let code = unsafe { GetLastError() };
            if code == ERROR_OPERATION_ABORTED {
                Ok(None)
            } else {
                Err(Error::Io {
                    code: code as i32,
                    context: "GetOverlappedResult",
                })
            }
        } else {
            let mut flags = 0;
            status(
                unsafe { sys::XskGetNotifyAsyncResult(&mut *self.overlapped, &mut flags) },
                "XskGetNotifyAsyncResult",
            )
            .map(|()| Some(NotifyResult(flags as u32)))
        };
        self.pending = false;
        self.result = Some(result.clone());
        result
    }
}

impl Drop for Notification<'_> {
    fn drop(&mut self) {
        let _ = self.cancel_inner();
        unsafe {
            CloseHandle(self.event);
        }
    }
}
