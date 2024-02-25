#![no_std]
#![no_main]
#![feature(decl_macro)]

use core::panic::PanicInfo;

mod printer;

#[riscv_rt::entry]
fn kmain() -> ! {
    printer::println!("Hello, World");
    loop {}
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}
