.option norvc

.section .text.init

.global _start
_start:
        # Disable linker relaxation to prevent the assembler from assuming the 
        # global pointer register is already initialized. This causes the value
        # stored to be calculated from the program counter.
.option push
.option norelax
        la gp, __global_pointer$
.option pop
        # Set SATP to zero, even though it should already be zero.
        csrw satp, zero

        # Ensure we are on the first hart.
        csrr t0, mhartid
        bnez t0, 4f

        # Zero out BSS.
        la a0, _bss_start
        la a1, _bss_end
        bgeu a0, a1, 2f
1:
        sd zero, (a0)
        addi a0, a0, 8
        bltu a0, a1, 1b
2:
        # Initialize the stack. The stack grows downwards, so set the stack pointer to the end of the stack.
        la sp, _stack_end

        call kmain 
4:
        wfi
        j 4b
