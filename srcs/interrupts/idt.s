%include "idt.h"

global isr_stub_table
global isr_stub_syscall
global irq_stub_0

extern next_task

extern JIFFIES
irq_0:
	cli
	push eax
	push edx

	mov eax, [JIFFIES]
	inc eax
	mov [JIFFIES], eax

	mov dx, 0x20,
	mov al, 0x20
	out dx, al

	pop edx
	pop eax

	call next_task

	iretd

isr_stub_table:
	%assign i 0
	%rep	48
		dd isr_stub_%+i
	%assign i i+1
	%endrep

isr_stub_syscall dd isr_stub_128
irq_stub_0 dd irq_0
