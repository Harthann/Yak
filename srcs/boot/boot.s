%include "boot.h"

global cursor

section .text
[BITS 64]
long_mode_start:
	; load 0 into all data segment registers
	mov ax, 0
	mov ss, ax
	mov ds, ax
	mov es, ax
	mov fs, ax
	mov gs, ax

	extern	rust_main
	call	rust_main
	hlt

section .rodata
gdt64:
	dq 0 ; zero entry
.code: equ $ - gdt64 ; new
	dq (1<<43) | (1<<44) | (1<<47) | (1<<53) ; code segment
.pointer:
	dw $ - gdt64 - 1
	dq gdt64

[BITS 32]
section .text:
	global _start

_start:
	push eax
	print_string msg_protected_mode, 0x0f
	pop eax
	call disable_cursor
	; https://wiki.osdev.org/Setting_Up_Long_Mode
	mov esp, stack_top		; Stack pointer initialisation

	push eax
	print_string msg_multiboot, 0x0f
	pop eax
	call check_multiboot
	print_string msg_cpuid, 0x0f
	call check_cpuid
	print_string msg_longmode, 0x0f
	call check_long_mode

	print_string msg_setup_page_tables, 0x0f
	call set_up_page_tables
	print_string msg_enable_paging, 0x0f
	call enable_paging

	; load the 64-bit GDT
	lgdt [gdt64.pointer]

	print_string msg_start_long_mode, 0x0f
	jmp gdt64.code:long_mode_start

enable_paging:
	; load P4 to cr3 register (cpu uses this to access the P4 table)
	mov eax, p4_table
	mov cr3, eax

	; enable PAE-flag in cr4 (Physical Address Extension)
	mov eax, cr4
	or eax, 1 << 5
	mov cr4, eax

	; set the long mode bit in the EFER MSR (model specific register)
	mov ecx, 0xC0000080
	rdmsr
	or eax, 1 << 8
	wrmsr

	; enable paging in the cr0 register
	mov eax, cr0
	or eax, 1 << 31
	mov cr0, eax

	ret

set_up_page_tables:
	; map first P4 entry to P3 table
	mov eax, p3_table
	or eax, 0b11 ; present + writable
	mov [p4_table], eax

	; map first P3 entry to P2 table
	mov eax, p2_table
	or eax, 0b11 ; present + writable
	mov [p3_table], eax

	; map each P2 entry to a huge 2MiB page
	mov ecx, 0         ; counter variable

.map_p2_table:
	; map ecx-th P2 entry to a huge page that starts at address 2MiB*ecx
	mov eax, 0x200000  ; 2MiB
	mul ecx            ; start address of ecx-th page
	or eax, 0b10000011 ; present + writable + huge
	mov [p2_table + ecx * 8], eax ; map ecx-th entry

	inc ecx            ; increase counter
	cmp ecx, 512       ; if counter == 512, the whole P2 table is mapped
	jne .map_p2_table  ; else map the next entry

	ret

check_long_mode:
	; test if extended processor info in available
	mov eax, 0x80000000    ; implicit argument for cpuid
	cpuid                  ; get highest supported argument
	cmp eax, 0x80000001    ; it needs to be at least 0x80000001
	jb .no_long_mode       ; if it's less, the CPU is too old for long mode
	
	; use extended info to test if long mode is available
	mov eax, 0x80000001    ; argument for extended processor info
	cpuid                  ; returns various feature bits in ecx and edx
	test edx, 1 << 29      ; test if the LM-bit is set in the D-register
	jz .no_long_mode       ; If it's not set, there is no long mode
	ret
.no_long_mode:
	mov edi, 2
	jmp error

check_cpuid:
	; Check if CPUID is supported by attempting to flip the ID bit (bit 21)
	; in the FLAGS register. If we can flip it, CPUID is available.

	; Copy FLAGS in to EAX via stack
	pushfd
	pop eax

	; Copy to ECX as well for comparing later on
	mov ecx, eax

	; Flip the ID bit
	xor eax, 1 << 21

	; Copy EAX to FLAGS via the stack
	push eax
	popfd

	; Copy FLAGS back to EAX (with the flipped bit if CPUID is supported)
	pushfd
	pop eax

	; Restore FLAGS from the old version stored in ECX (i.e. flipping the
	; ID bit back if it was ever flipped).
	push ecx
	popfd

	; Compare EAX and ECX. If they are equal then that means the bit
	; wasn't flipped, and CPUID isn't supported.
	cmp eax, ecx
	je .no_cpuid
	ret
