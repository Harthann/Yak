%include "boot.h"

global stack_bottom
global stack_top

extern GDT_ptr

[BITS 32]
section .text:
	global _start

_start:
	mov esp, stack_top ; Stack pointer initialisation
	lgdt [GDT_ptr]

	extern	rust_main
	call	rust_main
	hlt

; Stack creation
section .bss
stack_bottom:
	resb 8192
stack_top:
