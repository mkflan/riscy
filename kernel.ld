OUTPUT_ARCH(riscv64imac)
ENTRY(_start)

KERNEL_START = 0x80200000;

SECTIONS
{
	. = KERNEL_START;

	. = ALIGN(4K);
	.text : {
		PROVIDE(_text_start = .);
		*(.init)
		*(.text .text.*)
		. = ALIGN(4K);
		PROVIDE(_text_end = .);
	}

	. = ALIGN(4K);
	.rodata : {
		*(.srodata .srodata.*)
		*(.rodata .rodata.*)
	}

	. = ALIGN(4K);
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
	PROVIDE(_stack_start = _bss_end);
	PROVIDE(_stack_end = _stack_start + 0x1000);
	PROVIDE(KERNEL_END = _stack_end);
}
