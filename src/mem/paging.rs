use crate::arch::r_satp;
use bitflags::{bitflags, Flags};

/// Return the address of the upper most page table level.
fn upmost_page_table() -> usize {
    (r_satp() & 0xFFF) << 12
}

bitflags! {
    pub struct PTEFlags: usize {
        const VALID = 1 << 0;
        const READ = 1 << 1;
        const WRITE = 1 << 2;
        const EXECUTE = 1 << 3;
        const USER = 1 << 4;
    }
}

pub struct PageTableEntry(pub usize);

impl PageTableEntry {
    #[inline]
    pub fn new(addr: usize) -> Self {
        Self(addr)
    }

    /// Return the flags of this entry.
    #[inline]
    pub fn flags(self) -> PTEFlags {
        PTEFlags::from_bits_truncate(self.0)
    }
}

#[repr(C, align(4096))]
pub struct PageTable(pub [PageTableEntry; 512]);
