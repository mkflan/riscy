use super::PageFrameAllocator;
use crate::mem::paging::PageFrame;

/// A bitmap-based physical memory manager.
pub struct BitmapPMM {
    bitmap: *mut u64,

    /// Size of the bitmap.
    size: usize,

    /// The start address of the available memory region.
    avail_mem_start: usize,

    /// The end address of the available memory region.
    avail_mem_end: usize,
}

impl BitmapPMM {
    const BITS_PER_CHUNK: usize = 64;

    /// Create an uninitialized instance of the allocator.
    pub const fn uninit() -> Self {
        Self {
            bitmap: core::ptr::null_mut(),
            size: 0,
            avail_mem_start: 0,
            avail_mem_end: 0,
        }
    }

    /// Convert the bitmap pointer to a slice.
    fn as_slice(&mut self) -> &'static mut [u64] {
        // SAFETY: the passed arguments uphold the safety contract of the function.
        // TODO: convert bitmap ptr addr to virtual address so it works when paging is enabled.
        unsafe { core::slice::from_raw_parts_mut(self.bitmap, self.size) }
    }

    /// Return the row and column a frame is located within the bitmap.
    fn frame_loc(&self, frame: PageFrame) -> (usize, usize) {
        let loc = (frame.base_addr().addr() as usize - self.avail_mem_start) / 4096;
        (loc / Self::BITS_PER_CHUNK, loc % Self::BITS_PER_CHUNK)
    }
}

impl PageFrameAllocator for BitmapPMM {
    unsafe fn init(&mut self, start: *mut u8, end: *mut u8) {
        let mem_size_pages = (end as usize - start as usize) / 4096;
        let bitmap_size = mem_size_pages / Self::BITS_PER_CHUNK;
        self.bitmap = start.cast();
        self.size = bitmap_size;
        self.avail_mem_start = start as usize;
        self.avail_mem_end = end as usize;

        self.as_slice().fill_with(|| 0);

        // Mark the pages used by the bitmap as used.
        for page in 0..(self.size / 4096 + 1) {
            let frame = PageFrame::new((self.avail_mem_start + 4096 * page));
            self.mark_frame(frame, true);
        }
    }

    fn mark_frame(&mut self, frame: PageFrame, allocated: bool) {
        let (chunk, bit) = self.frame_loc(frame);
        let bitmap = self.as_slice();

        if allocated {
            bitmap[chunk] |= 1 << bit;
        } else {
            bitmap[chunk] &= !(1 << bit);
        }
    }

    unsafe fn alloc_frame(&mut self) -> Option<PageFrame> {
        if let Some((idx, chunk)) = self
            .as_slice()
            .iter_mut()
            .enumerate()
            .find(|(_, chunk)| **chunk != u64::MAX)
        {
            let next_free_bit = chunk.trailing_ones() as usize;
            let pa = (next_free_bit * 4096) + self.avail_mem_start;

            // Ensure the physical address is a valid address to prevent allocating an unusable frame.
            if pa <= self.avail_mem_end {
                let frame = PageFrame::new(pa);
                self.mark_frame(frame, true);
                return Some(frame);
            }
        }

        None
    }

    unsafe fn dealloc_frame(&mut self, frame: PageFrame) {
        self.mark_frame(frame, false);
    }

    fn is_frame_used(&mut self, frame: PageFrame) -> bool {
        let (chunk, bit) = self.frame_loc(frame);
        (self.as_slice()[chunk] & (1 << bit)) != 0
    }
}

unsafe impl Send for BitmapPMM {}
unsafe impl Sync for BitmapPMM {}
