%include "task.h"

global irq_0
global switch_task
global next_task

extern STACK_TASK_SWITCH
extern save_task
extern schedule_task

extern JIFFIES

irq_0:
	cli
	push 0; err_code
	push -1; int_no

	jmp swap_task

swap_task:
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
	mov esp, dword[STACK_TASK_SWITCH]

	; (regs: &mut Registers)
	push eax
	call save_task
	pop eax

	call schedule_task
	; Never goes there

; fn switch_task(regs: *const Registers)
switch_task:
	mov eax, dword[esp + 4] ; regs

	mov ebp, dword[eax + regs.cr3] ; cr3
	mov cr3, ebp

	mov edi, dword[eax + regs.edi]
	mov esi, dword[eax + regs.esi]
	mov ebp, dword[eax + regs.ebp]
	mov esp, dword[eax + regs.esp]
	mov ebx, dword[eax + regs.ebx]
	mov edx, dword[eax + regs.edx]
	mov ecx, dword[eax + regs.ecx]

	cmp dword[eax + regs.int_no], -1
	jne .new_task

	mov eax, dword[eax + regs.eax]
	add esp, 8 ; int_no and err_code

	sti
	iretd

	.new_task:
		push dword[eax + regs.eip]; jump directly on eip
		mov eax, dword[eax + regs.eax]

		sti
		ret
