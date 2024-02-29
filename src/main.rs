#![no_std]
#![no_main]
#![feature(decl_macro, panic_info_message)]
#![allow(unused, dead_code)]

use core::{
    arch::{asm, global_asm},
    panic::PanicInfo,
};
use printer::println;
use uart::init_uart;

mod printer;
mod uart;

global_asm!(include_str!("../asm/boot.S"));

#[no_mangle]
fn kmain() -> ! {
    init_uart();

    println!("Hello, World");

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(loc) = info.location() {
        println!(
            "[{}:{}:{}] {}",
            loc.file(),
            loc.line(),
            loc.column(),
            info.message().unwrap()
        );
    } else {
        println!("A panic occurred!");
    }

    loop {
        unsafe { asm!("wfi") }
    }
}
