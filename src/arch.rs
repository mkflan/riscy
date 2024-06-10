use core::arch::asm;

/// Read the SATP register.
pub fn r_satp() -> usize {
    let satp;
    unsafe { asm!("csrr {}, satp", out(reg) satp) };
    satp
}

/// Write to the SATP register.
pub fn w_satp(val: usize) {
    unsafe { asm!("csrw satp, {}", in(reg) val) };
}
