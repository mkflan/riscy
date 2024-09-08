use super::PageFrameAllocator;
use crate::mem::{paging::PageFrame, KERNEL_END};

/// A bitmap-based physical memory manager.
pub struct BitmapPMM {
    /// The bitmap itself, composed of 64
    bitmap: *mut u64,

    /// Size of the bitmap.
    size: usize,
}

impl BitmapPMM {
    const CHUNK_SIZE_BITS: u64 = 64;

    /// Create an uninitialized instance of the allocator.
    pub const fn uninit() -> Self {
        Self {
            bitmap: core::ptr::null_mut(),
            size: 0,
        }
    }

    /// Convert the bitmap pointer to a slice.
    fn as_slice(&mut self) -> &mut [u64] {
        // SAFETY: the passed arguments uphold the safety contract of the function.
        unsafe { core::slice::from_raw_parts_mut(self.bitmap, self.size) }
    }
}

impl PageFrameAllocator for BitmapPMM {
    unsafe fn init(&mut self, start: *mut u8, end: *mut u8) {
        let mem_size_pages = (end as usize - start as usize) / 4096;
        let bitmap_size = mem_size_pages / Self::CHUNK_SIZE_BITS as usize + 1;
        self.bitmap = start.cast();
        self.size = bitmap_size;
    }

    #[track_caller]
    unsafe fn alloc_frame(&mut self) -> Option<PageFrame> {
        if let Some((idx, chunk)) = self
            .as_slice()
            .iter_mut()
            .enumerate()
            .find(|(_, chunk)| **chunk != u64::MAX)
        {
            let next_free_bit = idx * 64 + chunk.trailing_ones() as usize;
            *chunk |= 1 << next_free_bit;
            let pa = (next_free_bit * 4096) + (core::ptr::addr_of!(KERNEL_END) as usize);
            return Some(PageFrame::new(pa as u64));
        }

        None
    }

    #[track_caller]
    unsafe fn dealloc_frame(&mut self, frame: PageFrame) {
        let pa = frame.base_addr();
        let loc = pa.addr() / 4096;
        let chunk = loc / Self::CHUNK_SIZE_BITS;
        let bit = loc % Self::CHUNK_SIZE_BITS;
        self.as_slice()[chunk as usize] &= !(1 << bit);
    }

    #[track_caller]
    fn is_frame_used(&mut self, frame: PageFrame) -> bool {
        let pa = frame.base_addr();
        let loc = pa.addr() / 4096;
        let chunk = loc / Self::CHUNK_SIZE_BITS;
        let bit = loc % Self::CHUNK_SIZE_BITS;
        (self.as_slice()[chunk as usize] & (1 << bit)) != 0
    }
}

unsafe impl Send for BitmapPMM {}
unsafe impl Sync for BitmapPMM {}
