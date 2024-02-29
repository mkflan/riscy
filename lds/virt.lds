OUTPUT_ARCH("riscv")

/* 
	Set the entry point to the _start symbol. It will be placed at the address 0x8000_0000
	as this is where memory for QEMU's virt machine begins.
*/
ENTRY(_start)

MEMORY
{
	/*
		Create a "ram" section that is 128 megabytes long starting at address 0x8000_0000.
		This section will be writable (w), executable (x), and allocatable (a).
	*/
	ram (wxa) : ORIGIN = 0x80000000, LENGTH = 128M
}

/*
	Declare program headers for the text, data, and bss sections.

	The text section holds executable code. The data section holds global, initialized variables.
	The bss section holds global, uninitialized variables.

	PT_LOAD tells the linker that the sections will be loaded from the file into memory.
*/
PHDRS
{
	text PT_LOAD;
	data PT_LOAD;
	bss PT_LOAD;
}

SECTIONS
{
	.text : {
		PROVIDE(_text_start = .);

		/*
			Layout all text sections. Starting with .text.init, this will map the .text.init section of ANY object file,
			which is indicated by the leading asterisk. After .text.init, all other .text and .text.* sections will be laid out.
		*/
		*(.text.init) *(.text .text.*)

		PROVIDE(_text_end = .);

		/*
			This final portion, >ram AT>ram :text, tells the linker what portion of memory to place this section. Let's break it down:
			- >ram tells the linker to put the entire .text section into the "ram" memory region.
			- AT>ram sets the "load memory address" (LMA) region to the "ram" memory region. This loads everything into its physical location.
			- :text tells the linker to put this into the :text program header.
		*/
	} >ram AT>ram :text

	/*
		The global pointer allows the linker to position global variables and constants into places relative to the global pointer register (gp).
		These globals start after the text section.
	*/
	PROVIDE(__global_pointer$ = .);

	.rodata : {
		PROVIDE(_rodata_start = .);
		*(.rodata .rodata.*)
		PROVIDE(_rodata_end = .);
	} >ram AT>ram :text

	.data : {
		/*
			The line below tells the linker to allign the current memory location to 4096 bytes (4 KiB), 
			which is the system's page size. 
		*/
		. = ALIGN(4096);
		PROVIDE(_data_start = .);
		*(.sdata .sdata.*) *(.data .data.*)
		PROVIDE(_data_end = .);
	} >ram AT>ram :data

	.bss : {
		PROVIDE(_bss_start = .);
		*(.sbss .sbss.*) *(.bss .bss.*)
		PROVIDE(_bss_end = .);
	} >ram AT>ram :bss

	PROVIDE(_memory_start = ORIGIN(ram));
	PROVIDE(_memory_end = _memory_start + LENGTH(ram));
	PROVIDE(_stack_start = _bss_end);
	PROVIDE(_stack_end = _stack_start + 0x80000);
	PROVIDE(_heap_start = _stack_end);
	PROVIDE(_heap_size = _memory_end - _heap_start);
}
