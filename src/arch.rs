use core::arch::asm;

pub mod satp {
    use core::arch::asm;

    /// Read the SATP register.
    #[inline(always)]
    pub fn r_satp() -> usize {
        let satp;
        unsafe { asm!("csrr {}, satp", out(reg) satp) };
        satp
    }

    /// Write to the SATP register.
    #[inline(always)]
    pub fn w_satp(val: usize) {
        unsafe { asm!("csrw satp, {}", in(reg) val) };
    }
}
/// Write to the STVEC register.
pub fn w_stvec(func: usize) {
    unsafe { asm!("csrw stvec, {}", in(reg) func) }
}

/// Read the SCAUSE register.
pub fn r_scause() -> usize {
    let scause: usize;
    unsafe { asm!("csrr {}, scause", out(reg) scause) };
    scause
}

#[inline(always)]
pub fn sfence_vma() {
    unsafe { asm!("sfence.vma zero, zero") };
}
