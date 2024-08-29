.equ PTE_VALID, 1 << 0
.equ PTE_READ, 1 << 1
.equ PTE_WRITE, 1 << 2
.equ PTE_EXEC, 1 << 3
.equ SATP_MODE, 8 /* Sv39 paging */

.section .bss
        .align 4
_stack_end:
        .skip 4096 /* 4 KiB stack */
_stack_start:

/* Allocate space for the page tables. */
        .align 12
_root_pt:
        .skip 4096
_kernel_pt:
        .skip 4096
_stack_pt:
        .skip 4096
_uart_pt:
        .skip 4096

/* This macro allows us to load addresses that are further than 12 bits from the program counter.

   See the "Medium Low Code Model" section on https://github.com/riscv-non-isa/riscv-elf-psabi-doc/blob/master/riscv-elf.adoc for more info. */
.macro LA_FAR, reg, sym
    lui \reg, %hi(\sym)
    addi \reg, \reg, %lo(\sym)
.endm

/* Create a new page table entry.
   - pt is the page table to create the entry in.
   - va is the virtual address to map.
   - pa is the physical address to map to.
   - vpn is the 9-bit VPN index to use.
   - flags is the flags the resulting PTE should have.

   - t0 will hold the virtual address and VPN.
   - t1 will hold the page table to create the entry in.
   - t2 will hold the PTE.
*/
.macro CREATE_PTE, pt, va, pa, vpn, flags
        la t2, \pa                   # Load the physical address.
        srli t2, t2, 12              # Isolate the PPN.
        la t0, \va                   # Load the virtual address.
        srli t0, t0, (12 + 9 * \vpn) # Shift desired 9-bit VPN to rightmost bits.
        andi t0, t0, (1 << 9) - 1    # Extract desired 9-bit VPN.
        slli t0, t0, 3               # Multiply VPN by PTE size (8 bytes) to get the proper offset.
        la t1, \pt                   # Load the base address of the page table that will contain the PTE.
        add t1, t1, t0               # Add the offset.
        slli t2, t2, 10              # Shift PPN over so PTE flags can be added. 
        addi t2, t2, \flags          # Set PTE flags.
        sw t2, 0(t1)                 # Store PTE in page table.
.endm

/* Create a new page table entry.
   - pt is the page table to create the entry in.
   - va is the virtual address to map.
   - pa is the physical address to map to.
   - vpn is the 9-bit VPN index to use.
   - flags is the flags the resulting PTE should have.

   - t0 will hold the virtual address and VPN.
   - t1 will hold the page table to create the entry in.
   - t2 will hold the PTE.
*/
.macro CREATE_PTE_FAR, pt, va, pa, vpn, flags
        la t2, \pa                   # Load the physical address.
        srli t2, t2, 12              # Isolate the PPN.
        LA_FAR t0, \va               # Load the virtual address.
        srli t0, t0, (12 + 9 * \vpn) # Shift desired 9-bit VPN to rightmost bits.
        andi t0, t0, (1 << 9) - 1    # Extract desired 9-bit VPN.
        slli t0, t0, 3               # Multiply VPN by PTE size (8 bytes) to get the proper offset.
        la t1, \pt                   # Load the base address of the page table that will contain the PTE.
        add t1, t1, t0               # Add the offset.
        slli t2, t2, 10              # Shift PPN over so PTE flags can be added. 
        addi t2, t2, \flags          # Set PTE flags.
        sd t2, 0(t1)                 # Store PTE in page table.
.endm

.section .init
.globl _start

_start:
        .option norelax
        .cfi_startproc
        .cfi_undefined ra

        /* Disable interrupts. */
        csrw sie, zero
        csrw sip, zero

        /* Setup our page tables. 
        
           We will create:
           - our root page table
           - kernel boot code identity mapping
           - kernel code mapping
           - 4 KiB identity mapping for the UART controller
        */

        /* Identity map the kernel. */
        CREATE_PTE _root_pt, KERNEL_PHYS_START, _kernel_pt, 2, PTE_VALID                                            # Create a branch entry at level 3 to the level 2 page table.
        CREATE_PTE _kernel_pt, KERNEL_PHYS_START, KERNEL_PHYS_START, 1, PTE_VALID | PTE_READ | PTE_WRITE | PTE_EXEC # Create a level 2 leaf for 2 MiB kernel mapping.

        CREATE_PTE_FAR _root_pt, KERNEL_VIRT_START, _kernel_pt, 2, PTE_VALID
        CREATE_PTE_FAR _kernel_pt, KERNEL_VIRT_START, KERNEL_PHYS_START, 1, PTE_VALID | PTE_READ | PTE_WRITE | PTE_EXEC

        /* Create a 4 KiB mapping for the kernel stack. */
        CREATE_PTE_FAR _kernel_pt, _stack_start, _stack_pt, 1, PTE_VALID
        CREATE_PTE_FAR _stack_pt, _stack_start, _stack_start, 0, PTE_VALID | PTE_READ | PTE_WRITE
        
        CREATE_PTE _kernel_pt, 0x10000000, _uart_pt, 1, PTE_VALID
        CREATE_PTE _uart_pt, 0x10000000, 0x10000000, 0, PTE_VALID | PTE_READ | PTE_WRITE
     
        /* Setup the satp register. */
        li t1, SATP_MODE # Load our paging mode (Sv39). 
        slli t1, t1, 60  # Shift MODE field to where it is expected within the satp register.
        la t0, _root_pt  # Load the address of our root page table.
        srli t0, t0, 12  # Extract the PPN of the root page table.
        or t0, t0, t1    # Include the PPN of the root PT along with the MODE field.
        csrw satp, t0    # Initialize satp.

        LA_FAR gp, __global_pointer$
        LA_FAR sp, _stack_start

        /* Zero registers. */
        li x1, 0
        li x2, 0
        li x3, 0
        li x4, 0
        li x5, 0
        li x6, 0
        li x7, 0
        li x8, 0
        li x9, 0
        li x13, 0
        li x14, 0
        li x15, 0
        li x16, 0
        li x17, 0
        li x18, 0
        li x19, 0
        li x20, 0
        li x21, 0
        li x22, 0
        li x23, 0
        li x24, 0
        li x25, 0
        li x26, 0
        li x27, 0
        li x28, 0
        li x29, 0
        li x30, 0
        li x31, 0

        /* Jump to the kernel entry point in Rust, which is in virtual memory. */
        LA_FAR a2, kmain
        jr a2

        .cfi_endproc


/* setup kernel identity mapping, then unmap it */
