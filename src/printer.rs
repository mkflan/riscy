use crate::uart::UART;
use core::fmt::{Arguments, Write};

struct Printer;

impl Write for Printer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for ch in s.bytes() {
            UART.get().map(|l| l.lock().put(ch)).unwrap()
        }

        Ok(())
    }
}

pub fn print_args(args: Arguments) {
    let mut printer = Printer;
    printer.write_fmt(args).unwrap();
}

pub macro print($($args:tt)*) {
    $crate::printer::print_args(format_args!($($args)*));
}

pub macro println {
    ($($args:tt)*) => { $crate::printer::print!("{}\n", format_args!($($args)*)) },
    () => { $crate::printer::print!("\n") }
}
