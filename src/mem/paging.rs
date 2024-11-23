use super::{
    addr::{PhysAddr, VirtAddr},
    pmm,
};
use crate::mem::align_down;
use crate::mem::{
    _stack_end, _stack_start, _text_end, _text_start, KERNEL_END, KERNEL_START, MEM_END,
};
use core::ops::{BitAnd, BitOr, Index, IndexMut};

/// Constant representing the amount of physical/virtual page numbers used in the enabled MMU mode.
pub(super) const PN_CNT: usize = 3;

/// Constant representing the length, in bits, of the upmost PPN.
pub(super) const HIGHEST_PPN_LEN: usize = 26;

pub const SATP_MODE: usize = 8;

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PageTableEntryFlags(u8);

impl PageTableEntryFlags {
    pub const VALID: Self = Self(1 << 0);
    pub const READ: Self = Self(1 << 1);
    pub const WRITE: Self = Self(1 << 2);
    pub const EXEC: Self = Self(1 << 3);
    pub const USER: Self = Self(1 << 4);
    pub const GLOBAL: Self = Self(1 << 5);
    pub const ACCESSED: Self = Self(1 << 6);
    pub const DIRTY: Self = Self(1 << 7);

    pub const fn flags(self) -> u8 {
        self.0
    }
}

impl BitOr<PageTableEntryFlags> for PageTableEntryFlags {
    type Output = Self;

    fn bitor(self, rhs: PageTableEntryFlags) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitAnd<PageTableEntryFlags> for PageTableEntryFlags {
    type Output = Self;

    fn bitand(self, rhs: PageTableEntryFlags) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct PageTableEntry(u64);

impl PageTableEntry {
    const UNUSED: Self = Self(0);

    /// Check if this entry has the given flag(s) set.
    pub fn has_flags_set(self, flags: PageTableEntryFlags) -> bool {
        self.0 as u8 & flags.flags() == 1
    }

    /// Check if this entry is valid.
    pub fn is_valid(self) -> bool {
        self.has_flags_set(PageTableEntryFlags::VALID)
    }

    /// Check if this entry is a leaf.
    pub fn is_leaf(self) -> bool {
        self.0 & 0xE != 0
    }

    /// Set this entry's flags.
    pub fn set_flags(&mut self, flags: PageTableEntryFlags) {
        let entry = self.0 & !(0xFF);
        self.0 = entry | flags.flags() as u64;
    }

    /// Set this entry's PPN to the given PPN.
    pub fn set_ppn(&mut self, ppn: usize) {
        let addr = (ppn << 10) as u64;
        let flags = self.0 & 0x3FF;
        self.0 = addr | flags;
    }

    /// Convert this page table entry to a valid physical address.
    pub fn as_phys_addr(self) -> PhysAddr {
        PhysAddr::new(((self.0 >> 10) << 12) as usize)
    }

    /// Retrieve the flags of this PTE.
    pub fn flags(self) -> PageTableEntryFlags {
        PageTableEntryFlags(self.0 as u8)
    }
}

#[repr(C, align(4096))]
#[derive(Debug, Clone)]
pub struct PageTable {
    pub entries: [PageTableEntry; 512],
}

impl Index<usize> for PageTable {
    type Output = PageTableEntry;

    fn index(&self, index: usize) -> &Self::Output {
        &self.entries[index]
    }
}

impl IndexMut<usize> for PageTable {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.entries[index]
    }
}

impl PageTable {
    /// Create a new page table of unused entries.
    pub const fn new() -> Self {
        Self {
            entries: [PageTableEntry::UNUSED; 512],
        }
    }

    pub fn base_addr(&self) -> usize {
        self.entries.as_ptr() as usize
    }

    pub fn walk(&self, va: VirtAddr) -> Option<PageTableEntry> {
        let vpns = va.vpns();
        let mut pt = &*self;
        let mut pte = &self.entries[vpns[2]];

        for level in (0..PN_CNT - 1).rev() {
            if pte.is_valid() {
                pt = unsafe {
                    pte.as_phys_addr()
                        .as_mut_ptr()
                        .cast::<PageTable>()
                        .as_ref()?
                };
            }

            // Allocate a new page table.
            let new_pt = pmm::zalloc();
            let mut new_pte = PageTableEntry::UNUSED;
            new_pte.set_ppn(new_pt.base_addr().ppn());
            new_pte.set_flags(PageTableEntryFlags::VALID);
            pte = unsafe { (&new_pte as *const PageTableEntry).as_ref()? };
            pt = unsafe { new_pt.as_mut_ptr().cast::<PageTable>().as_ref()? };
        }

        Some(pt[vpns[0]])
    }

