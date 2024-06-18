/* Define the entry point. */
ENTRY(_start)

/* Define symbols to help with putting things where they need to be. */
_KERNEL_BASE = 0x80200000;
_KERNEL_VIRT_BASE = 
_KERNEL_STACK_BASE =
_KERNEL_STACK_LEN = 2M;

SECTIONS
{
	/* Set the location counter to the kernel base address. */
	. = _KERNEL_BASE;

	/* Put the boot code first, responsible for putting our kernel into the higher half,
	   enabling paging, and jumping to Rust. */
	.boot : {
		_boot_start = .;
		*(.boot)
		_boot_end = ABSOLUTE(.);
	}

	.data ALIGN(8) : {
		PROVIDE(__global_pointer$ = . + 0x800);
	}
}
