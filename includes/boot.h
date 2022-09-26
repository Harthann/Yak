%ifndef BOOT_H
%define BOOT_H

; multiboot header
%define MULTIBOOT_MAGIC		0xe85250d6

%define KERNEL_BASE			0xc0000000

; gdt
extern gdt_desc

%macro reload_segments 0
	mov ax, 0x10
	mov ds, ax
	mov es, ax
	mov fs, ax
	mov gs, ax
	mov ss, ax
%endmacro

; paging
%macro setup_page_table 2
	mov eax, 0x0
	mov ebx, %2
	%%.fill_table:
		mov ecx, ebx
		or ecx, 3
		mov [%1-KERNEL_BASE+eax*4], ecx
		add ebx, 0x1000
		inc eax
		cmp eax, 1024
		je %%.end
		jmp %%.fill_table
	%%.end:
%endmacro

%macro enable_paging 1
	mov eax, %1 - KERNEL_BASE
	mov cr3, eax
	mov eax, cr0
	or eax, 0x80010000
	mov cr0, eax
%endmacro

%endif
