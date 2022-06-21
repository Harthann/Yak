%include "boot.h"

global stack_bottom
global stack_top

extern gdt_desc

[BITS 32]
section .text:
	global _start

_start:
	mov esp, stack_top ; Stack pointer initialisation
	lgdt [gdt_desc] ; load gdt
	call reload_segments

	extern	rust_main
	call	rust_main
	hlt

reload_segments:
	jmp 0x08:.reload_cs
	.reload_cs:
	mov ax, 0x10
	mov ds, ax
	mov es, ax
	mov fs, ax
	mov gs, ax
	mov ss, ax
	ret

; Stack creation
section .bss
stack_bottom:
	resb 8192
stack_top:
