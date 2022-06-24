ENTRY(_start)

SECTIONS {

	. = 0x800;
	.gdt :
	{
		*(.gdt)
	}

	.text BLOCK(4k) : ALIGN(4k)
	{ 
		*(.multiboot_header)
		*(.text)
	}

	.rodata BLOCK(4k) : ALIGN(4k)
	{
		*(.rodata);
	}

	.data BLOCK(4k) : ALIGN(4k)
	{
		*(.data);
	}


	.bss BLOCK(4k) : ALIGN(4k)
	{
		*(.bss)
	}

}
