use core::fmt::{Arguments, Write};

struct Printer;

fn sbi_print(s: &str) {
    s.chars()
        .for_each(|c| sbi::legacy::console_putchar(c.try_into().unwrap_or(b'?')));
}

impl Write for Printer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        sbi_print(s);
        Ok(())
    }
}

pub fn print_args(args: Arguments) {
    let mut printer = Printer;
    printer.write_fmt(args).unwrap();
}

pub macro print($($args:tt),+) {
    $crate::printer::print_args(format_args!($($args),+));
}

pub macro println {
    ($($args:tt),+) => {$crate::printer::print!($($args),+) },
    () => { $crate::printer::print!("\n") }
}
