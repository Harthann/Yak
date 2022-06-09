%include "kfs.h"

section .text:
	global _start

_start:
	jmp _start

	times 510-($-$$) db 0
	db `\x55\xAA` ; boot signature - legacy BIOS or QEMU
