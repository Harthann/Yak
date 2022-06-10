%include "boot.h"

section .text:
	global _start

[BITS 32]
_start:
	mov dword [0xb8000], 0x2f322f34
    hlt
