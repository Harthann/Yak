%ifndef BOOT_H
%define BOOT_H

; multiboot header
%define MULTIBOOT_MAGIC		0xe85250d6

; bootloader
%define BOOTLOADER_MAGIC	0x36d76289

; VGA
%define WINDOW_OFFSET		0xb8000
%define VGA_WIDTH			80
%define VGA_HEIGHT			25

; print_string(label, color)
%macro print_string 2
	lea edi, [rel %1]
	mov esi, %2
	call print_32bit_msg
%endmacro

%endif
