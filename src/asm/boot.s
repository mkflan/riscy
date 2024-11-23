.section .init
.globl _start

_start:
        csrw sie, zero
        
        .option push
        .option norelax
                la gp, __global_pointer$
        .option pop

        lla t0, _bss_start
        lla t1, _bss_end

        clear_bss:
                beq t0, t1, post_clear_bss
                sd zero, (t0)
                addi t0, t0, 8
                j clear_bss

        post_clear_bss:

        csrw satp, zero
        lla sp, _stack_end

        j kmain
