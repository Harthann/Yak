%include "task.h"

global irq_0
global switch_task
global save_and_next_task

extern STACK_TASK_SWITCH
extern next_task

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
	mov esp, dword[STACK_TASK_SWITCH]

	; (regs: &mut Registers)
	push eax

	call next_task
	; Never goes there

; fn save_and_next_task()
save_and_next_task:
	cli

	sub esp, 20

	push eax
	xor eax, eax

	mov ax, ss
	mov dword[esp + 20], eax ; ss
	mov ax, sp
	mov dword[esp + 16], eax ; useresp
	pushfd
	pop eax
	mov dword[esp + 12], eax ; eflags
	mov ax, cs
	mov dword[esp + 8], eax ; cs
	mov eax, dword[esp + 24]
	mov dword[esp + 4], eax ; eip

	pop eax

	jmp irq_0

; fn switch_task(regs: *const Registers)
switch_task:
	mov ebp, dword[esp + 4] ; regs

	mov eax, dword[ebp + regs.cr3] ; cr3
	mov cr3, eax

	mov eax, ebp

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
