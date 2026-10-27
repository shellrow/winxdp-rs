use crate::{Error, Result, sys};
use std::{
    ptr,
    sync::atomic::{AtomicU32, Ordering},
};

/// An SPSC endpoint. Construction is restricted to driver-owned mappings.
pub(crate) struct Ring {
    pub info: sys::XSK_RING_INFO,
}
impl Ring {
    pub unsafe fn new(info: sys::XSK_RING_INFO, minimum: usize) -> Result<Self> {
        if info.Ring.is_null()
            || !info.Size.is_power_of_two()
            || info.Size > 1 << 30
            || (info.ElementStride as usize) < minimum
        {
            return Err(Error::CorruptDescriptor("invalid ring layout"));
        }
        for off in [
            info.ProducerIndexOffset,
            info.ConsumerIndexOffset,
            info.FlagsOffset,
        ] {
            if (info.Ring as usize)
                .checked_add(off as usize)
                .is_none_or(|p| p % 4 != 0)
            {
                return Err(Error::CorruptDescriptor("unaligned ring index"));
            }
        }
        let end = (info.DescriptorsOffset as usize)
            .checked_add(info.Size as usize * info.ElementStride as usize);
        if end
            .and_then(|e| (info.Ring as usize).checked_add(e))
            .is_none()
        {
            return Err(Error::CorruptDescriptor("ring overflow"));
        }
        Ok(Self { info })
    }
    fn atomic(&self, off: u32) -> &AtomicU32 {
        // SAFETY: new validates alignment; driver owns the live mapping until socket close.
        unsafe { &*self.info.Ring.add(off as usize).cast::<AtomicU32>() }
    }
    pub fn flags(&self) -> u32 {
        self.atomic(self.info.FlagsOffset).load(Ordering::Acquire)
    }
    pub fn reserve(&self, producer: bool, max: u32) -> Result<(u32, u32)> {
        if self.flags() & sys::XSK_RING_FLAG_ERROR as u32 != 0 {
            return Err(Error::RingError {
                flags: self.flags(),
            });
        }
        let p = self
            .atomic(self.info.ProducerIndexOffset)
            .load(Ordering::Acquire);
        let c = self
            .atomic(self.info.ConsumerIndexOffset)
            .load(Ordering::Acquire);
        let used = p.wrapping_sub(c);
        if used > self.info.Size {
            return Err(Error::CorruptDescriptor("ring occupancy exceeds capacity"));
        }
        Ok((
            if producer { p } else { c },
            max.min(if producer {
                self.info.Size - used
            } else {
                used
            }),
        ))
    }
    pub fn advance(&mut self, producer: bool, count: u32) {
        let a = self.atomic(if producer {
            self.info.ProducerIndexOffset
        } else {
            self.info.ConsumerIndexOffset
        });
        a.store(
            a.load(Ordering::Relaxed).wrapping_add(count),
            Ordering::Release,
        );
    }
    pub unsafe fn element(&self, index: u32) -> *mut u8 {
        unsafe {
            self.info.Ring.add(
                self.info.DescriptorsOffset as usize
                    + (index & (self.info.Size - 1)) as usize * self.info.ElementStride as usize,
            )
        }
    }
    pub unsafe fn read<T: Copy>(&self, index: u32) -> T {
        unsafe { ptr::read_unaligned(self.element(index).cast()) }
    }
    pub unsafe fn write<T>(&mut self, index: u32, value: T) {
        unsafe {
            ptr::write_unaligned(self.element(index).cast(), value);
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn wraparound_and_pressure() {
        let mut memory = vec![0u64; 16];
        let info = sys::XSK_RING_INFO {
            Ring: memory.as_mut_ptr().cast(),
            ProducerIndexOffset: 0,
            ConsumerIndexOffset: 4,
            FlagsOffset: 8,
            DescriptorsOffset: 16,
            Size: 4,
            ElementStride: 8,
            Reserved: 0,
        };
        let mut r = unsafe { Ring::new(info, 8).unwrap() };
        r.atomic(0).store(u32::MAX - 1, Ordering::Relaxed);
        r.atomic(4).store(u32::MAX - 1, Ordering::Relaxed);
        assert_eq!(r.reserve(true, 9).unwrap(), (u32::MAX - 1, 4));
        r.advance(true, 4);
        assert_eq!(r.reserve(true, 1).unwrap().1, 0);
        assert_eq!(r.reserve(false, 9).unwrap().1, 4);
        r.advance(false, 3);
        assert_eq!(r.reserve(true, 9).unwrap().1, 3);
        r.advance(true, 4);
        assert!(r.reserve(false, 1).is_err());
    }
}
