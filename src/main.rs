#![no_std]
#![no_main]
#![feature(decl_macro)]
#![allow(unused, unused_comparisons, dead_code)]
#![warn(clippy::pedantic, clippy::nursery)]
#![deny(rust_2018_idioms, unsafe_op_in_unsafe_fn)]

#[cfg(not(target_pointer_width = "64"))]
compile_error!("riscy can only run on a 64-bit system.");

mod arch;
mod logger;
mod mem;
mod printer;
mod sync;
mod uart;

use core::{arch::global_asm, panic::PanicInfo};
use mem::{
    paging::PageTable,
    pmm::{BitmapPMM, PageFrameAllocator, PHYSICAL_MEMORY_MANAGER},
    KERNEL_END, KERNEL_START, MEM_END,
};
use printer::println;
use sbi::system_reset::{ResetReason, ResetType};
use uart::init_uart;

global_asm!(include_str!("asm/boot.s"));

fn shutdown() -> ! {
    let _ = sbi::system_reset::system_reset(ResetType::Shutdown, ResetReason::NoReason);
    unreachable!("System reset failed");
}

#[no_mangle]
pub extern "C" fn kmain(hart_id: usize, fdt: *const u8) -> ! {
    init_uart();
    logger::init();

    log::info!("SYSTEM INFO");
    log::info!("  boot hart id: {hart_id}");

    let mem_start = unsafe { core::ptr::addr_of_mut!(KERNEL_END) };
    let mem_end = MEM_END as *mut u8;
    let mut pmm = PHYSICAL_MEMORY_MANAGER.acquire();

    unsafe {
        pmm.init(mem_start, mem_end);
    }

    let frame = unsafe { pmm.alloc_frame().unwrap() };

    // TODO: Setup page table mappings
    // - identity mapping for UART
    // - identity mapping for kernel
    // - higher half mapping for stack
    let mut root_pt = PageTable::new();

    shutdown()
}

#[panic_handler]
fn panic(info: &PanicInfo<'_>) -> ! {
    let loc = info.location().unwrap();
    let file = loc.file();
    let file = file.strip_prefix("src/").unwrap_or(file);
    let line = loc.line();

    log::error!("(originated from: {}:{}) {}", file, line, info.message());

    shutdown()
}
