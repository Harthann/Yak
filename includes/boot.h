%ifndef BOOT_H
%define BOOT_H

; multiboot header
%define MULTIBOOT_MAGIC		0xe85250d6

%define KERNEL_BASE			0xc0000000

; gdt
extern gdt_desc

%macro load_kernel_segments 0
	mov ax, 0x10
	mov ds, ax
	mov es, ax
	mov fs, ax
	mov gs, ax
%endmacro

%endif
