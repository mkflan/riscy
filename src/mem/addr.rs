use super::paging::VPN_CNT;
use core::fmt::{self, UpperHex};

/// A valid RV64 physical address.
///
/// Physical addresses in RV64 only use 55 bits. Thus, this type will ensure the remaining bits are set to zero.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PhysAddr(u64);

impl PhysAddr {
    /// Create a new physical address, with all bits after bit 55 set to 0.
    pub const fn new(addr: u64) -> Self {
        Self(addr % (1 << 55))
    }

    /// Return the physical address as a u64.
    pub const fn addr(self) -> u64 {
        self.0
    }

    /// Check if the physical address is null.
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }

    /// Return the PPN of this physical address.
    pub const fn ppn(self) -> u64 {
        self.0 >> 12
    }
}

impl UpperHex for PhysAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#X}", self.0)
    }
}

/// A valid RV64 virtual address.
///
/// Based on the paging mode being used, a different amount of lower bits contribute to the virtual address while the remaining
/// higher order bits become copies of the highest bit that contributes to the virtual address. This type will ensure these
/// invariants are upheld to prevent unwanted page faults.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct VirtAddr(u64);

impl VirtAddr {
    /// Create a new virtual address, with proper sign extension.
    pub const fn new(addr: u64) -> Self {
        Self(((addr << 24) as i64 >> 24) as u64)
    }

    /// Return the virtual address as a u64.
    pub const fn addr(self) -> u64 {
        self.0
    }

    /// Check if the virtual address is null.
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }

    /// Return the page offset of this virtual address.
    pub const fn offset(self) -> u64 {
        self.0 & 0xFFF
    }

    /// Return the desired virtual page number.
    pub fn vpn(self, idx: u8) -> usize {
        assert!(idx >= 0 && idx < VPN_CNT as u8);

        ((self.0 >> (12 + 9 * idx)) & 0x1FF) as usize
    }
}

impl UpperHex for VirtAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#X}", self.0)
    }
}
