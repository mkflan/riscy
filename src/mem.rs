pub mod addr;
pub mod paging;
pub mod pmm;

use core::alloc::Layout;

extern "C" {
    static KERNEL_START: usize;
    static KERNEL_END: usize;
    static _text_start: usize;
    static _text_end: usize;
    static _stack_start: usize;
    static _stack_end: usize;
}

pub const MEM_END: usize = 0x8000_0000 + (1024 * 1024 * 128);

/// Align an address to an upper bound based on the specified order.
pub const fn align_up(addr: usize, order: usize) -> usize {
    let mask = (1usize << order) - 1;
    (addr + mask) & !mask
}

pub const fn align_down(addr: usize, order: usize) -> usize {
    addr & !((1usize << order) - 1)
}

#[alloc_error_handler]
fn alloc_error(layout: Layout) -> ! {
    panic!("memory allocation of {} bytes failed", layout.size());
}
