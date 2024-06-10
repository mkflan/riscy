use crate::sync::spinlock::Spinlock;
use core::fmt::{Result, Write};
use spin::Once;

// The base address of the UART on QEMU's virt machine.
pub const UART_BASE: usize = 0x1000_0000;
pub static UART: Once<Spinlock<Uart>> = Once::new();

pub struct Uart {
    base_addr: usize,
}

impl Uart {
    fn new(base_addr: usize) -> Self {
        Self { base_addr }
    }

    /// Initialize the UART.
    fn init(&mut self) {
        let base_ptr = self.base_addr as *mut u8;

        unsafe {
            // Set the first two bits of the Line Control Register (LCR), which set the word length.
            // This register is at offset 3 from the base address.
            base_ptr.add(3).write_volatile(0b11);

            // Set the first bit of the FIFO Control Register (FCR), which enables FIFO.
            // This register is at offset 2 from the base address.
            base_ptr.add(2).write_volatile(0b1);

            // Set the first bit of the Interrupt Enable Register (IER), which enables interrupts.
            // This register is at offset 1 from the base address.
            base_ptr.add(1).write_volatile(0b1);
        }
    }

    /// Read from the Line Status Register (LSR).
    fn read_lsr(&self) -> u8 {
        let base_ptr = self.base_addr as *mut u8;
        unsafe { base_ptr.add(5).read_volatile() }
    }

    /// Get a character from the UART.
    pub fn get(&self) -> Option<u8> {
        let base_ptr = self.base_addr as *mut u8;
        let lsr = self.read_lsr();

        // No data has been received.
        if lsr & 1 == 0 {
            return None;
        }

        Some(unsafe { base_ptr.add(0).read_volatile() })
    }

    /// Put a character to the UART.
    pub fn put(&mut self, ch: u8) {
        let base_ptr = self.base_addr as *mut u8;
        unsafe { base_ptr.add(0).write_volatile(ch) }
    }
}

impl Write for Uart {
    fn write_str(&mut self, s: &str) -> Result {
        for ch in s.bytes() {
            self.put(ch);
        }

        Ok(())
    }
}

/// Initialize UART.
pub fn init_uart() {
    let mut uart = Uart::new(UART_BASE);
    uart.init();
    UART.call_once(|| Spinlock::new(uart));
}
