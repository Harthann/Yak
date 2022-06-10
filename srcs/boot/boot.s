%include "boot.h"


section .text:
	global _start

[BITS 32]
_start:
	mov esp, stack_top		; Stack pointer initialisation

;	According to osdev.wiki, GDT should be initialized here

;In long mode, x86 uses a page size of 4096 bytes and a 4 level page table that consists of:
;    Page-Map Level-4 Table			(PML4)
;    Page-Directory Pointer Table	(PDP)
;    Page-Directory Table			(PD)
;    Page Table						(PT)

	mov dword [0xb8000], 0x2f322f34
	extern	rust_main
	call	rust_main
	hlt

; Stack creation
section .bss
stack_bottom:
	resb 64
stack_top:
