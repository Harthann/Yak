%include "idt.h"
%include "task.h"

global isr_stub_table
global isr_stub_syscall
global irq_stub_0
global isr_common_stub

extern irq_0
extern page_directory
extern exception_handler

isr_stub_table:
	%assign i 0
	%rep	48
		dd isr_stub_%+i
	%assign i i+1
	%endrep

isr_common_stub:
	pusha

	mov eax, cr3
	push eax

	mov eax, page_directory - 0xc0000000
	mov cr3, eax

	xor eax, eax
	mov ax, ds      ; Lower 16-bits of eax = ds.
	push eax        ; save the data segment descriptor

	mov ax, 0x10    ; load the kernel data segment descriptor
	mov ds, ax
	mov es, ax
	mov fs, ax
	mov gs, ax

	mov eax, esp
	push eax        ; push pointer to regs

	call exception_handler

	pop eax

	pop eax         ; reload the original data segment descriptor
	mov ds, ax
	mov es, ax
	mov fs, ax
	mov gs, ax

	pop eax
	mov cr3, eax

	popa
	add esp, 8

	sti
	iretd

isr_stub_syscall dd isr_stub_128
irq_stub_0 dd irq_0
