OUTPUT_ARCH(riscv64imac)
ENTRY(_start)

KERNEL_START = 0x80200000;
STACK_SIZE = 4K;

SECTIONS
{
	. = KERNEL_START;

	. = ALIGN(4K);
	.text : {
		*(.init)
		*(.text .text.*)
	}

	. = ALIGN(4K);
	.rodata : {
		*(.srodata .srodata.*)
		*(.rodata .rodata.*)
	}

	.data : {
		*(.sdata .sdata.*) *(.data .data.*)
		. = ALIGN(8);
		PROVIDE(__global_pointer$ = . + 0x800);
	}

	. = ALIGN(4K);
	.bss (NOLOAD) : {
		PROVIDE(_bss_start = .);
		*(.sbss .sbss.*)
		*(.bss .bss.*)
		. = ALIGN(4K);
		PROVIDE(_bss_end = .);
	}

	. = ALIGN(4K);
	.stack (NOLOAD) : {
		PROVIDE(_stack_bottom = .);
		. = ALIGN(4K);
		. += STACK_SIZE;
		. = ALIGN(4K);
		PROVIDE(_stack_top = .);
	}

	. = ALIGN(4K);
	PROVIDE(KERNEL_END = .);
}
