%include "idt.h"

global isr_stub_table

isr_stub_table:
	%assign i 0 
	%rep	32 
		dd isr_stub_%+i ; use DQ instead if targeting 64-bit
	%assign i i+1 
	%endrep

isr_common_stub:
	pusha

	mov ax, ds               ; Lower 16-bits of eax = ds.
	push eax                 ; save the data segment descriptor

	mov ax, 0x10  ; load the kernel data segment descriptor
	mov ds, ax
	mov es, ax
	mov fs, ax
	mov gs, ax

	call exception_handler

	pop eax        ; reload the original data segment descriptor
	mov ds, ax
	mov es, ax
	mov fs, ax
	mov gs, ax

	add esp, 8
	popa
	iret
