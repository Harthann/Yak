%include "idt.h"
%include "task.h"

global isr_stub_table
global isr_stub_syscall
global irq_stub_0

extern STACK_TASK_SWITCH
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

	add dword[JIFFIES], 1

	mov dx, 0x20
	mov al, 0x20
	out dx, al

	mov eax, esp

	; Setup temp task
	mov esp, STACK_TASK_SWITCH
	sub esp, 8

	; (regs: &mut Registers)
	push eax

	call next_task
	; Never goes there

isr_stub_table:
	%assign i 0
	%rep	48
		dd isr_stub_%+i
	%assign i i+1
	%endrep

isr_stub_syscall dd isr_stub_128
irq_stub_0 dd irq_0
