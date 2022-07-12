%include "boot.h"

global _start
global stack_bottom
global stack_top
global page_directory
global page_table
global multiboot_ptr

[BITS 32]
section .boot
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
	extern	kmain
	call	kmain
	hlt

section .bss
stack_bottom:
	resb 8192
stack_top:
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
