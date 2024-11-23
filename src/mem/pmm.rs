mod bitmap;

use super::{addr::PhysAddr, paging::PageFrame};
use crate::sync::spinlock::Spinlock;
pub use bitmap::BitmapPMM;

// At the moment, the PMM implementation only allocates normal 4 KiB pages.
// TODO: add support for allocation of larger 2 MiB and 1 GiB pages.
// #[global_allocator]
pub static PHYSICAL_MEMORY_MANAGER: PhysicalMemoryManager<BitmapPMM> =
    PhysicalMemoryManager::uninit(BitmapPMM::uninit());

/// Trait representing the core functionality of an allocator that allocates physical page frames.
pub trait PageFrameAllocator {
    /// Initialize an instance of the allocator.
    ///
    /// # Safety
    /// Both `start` and `end` must be valid physical addresses.
    unsafe fn init(&mut self, start: *mut u8, end: *mut u8);

    /// Mark a frame based on the given status.
    fn mark_frame(&mut self, frame: PageFrame, allocated: bool);

    /// Attempt to allocate a page frame.
    ///
    /// # Safety
    /// This method returns unique frames and must not return a previously allocated frame
    /// if the frame hasn't been deallocated before the call to this method.
    unsafe fn alloc_frame(&mut self) -> Option<PageFrame>;

    /// Deallocate the given page frame.
    ///
    /// # Safety
    /// The given frame must be unused, with no references remaining to it.
    unsafe fn dealloc_frame(&mut self, frame: PageFrame);

    fn is_frame_used(&mut self, frame: PageFrame) -> bool;
}

pub struct PhysicalMemoryManager<A: PageFrameAllocator>(Spinlock<A>);

impl<A: PageFrameAllocator> PhysicalMemoryManager<A> {
    const fn uninit(alloc: A) -> Self {
        Self(Spinlock::new(alloc))
    }

    #[track_caller]
    unsafe fn init(&self, start: *mut u8, end: *mut u8) {
        unsafe { self.0.acquire().init(start, end) };
    }

    fn mark_frame(&self, frame: PageFrame, allocated: bool) {
        self.0.acquire().mark_frame(frame, allocated);
    }

    #[track_caller]
    unsafe fn alloc_frame(&self) -> Option<PageFrame> {
        unsafe { self.0.acquire().alloc_frame() }
    }

    #[track_caller]
    unsafe fn dealloc_frame(&self, frame: PageFrame) {
        unsafe { self.0.acquire().dealloc_frame(frame) };
    }

    fn is_frame_used(&self, frame: PageFrame) -> bool {
        self.0.acquire().is_frame_used(frame)
    }
}

/// Allocate a page frame.
pub fn alloc() -> PageFrame {
    unsafe { PHYSICAL_MEMORY_MANAGER.alloc_frame() }.expect("unable to allocate frame")
}

/// Allocate a page frame and zero it out.
pub fn zalloc() -> PageFrame {
    let frame = unsafe { PHYSICAL_MEMORY_MANAGER.alloc_frame() }.expect("unable to allocate frame");
    let frame_ptr = frame.as_mut_ptr().cast::<u64>();

    unsafe {
        for i in 0..(4096 / 8) {
            *frame_ptr.add(i) = 0;
        }
    }

    frame
}

pub fn init() {
    let mem_start = &raw const crate::mem::KERNEL_END as usize;
    let mem_end = crate::mem::MEM_END;

    unsafe { PHYSICAL_MEMORY_MANAGER.init(mem_start as *mut u8, mem_end as *mut u8) }
}
