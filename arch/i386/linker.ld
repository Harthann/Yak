ENTRY(_start)

SECTIONS {

	. = 0x800;
	.gdt :
	{
		*(.gdt)
	}

	. = 1M;
	.boot BLOCK(4k) : ALIGN(4k)
	{
		*(.multiboot_header)
		*(.boot)
	}

	. = 0xc0200000;
	.text ALIGN(0x1000) : AT(ADDR(.text) - 0xc0000000)
	{
		*(.text*)
	}

	.rodata ALIGN(0x1000) : AT(ADDR(.rodata) - 0xc0000000)
	{
		*(.rodata*);
	}

	.data ALIGN(0x1000) : AT(ADDR(.data) - 0xc0000000)
	{
		*(.data*);
	}

	.bss ALIGN(0x1000) : AT(ADDR(.bss) - 0xc0000000)
	{
		*(.bss*)
	}

}
