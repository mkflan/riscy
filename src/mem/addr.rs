use super::paging::{HIGHEST_PPN_LEN, PN_CNT};
use core::{
    fmt::{self, UpperHex},
    ops::{Add, AddAssign},
};

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PhysAddr(usize);

impl PhysAddr {
    pub const fn new(addr: usize) -> Self {
        Self(addr)
    }

    /// Return the inner physical address.
    pub const fn addr(self) -> usize {
        self.0
    }

    pub fn from_ptr(ptr: *mut u8) -> Self {
        Self(ptr as usize)
    }

    pub const fn as_mut_ptr(self) -> *mut u8 {
        self.0 as *mut u8
    }

    /// Return the full PPN of this physical address.
    pub fn ppn(self) -> usize {
        self.0 >> 12
    }

    /// Return all the PPNs of this physical address.
    pub fn ppns(self) -> [usize; PN_CNT] {
        core::array::from_fn(|idx| {
            let ppn = self.0 >> (12 + 9 * idx);

            if idx == PN_CNT - 1 {
                ppn & HIGHEST_PPN_LEN
            } else {
                ppn & 0x1FF
            }
        })
    }
}

impl Add<usize> for PhysAddr {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        Self::new(self.addr() + rhs)
    }
}

impl AddAssign<usize> for PhysAddr {
    fn add_assign(&mut self, rhs: usize) {
        self.0 += rhs;
    }
}

impl UpperHex for PhysAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#X}", self.0)
    }
}

/// A valid RV64 virtual address.
///
/// Bits 39-63 of the virtual address must be equal to bit 38 to prevent a page fault.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct VirtAddr(usize);

impl VirtAddr {
    /// Create a valid virtual address.
    pub const fn new(mut addr: usize) -> Self {
        // let top_bit_mask = 1 << (12 + PN_CNT * 9 - 1);
        // let top_bit = addr & top_bit_mask;

        // match top_bit {
        //     0 => addr &= !(usize::MAX << (12 + PN_CNT * 9 - 1)),
        //     1 => addr |= (usize::MAX << (12 + PN_CNT * 9 - 1)),
        //     _ => unreachable!(),
        // }

        Self(addr)
    }

    /// Return the inner virtual address.
    pub const fn addr(self) -> usize {
        self.0
    }

    /// Return the page offset of this virtual address.
    pub const fn offset(self) -> usize {
        self.0 & 0xFFF
    }

    /// Return all the VPNs of this virtual address.
    pub fn vpns(self) -> [usize; PN_CNT] {
        core::array::from_fn(|idx| (self.0 >> (12 + 9 * idx)) & 0x1FF)
    }
}

impl Add<usize> for VirtAddr {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        Self::new(self.addr() + rhs)
    }
}

impl AddAssign<usize> for VirtAddr {
    fn add_assign(&mut self, rhs: usize) {
        self.0 += rhs;
    }
}

impl UpperHex for VirtAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#X}", self.0)
    }
}
