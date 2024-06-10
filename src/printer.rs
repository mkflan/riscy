use crate::uart::UART;
use core::fmt::{Arguments, Write};

struct Printer;

impl Write for Printer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let mut uart = UART.get().map(|l| l.acquire()).unwrap();

        uart.write_str(s)
    }
}

pub fn print_args(args: Arguments<'_>) {
    let mut printer = Printer;
    printer.write_fmt(args).unwrap();
}

pub macro print($($args:tt)*) {
    $crate::printer::print_args(format_args!($($args)*))
}

pub macro println {
    ($($args:tt)*) => { $crate::printer::print!("{}\n", format_args!($($args)*)) },
    () => { $crate::printer::print!("\n") }
}