    /// Create a page table mapping for the given virtual address to the given physical address using a 4 KiB page.
    /// TODO: add support for creating non-4 KiB sized mappings.
    #[track_caller]
    pub fn map(&mut self, va: VirtAddr, pa: PhysAddr, flags: PageTableEntryFlags) {
        // Assuming Sv39 paging for now.
        // assert!(flags.flags() & 0xE != 0);

        log::info!("Mapping {va:#X} to {pa:#X}");

        let mut pte = self.walk(va).expect("unable to get pte");

        if pte.is_valid() {
            panic!("entry already mapped");
        }

        pte.set_ppn(pa.ppn());
        pte.set_flags(flags | PageTableEntryFlags::VALID);

        // log::info!("{:064b}", pte.0);
        log::info!("mapping complete");

        //     let vpns = va.vpns();
        //     let mut pt = &mut *self;

        //     for level in (0..PN_CNT).rev() {
        //         let mut pte = &mut pt[vpns[level]];

        //         if level == 0 {
        //             pte.set_ppn(pa.ppn());
        //             pte.set_flags(flags | PageTableEntryFlags::VALID);
        //             return;
        //         }

        //         if !pte.is_valid() {
        //             let new_pt = pmm::zalloc();
        //             pte.set_ppn(new_pt.base_addr().ppn());
        //             pte.set_flags(PageTableEntryFlags::VALID);
        //             pt = unsafe { new_pt.as_mut_ptr().cast::<PageTable>().as_mut().unwrap() };
        //         } else {
        //             pt = unsafe {
        //                 pte.as_phys_addr()
        //                     .as_mut_ptr()
        //                     .cast::<PageTable>()
        //                     .as_mut()
        //                     .unwrap()
        //             };
        //         }
        //     }
    }

    /// Identity map contiguous virtual address to contiguous physical addresses using 4 KiB pages.
    pub fn id_map_contiguous(&mut self, va: VirtAddr, size: usize, flags: PageTableEntryFlags) {
        assert!(va.addr() % 4096 == 0, "va is not page-aligned");
        assert!(size != 0, "size must be greater than zero");
        assert!(size % 4096 == 0, "size must be page aligned");

        let mut addr = VirtAddr::new(align_down(va.addr(), 12));
        let num_pages = size / 4096 - 1;

        for _ in 0..num_pages {
            self.map(addr, PhysAddr::new(addr.addr()), flags);
            addr += 4096;
        }
    }

    fn walk_dbg(&self) {
        for (idx, entry) in self.entries.iter().enumerate() {
            if !entry.is_valid() {
                continue;
            }

            if entry.is_leaf() {
                let pa = entry.as_phys_addr();
                let flags = entry.flags();

                crate::printer::println!("entry {idx}: pa {pa:#X} - flags {:#b}", flags.flags());
            } else {
                let pt = unsafe { &*entry.as_phys_addr().as_mut_ptr().cast::<PageTable>() };
                pt.walk_dbg();
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PageFrame(PhysAddr);

impl PageFrame {
    /// Create a new page frame, given its base address.
    ///
    /// # Panics
    /// This function panics if it is given a physical address that is not page-aligned.
    #[track_caller]
    pub fn new(base_addr: usize) -> Self {
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

    pub const fn as_mut_ptr(self) -> *mut u8 {
        self.0.as_mut_ptr()
    }
}

pub fn init() {
    let mut root_pt = unsafe {
        pmm::zalloc()
            .as_mut_ptr()
            .cast::<PageTable>()
            .as_mut()
            .unwrap()
    };

    root_pt.map(
        VirtAddr::new(0x1000_0000),
        PhysAddr::new(0x1000_0000),
        PageTableEntryFlags::READ | PageTableEntryFlags::WRITE,
    );

    let kstart = unsafe { &KERNEL_START as *const _ as usize };
    let tstart = unsafe { &_text_start as *const _ as usize };
    let tend = unsafe { &_text_end as *const _ as usize };
    let stackbot = unsafe { &_stack_end as *const _ as usize };

    // Map kernel .text section
    root_pt.id_map_contiguous(
        VirtAddr::new(kstart),
        tend - tstart,
        PageTableEntryFlags::READ | PageTableEntryFlags::EXEC,
    );

    // Map rest of kernel
    root_pt.id_map_contiguous(
        VirtAddr::new(tend),
        MEM_END - tend,
        PageTableEntryFlags::READ | PageTableEntryFlags::WRITE,
    );

    // Map stack
    root_pt.map(
        VirtAddr::new(stackbot),
        PhysAddr::new(MEM_END - 4096),
        PageTableEntryFlags::READ | PageTableEntryFlags::WRITE,
    );

    let pt_ppn = root_pt.base_addr() >> 12;
    let satp = (SATP_MODE << 60) | pt_ppn;
    // crate::arch::sfence_vma();
    crate::arch::satp::w_satp(satp); // instruction page fault here.
                                     // crate::arch::sfence_vma();

    log::info!("hi");
}
