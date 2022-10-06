%include "idt.h"
%include "task.h"

global isr_stub_table
global isr_stub_syscall
global irq_stub_0

extern next_task

extern regs

extern JIFFIES
irq_0:
	cli
	push 0; err_code
	push -1; int_no

	pusha

	mov eax, cr3
	push eax

	xor eax, eax
	mov ax, ds
	push eax

	pushf
	mov eax, dword[esp]
	mov dword[esp + regs.eflags], eax
	popf

	mov eax, dword[JIFFIES]
	inc eax
	mov dword[JIFFIES], eax

	mov dx, 0x20
	mov al, 0x20
	out dx, al

	add dword[esp + regs.esp], 20; ret

	mov eax, esp
	push eax

	call next_task

	; never goes there

isr_stub_table:
	%assign i 0
	%rep	48
		dd isr_stub_%+i
	%assign i i+1
	%endrep

isr_stub_syscall dd isr_stub_128
irq_stub_0 dd irq_0
