%include "boot.h"

global _start
global stack_bottom
global stack_top
global page_directory
global page_table

[BITS 32]
section .boot
_start:
	mov esp, stack_top - 0xc0000000 ; Stack pointer initialisation
	setup_page_table page_table, 0x0
	enable_paging page_directory

	extern gdt_desc
	lgdt [gdt_desc]

	jmp 0x08:high_kernel

section .text
high_kernel:
	reload_segments

	mov esp, stack_top
	extern	kernel_main
	call	kernel_main
	hlt

section .bss
stack_bottom:
	resb 8192
stack_top:

section .padata
page_directory:
	dd page_table - KERNEL_BASE + 3
	times 768 - 1 dd 0x00000002
	dd page_table - KERNEL_BASE + 3
	times 256 - 1 dd 0x00000002
page_table:
	times 1024 dd 0x0
