%include "task.h"

global switch_task

; fn switch_task(regs: *const Registers)
switch_task:
	mov ebp, dword[esp + 4] ; regs

	mov eax, dword[ebp + regs.ds]
	mov ds, ax

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

	push dword[eax + regs.eflags]
	popf

	push dword[eax + regs.eip]; jump directly on eip
	mov eax, dword[eax + regs.eax]

	sti
	ret
