use bitflags::{bitflags, Flags};

bitflags! {
    pub struct PTEFlags: usize {
        const VALID = 1 << 0;
        const READ = 1 << 1;
        const WRITE = 1 << 2;
        const EXECUTE = 1 << 3;
        const USER = 1 << 4;
    }
}

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
pub struct PageTable(pub [PageTableEntry; 512]);
