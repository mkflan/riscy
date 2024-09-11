use super::PageFrameAllocator;
use crate::mem::paging::PageFrame;

/// A stack-based physical memory manager.
pub struct StackPMM {
    /// The start address of the available memory region.
    avail_mem_start: usize,

    /// The end address of the available memory region.
    avail_mem_end: usize,
}

impl StackPMM {
    /// Create an uninitialized instance of the allocator.
    pub const fn uninit() -> Self {
        Self {
            avail_mem_start: 0,
            avail_mem_end: 0,
        }
    }
}

impl PageFrameAllocator for StackPMM {
    unsafe fn init(&mut self, start: *mut u8, end: *mut u8) {
        todo!();
    }

    fn mark_frame(&mut self, frame: PageFrame, allocated: bool) {
        todo!();
    }

    unsafe fn alloc_frame(&mut self) -> Option<PageFrame> {
        todo!();
    }

    unsafe fn dealloc_frame(&mut self, frame: PageFrame) {
        todo!();
    }

    fn is_frame_used(&mut self, frame: PageFrame) -> bool {
        todo!();
    }
}
