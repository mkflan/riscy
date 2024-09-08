use crate::printer::{print, println};
use log::{Level, LevelFilter, Log, Metadata, Record};

static LOGGER: KernelLogger = KernelLogger;

struct KernelLogger;

impl Log for KernelLogger {
    fn enabled(&self, metadata: &Metadata<'_>) -> bool {
        true
    }

    fn log(&self, record: &Record<'_>) {
        match record.level() {
            Level::Error => print!("\x1b[1;31m[ERROR] "),
            Level::Warn => print!("\x1b[1;33m[WARNING] "),
            Level::Info => print!("\x1b[1;37m[INFO] "),
            Level::Debug => print!("\x1b[1;34m[DEBUG] "),
            Level::Trace => print!("\x1b[1;36m[TRACE] "),
        }

        print!("\x1b[1;0m{}\n", record.args());
    }

    fn flush(&self) {}
}

pub fn init() {
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(LevelFilter::Trace))
        .unwrap()
}
