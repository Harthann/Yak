%include "boot.h"

global cursor

[BITS 32]
section .text:
	global _start

_start:
	mov esp, stack_top		; Stack pointer initialisation

	extern	rust_main
	call	rust_main
	hlt

; Stack creation
section .bss
stack_bottom:
	resb 8192
stack_top:
