%include "boot.h"

global stack_bottom
global stack_top

extern gdt_desc

[BITS 32]
section .text:
	global _start

_start:
	mov esp, stack_top ; Stack pointer initialisation
	extern load_gdt
	call load_gdt
	extern reload_segments
	call reload_segments

	extern	rust_main
	call	rust_main
	hlt

; Stack creation
section .bss
stack_bottom:
	resb 8192
stack_top:
