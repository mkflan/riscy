use super::addr::{PhysAddr, VirtAddr};
use crate::arch::r_satp;

pub(super) const VPN_CNT: usize = 3;

/// Return the address of the upper most page table level.
fn upmost_page_table() -> usize {
    (r_satp() & 0xFFF) << 12
}

pub struct PageTableEntryFlags(usize);

impl PageTableEntryFlags {
    pub const VALID: usize = 1 << 0;
    pub const READ: usize = 1 << 1;
    pub const WRITE: usize = 1 << 2;
    pub const EXEC: usize = 1 << 3;
    pub const USER: usize = 1 << 4;
    pub const GLOBAL: usize = 1 << 5;
    pub const ACCESSED: usize = 1 << 6;
    pub const DIRTY: usize = 1 << 7;

    pub const fn leaf() -> Self {
        Self(Self::VALID)
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct PageTableEntry(pub u64);

impl PageTableEntry {
    const UNUSED: Self = Self(0);

    /// Return the flags of this entry.
    pub const fn flags(&self) -> u64 {
        self.0 & 0xFF
    }

    /// Check if this entry has the given flag set.
    pub const fn has_flag_set(self, flag: u64) -> bool {
        (self.flags() & flag) == 1
    }

    /// Set the flags of this entry.
    pub fn set_flags(&mut self, flags: u64) {
        self.0 |= flags;
    }

    /// Set the PPN of this entry.
    pub fn set_ppn(&mut self, pa: PhysAddr) {
        let ppn = pa.ppn();
        let flags = self.flags();
        self.0 = (ppn << 10) | flags;
    }
}

#[repr(C, align(4096))]
pub struct PageTable {
    entries: [PageTableEntry; 512],
}

impl PageTable {
    /// Create a new page table of unused entries.
    pub const fn new() -> Self {
        Self {
            entries: [PageTableEntry::UNUSED; 512],
        }
    }

    /// Create a page table mapping with the given flags at the given level of page table.
    pub fn map(&mut self, va: VirtAddr, pa: PhysAddr, flags: u64, level: u8) {
        // Assuming Sv39 paging for now.
        assert!(level <= 2, "Sv39 only supports 3 levels of page table");

        log::info!("Mapping {va:#?} to {pa:#?}");

        for vpn_idx in (0..VPN_CNT as u8).rev() {
            let vpn = va.vpn(vpn_idx);
            let pte = &mut self.entries[vpn];

            if vpn_idx == level {
                pte.set_flags(flags);
                pte.set_ppn(pa);
            }
        }

        todo!();
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct PageFrame(PhysAddr);

impl PageFrame {
    /// Create a new page frame, given its base address.
    ///
    /// # Panics
    /// This function panics if it is given a physical address that is not page-aligned.
    #[track_caller]
    pub fn new(base_addr: u64) -> Self {
        assert_eq!(
            base_addr % 4096,
            0,
            "attempted to create page frame with unaligned physical address"
        );

        Self(PhysAddr::new(base_addr))
    }

    pub const fn base_addr(self) -> PhysAddr {
        self.0
    }
}
