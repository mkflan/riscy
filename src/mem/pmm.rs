pub mod bitmap;

use super::paging::PageFrame;
use crate::sync::spinlock::Spinlock;
pub use bitmap::BitmapPMM;

pub static PHYSICAL_MEMORY_MANAGER: Spinlock<BitmapPMM> = Spinlock::new(BitmapPMM::uninit());

pub trait PageFrameAllocator {
    unsafe fn init(&mut self, start: *mut u8, end: *mut u8);
    unsafe fn alloc_frame(&mut self) -> Option<PageFrame>;
    unsafe fn dealloc_frame(&mut self, frame: PageFrame);
    fn is_frame_used(&mut self, frame: PageFrame) -> bool;
}
