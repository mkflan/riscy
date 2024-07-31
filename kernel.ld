OUTPUT_ARCH(riscv)

/* Define the entry point. */
ENTRY(_start)

MEMORY
{
	/* Specify the memory available to our QEMU machine.
		 Since we are using OpenSBI, our code will start at 0x80200000.
		 From 0x80000000, we have 128 MiB of memory, but 0x80200000 is 2 MiB
		 into that memory. So, we actually only have 126 MiB for the kernel. */
	RAM (rwx) : ORIGIN = 0x80200000, LENGTH = 126M
}

KERNEL_OFFSET = 0xFFFFFFFF80000000;
STACK_START = ORIGIN(RAM) + LENGTH(RAM);

SECTIONS
{
	/* Put the code responsible for putting the kernel into the higher half, enabling paging,
		 and setting up a runtime for Rust first. */
	.init : {
		*(.init)
	} >RAM AT>RAM

	. += KERNEL_OFFSET;

	.text : AT(ADDR(.text) - KERNEL_OFFSET) {
		*(.text .text.*)
	} >RAM

	.rodata ALIGN(4) : AT(ADDR(.rodata) - KERNEL_OFFSET) {
		*(.srodata .srodata.*)
		*(.rodata .rodata.*)

		/* 4-byte align the end of the section to ensure
			 the LMA of .data is correctly aligned. */
		. = ALIGN(4);
	} >RAM

	.data ALIGN(8) : AT(ADDR(.data) - KERNEL_OFFSET) {
		*(.sdata .sdata.*) *(.data .data.*)
		. = ALIGN(8);
		PROVIDE(__global_pointer$ = . + 0x800);
	} >RAM

	.bss ALIGN(8) (NOLOAD) : AT(ADDR(.bss) - KERNEL_OFFSET) {
		__bss_start = .;
		*(.sbss .sbss.* .bss .bss.*)
		. = ALIGN(8);
		__bss_end = .;
	} >RAM
} 
