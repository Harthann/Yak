%include "boot.h"

[BITS 16]
[ORG 0x7c00]

	xor ax, ax
	mov ds, ax
	cld

	mov si, msg
	cld

	call bios_print

hang:
	jmp hang

bios_print:
	lodsb
	or al, al
	jz .end_print
	mov ah, BINT_TELETYPE
	mov bh, 0x0
	int 0x10
	jmp bios_print
	.end_print:
		ret

	msg db `42 superboot powered by nieyraud and lmartin`, 0x0
_signature:
	times 510-($-$$) db 0
	db `\x55\xAA` ; boot signature - legacy BIOS or QEMU
