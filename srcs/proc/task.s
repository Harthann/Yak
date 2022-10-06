%include "task.h"

global switch_task

; fn switch_task(regs: *const Registers)
switch_task:
	mov eax, cr3
	push eax

	mov eax, dword[esp + 8] ; regs

	mov ebx, dword[eax + regs.ebx]
	mov ecx, dword[eax + regs.ecx]
	mov edx, dword[eax + regs.edx]
	mov esi, dword[eax + regs.esi]
	mov edi, dword[eax + regs.edi]
	mov ebp, dword[eax + regs.ebp]

	push dword[eax + regs.eflags]
	popf

	mov esp, dword[eax + regs.esp]

	push eax

	mov eax, dword[eax + regs.cr3] ; cr3
	mov cr3, eax

	pop eax
	push eax ; ?????

	mov eax, dword[eax + regs.eip] ; eip
	xchg eax, dword[esp]

	mov eax, dword[eax + regs.eax]

	sti
	ret
