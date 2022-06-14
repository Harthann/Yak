%include "boot.h"

section .multiboot_header
header_start:
	dd MULTIBOOT_MAGIC ; magic
	dd 0 ; architecture - 0: protected mode
	dd header_end - header_start ; header length
	dd 0x100000000 - (MAGIC + 0 + (header_end - header_start)) ; checksum
	dw 0 ; type
	dw 0 ; flags
	dd 8 ; size
header_end:
