#![no_std]
#![no_main]
#![feature(decl_macro, alloc_error_handler)]
#![allow(unused, unused_comparisons, dead_code)]
#![warn(clippy::pedantic, clippy::nursery)]
#![deny(rust_2018_idioms, unsafe_op_in_unsafe_fn)]

#[cfg(not(target_pointer_width = "64"))]
compile_error!("riscy can only run on a 64-bit system.");

// extern crate alloc;

mod arch;
mod logger;
mod mem;
mod printer;
mod sync;
mod trap;
mod uart;

use core::{arch::global_asm, panic::PanicInfo};
use printer::println;
use sbi::system_reset::{ResetReason, ResetType};
use uart::init_uart;

global_asm!(include_str!("asm/boot.s"));

#[no_mangle]
pub extern "C" fn kmain(hart_id: usize, fdt: *const u8) -> ! {
    // arch::w_stvec(trap::trap_handler as usize);

    init_uart();
    logger::init();
    unsafe { mem::pmm::init() };
    mem::paging::init();

    loop {
        unsafe {
            riscv::asm::wfi();
        }
    }
}

#[panic_handler]
fn panic(info: &PanicInfo<'_>) -> ! {
    let loc = info.location().unwrap();
    let file = loc.file();
    let file = file.strip_prefix("src/").unwrap_or(file);
    let line = loc.line();

    log::error!("(originated from: {}:{}) {}", file, line, info.message());

    loop {
        unsafe {
            riscv::asm::wfi();
        }
    }
}
