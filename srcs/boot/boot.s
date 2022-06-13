%include "boot.h"

global long_mode_start
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

section .text:
	global _start

section .rodata
gdt64:
	dq 0 ; zero entry
.code: equ $ - gdt64 ; new
	dq (1<<43) | (1<<44) | (1<<47) | (1<<53) ; code segment
.pointer:
	dw $ - gdt64 - 1
	dq gdt64

[BITS 32]
_start:
	mov esp, stack_top		; Stack pointer initialisation

	push eax
	lea edi, [rel msg_multiboot]
	mov esi, 0x0f
	call print_32bit_msg
	pop eax
	call check_multiboot
	lea edi, [rel msg_cpuid]
	mov esi, 0x0f
	call print_32bit_msg
	call check_cpuid
	lea edi, [rel msg_longmode]
	mov esi, 0x0f
	call print_32bit_msg
	call check_long_mode

	call set_up_page_tables ; new
	call enable_paging     ; new

	; load the 64-bit GDT
	lgdt [gdt64.pointer]

	jmp gdt64.code:long_mode_start

msg_multiboot db `Check multiboot...\n`, 0x0
msg_cpuid db `Check cpuid...\n`, 0x0
msg_longmode db `Check longmode...\n`, 0x0

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
	cmp eax, 0x36d76289
	jne .no_multiboot
	ret
.no_multiboot:
	mov edi, 2
	jmp error

error: ; (uint8_t error_code {edi})
	xor eax, eax
	push eax
	add edi, 0x30; print error code as ascii (must be < 10)
	push edi
	lea edi, [rel error_msg]
	mov esi, 0x4f
	call print_32bit_msg
	mov edi, esp
	mov esi, 0x4f
	call print_32bit_msg
	pop eax
	pop eax
	hlt

error_msg db `Error: `, 0x0

; print a msg {edi} of color {esi}
print_32bit_msg:; (uint32_t *msg {edi}, uint8_t color {esi})
	xor ecx, ecx
	.loop:
	cmp byte[edi + ecx], 0x0
	jz .end_loop
	cmp byte[edi + ecx], `\n`
	je .newline
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

print_newline:
	push edx
	push ecx
	mov eax, [rel cursor]
	sub eax, 0x000b8000
	xor edx, edx
	mov ecx, 80 * 2
	div ecx
	mov eax, 80 * 2
	sub eax, edx
	add dword[rel cursor], eax
	pop ecx
	pop edx
	ret

cursor dd 0x000b8000

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

