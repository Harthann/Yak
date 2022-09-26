%include "boot.h"

global _start
global stack_bottom
global stack_top
global page_directory
global page_table
global multiboot_ptr

[BITS 32]
section .boot
header_start:
	dd MULTIBOOT_MAGIC ; magic
	dd 0 ; architecture - 0: protected mode
	dd header_end - header_start ; header length
	dd 0x100000000 - (MULTIBOOT_MAGIC + 0 + (header_end - header_start)) ; checksum
	dw 0 ; type
	dw 0 ; flags
	dd 8 ; size
header_end:

_start:
	mov [multiboot_ptr - KERNEL_BASE], ebx
	mov esp, stack_top - KERNEL_BASE ; Stack pointer initialisation
	setup_page_table page_table, 0x0
	enable_paging page_directory

	extern gdt_desc
	lgdt [gdt_desc]

	jmp 0x08:high_kernel

section .text
high_kernel:
	reload_segments

	mov esp, stack_top
	extern	kinit
	call	kinit
	hlt

section .bss
stack_bottom:
	resb 8192
stack_top:
section .data
multiboot_ptr:
	dd 0

section .padata
page_directory:
	dd page_table - KERNEL_BASE + 3
	times 768 - 1 dd 0x00000002
	dd page_table - KERNEL_BASE + 3
	times 256 - 1 dd 0x00000002
page_table:
	times 1024 dd 0x0