.no_cpuid:
    mov edi, 2
    jmp error

check_multiboot:
	cmp eax, BOOTLOADER_MAGIC
	jne .no_multiboot
	ret
.no_multiboot:
	mov edi, 2
	jmp error

disable_cursor:
	pushf
	push eax
	push edx
	mov dx, 0x3D4
	mov al, 0xA ; low cursor shape register
	out dx, al

	inc dx
	mov al, 0x20 ; bits 6-7 unused, bit 5 disables the cursor, bits 0-4 control the cursor shape
	out dx, al

	pop edx
	pop eax
	popf
	ret

error: ; (uint8_t error_code {edi})
	xor eax, eax
	push eax
	add edi, 0x30; print error code as ascii (must be < 10)
	push edi
	lea edi, [rel msg_error]
	mov esi, 0x4f
	call print_32bit_msg
	mov edi, esp
	mov esi, 0x4f
	call print_32bit_msg
	pop eax
	pop eax
	hlt

; print a msg {edi} of color {esi}
print_32bit_msg:; (uint32_t *msg {edi}, uint8_t color {esi})
	xor ecx, ecx
	.loop:
	cmp byte[edi + ecx], 0x0
	jz .end_loop
	cmp byte[edi + ecx], `\n`
	je .newline
	cmp dword[rel cursor], WINDOW_OFFSET + (VGA_WIDTH * 2) * VGA_HEIGHT; 25 lines - 80 char width
	je .scroll
	.after_scroll:
	mov eax, [rel cursor]
	mov dl, byte[edi + ecx]
	mov byte[eax], dl
	mov edx, esi
	mov byte[eax + 1], dl
	add dword[rel cursor], 2
	.continue:
	inc ecx
	jmp .loop
	.end_loop:
	add ecx, ecx
	ret
	.newline:
	call print_newline
	jmp .continue
	.scroll:
	push edi
	push esi
	mov edi, WINDOW_OFFSET
	mov esi, WINDOW_OFFSET + (VGA_WIDTH * 2)
	mov edx, (VGA_WIDTH * 2) * 24
	call _memcpy
	mov dword[rel cursor], WINDOW_OFFSET + (VGA_WIDTH * 2) * (VGA_HEIGHT - 1)
	mov edi, dword[rel cursor]
	mov esi, 0x0
	mov edx, VGA_WIDTH * 2
	call _memset
	pop esi
	pop edi
	jmp .after_scroll

; print a newline on screen
print_newline:
	push edx
	push ecx

	mov eax, [rel cursor]
	sub eax, WINDOW_OFFSET
	xor edx, edx
	mov ecx, VGA_WIDTH * 2
	div ecx
	mov eax, VGA_WIDTH * 2
	sub eax, edx
	add dword[rel cursor], eax

	pop ecx
	pop edx
ret

_memset: ; (uint8_t *dst {edi}, uint8_t c {esi}, uint32_t size {edx})
	push ecx

	mov ecx, edx
	mov eax, esi
	rep stosb
	mov eax, edi

	pop ecx
ret

_memcpy: ; (uint8_t *dst {edi}, uint8_t *src {esi}, uint32_t size {edx})
	push ecx

	mov eax, edi
	mov ecx, edx
	push edx
	mov edx, esi
	rep movsb
	push eax
	pop edi
	push edx
	pop esi
	pop edx
	mov eax, edi

	pop ecx
ret

section .data
msg_protected_mode		db `---------------------------[ PROTECTED MODE (32BITS) ]--------------------------`, 0x0
msg_multiboot			db `Check multiboot...\n`, 0x0
msg_cpuid				db `Check cpuid...\n`, 0x0
msg_longmode			db `Check longmode...\n`, 0x0
msg_setup_page_tables	db `Setup up page tables...\n`, 0x0
msg_enable_paging		db `Enable paging...\n`, 0x0
msg_start_long_mode		db `Starting up longmode...\n`, 0x0
msg_error				db `ERROR: `, 0x0

cursor				dd WINDOW_OFFSET

; Stack creation
section .bss
align 4096
p4_table:
    resb 4096
p3_table:
    resb 4096
p2_table:
    resb 4096
stack_bottom:
	resb 64
stack_top:

