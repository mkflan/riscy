pub mod bitmap;
pub mod stack;

use super::paging::PageFrame;
use crate::sync::spinlock::Spinlock;
pub use bitmap::BitmapPMM;
pub use stack::StackPMM;

pub static PHYSICAL_MEMORY_MANAGER: Spinlock<BitmapPMM> = Spinlock::new(BitmapPMM::uninit());

pub trait PageFrameAllocator {
    /// Initialize an instance of the allocator.
    ///
    /// # Safety
    /// Both `start` and `end` must be valid physical addresses.
    #[track_caller]
    unsafe fn init(&mut self, start: *mut u8, end: *mut u8);

    /// Mark a frame based on the given status.
    fn mark_frame(&mut self, frame: PageFrame, allocated: bool);

    /// Attempt to allocate a page frame.
    ///
    /// # Safety
    /// This method returns unique frames and must not return a previously allocated frame
    /// if the frame hasn't been deallocated before the call to this method.
    #[track_caller]
    unsafe fn alloc_frame(&mut self) -> Option<PageFrame>;

    /// Deallocate the given page frame.
    ///
    /// # Safety
    /// The given frame must be unused, with no references remaining to it.
    #[track_caller]
    unsafe fn dealloc_frame(&mut self, frame: PageFrame);

    fn is_frame_used(&mut self, frame: PageFrame) -> bool;
}
