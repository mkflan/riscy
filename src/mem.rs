pub mod addr;
pub mod paging;
pub mod pmm;

extern "C" {
    pub static mut KERNEL_START: u8;
    /// This holds the first address after the end of the kernel.
    pub static mut KERNEL_END: u8;
}

pub const MEM_END: usize = 0x80000000 + (1024 * 1024 * 128);
