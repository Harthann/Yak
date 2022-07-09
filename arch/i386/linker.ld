ENTRY(_start)

KERNEL_BASE = 0xc0000000;

SECTIONS {
	. = 0x800;
	.gdt :
	{
		*(.gdt)
	}

	. = 0x100000;
	.boot BLOCK(4k) : ALIGN(4k)
	{
		*(.multiboot_header)
		*(.boot)
	}

	. += KERNEL_BASE;
	.text ALIGN(0x1000) : AT(ADDR(.text) - KERNEL_BASE)
	{
		*(.text*)
	}

	.rodata ALIGN(0x1000) : AT(ADDR(.rodata) - KERNEL_BASE)
	{
		*(.rodata*)
	}

	.data ALIGN(0x1000) : AT(ADDR(.data) - KERNEL_BASE)
	{
		*(.padata)
		*(.data*)
	}

	.bss ALIGN(0x1000) : AT(ADDR(.bss) - KERNEL_BASE)
	{
		*(.bss*)
	}

}
