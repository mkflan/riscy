OUTPUT_ARCH(riscv)

/* Define the entry point. */
ENTRY(_start)

KERNEL_PHYS_START = 0x80200000;
KERNEL_OFFSET = 0xFFFFFFFF80000000;

SECTIONS
{
	/* Set the location counter to the physical start address of the kernel. */
	. = KERNEL_PHYS_START;

	/* The location counter represents the VMA and is currently 0x80200000. We want the LMA
		 to equal the VMA here so the bootstrapping code will start at physical address 0x80200000, 
		 which is where OpenSBI will jump to and start executing code at. Based on its heuristics, the linker 
		 will determine that the LMA should equal the VMA, which is what we want.

		 See https://sourceware.org/binutils/docs/ld/Output-Section-LMA.html for more info. */
	.init : {
		*(.init)
	}

	/* Add the offset our kernel will live at in virtual memory. */
	. += KERNEL_OFFSET;

	KERNEL_VIRT_START = .;

	.text : AT(ADDR(.text) - KERNEL_OFFSET) {
		*(.text .text.*)
	}

	.rodata ALIGN(4) : AT(ADDR(.rodata) - KERNEL_OFFSET) {
		*(.srodata .srodata.*)
		*(.rodata .rodata.*)

		/* 4-byte align the end (VMA) of the section to ensure
			 the LMA of .data is correctly aligned. */
		. = ALIGN(4);
	}

	.data ALIGN(8) : AT(ADDR(.data) - KERNEL_OFFSET) {
		*(.sdata .sdata.*) *(.data .data.*)
		. = ALIGN(8);
		PROVIDE(__global_pointer$ = . + 0x800);
	}

	.bss ALIGN(8) (NOLOAD) : AT(ADDR(.bss) - KERNEL_OFFSET) {
		__bss_start = .;
		*(.sbss .sbss.* .bss .bss.*)
		. = ALIGN(8);
		__bss_end = .;
	}
} 
