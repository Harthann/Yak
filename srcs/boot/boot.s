%include "boot.h"

global stack_bottom
global stack_top
global _start

[BITS 32]
section .boot
_start:
	mov esp, stack_top - 0xc0200000 ; Stack pointer initialisation

	extern	rust_start
	call	rust_start
	hlt

; Stack creation
section .bss
stack_bottom:
	resb 8192
stack_top:
