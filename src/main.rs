#![no_std]
#![no_main]
#![feature(decl_macro, panic_info_message)]
#![allow(unused, dead_code)]
#![warn(clippy::pedantic, clippy::nursery)]
#![deny(rust_2018_idioms, unsafe_op_in_unsafe_fn)]

use core::{arch::global_asm, panic::PanicInfo};
use printer::println;
use sbi::system_reset::{ResetReason, ResetType};
use uart::init_uart;

mod arch;
mod logger;
mod mem;
mod printer;
mod sync;
mod uart;

global_asm!(include_str!("../boot.s"));

fn shutdown() -> ! {
    let _ = sbi::system_reset::system_reset(ResetType::Shutdown, ResetReason::NoReason);
    unreachable!("System reset failed");
}

fn kmain() -> ! {
    init_uart();
    logger::init();

    println!("Hello, World");

    shutdown()
}

#[panic_handler]
fn panic(info: &PanicInfo<'_>) -> ! {
    log::error!("{}", info.message().unwrap());

    shutdown()
}
