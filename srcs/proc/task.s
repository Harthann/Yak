%include "boot.h"
%include "task.h"

global irq_0
global switch_task
global swap_task
global next_task

extern page_directory
extern save_task
extern schedule_task
extern tmp_registers

extern JIFFIES

swap_task:
	pusha

	mov eax, cr3
	push eax

	xor eax, eax
	mov ax, ds
	push eax

	mov eax, page_directory - KERNEL_BASE
	mov ebx, cr3
	cmp eax, ebx
	je .jiffies ; if cr3 is kernel don't swap

	mov cr3, eax

	.jiffies:
	add dword[JIFFIES], 1

	mov dx, 0x20
	mov al, 0x20
	out dx, al

	load_kernel_segments

	mov eax, esp

	; (regs: &mut Registers)
	push eax
	call save_task
	pop eax

	call schedule_task
	; Never goes there

; fn switch_task()
switch_task:
	mov eax, tmp_registers ; regs

	mov ebx, dword[eax + regs.cr3] ; cr3
	mov ecx, cr3
	cmp ebx, ecx
	je .get_regs ; if cr3 is kernel don't swap

	mov cr3, ebx

	.get_regs:
		mov ebx, dword[eax + regs.ds] ; reload the original data segment descriptor
		mov ds, bx
		mov es, bx
		mov fs, bx
		mov gs, bx

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

		; no sti: iretd enable interrupt itself
		iret

	.new_task:
		push dword[eax + regs.eip]; jump directly on eip
		mov eax, dword[eax + regs.eax]

		; sti in wrappers
		ret
