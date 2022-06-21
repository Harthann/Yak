ENTRY(_start)

SECTIONS {
	. = 0x0;
	.boot :
	{ *(.multiboot_header) }

	. = 0x800;
	.gdt :
	{ *(.gdt) }

	.text :
	{ *(.text) }
}
