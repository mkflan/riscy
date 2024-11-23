use crate::uart::UART;
use core::fmt::{Arguments, Write};

pub fn print_args(args: Arguments<'_>) {
    let mut uart = UART.acquire();
    uart.write_fmt(args).unwrap();
}

pub macro print($($args:tt)*) {
    $crate::printer::print_args(format_args!($($args)*))
}

pub macro println {
    ($($args:tt)*) => { $crate::printer::print!("{}\n", format_args!($($args)*)) },
    () => { $crate::printer::print!("\n") }
}
