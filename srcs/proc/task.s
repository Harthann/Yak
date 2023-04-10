%include "boot.h"
%include "task.h"

global irq_0
global switch_task
global swap_task
global next_task

extern page_directory
extern save_task
extern schedule_task

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
	je .get_kernel_kstack ; if cr3 is kernel don't swap

	mov cr3, eax
	jmp .jiffies

	.get_kernel_kstack:
	mov eax, esp

	mov esp, KSTACK_ADDR + 1

	push dword[eax + regs.ss]
	push dword[eax + regs.useresp]
	push dword[eax + regs.eflags]
	push dword[eax + regs.cs]
	push dword[eax + regs.eip]
	push dword[eax + regs.err_code]
	push dword[eax + regs.int_no]
	push dword[eax + regs.eax]
	push dword[eax + regs.ecx]
	push dword[eax + regs.edx]
	push dword[eax + regs.ebx]
	push dword[eax + regs.esp]
	push dword[eax + regs.ebp]
	push dword[eax + regs.esi]
	push dword[eax + regs.edi]
	push dword[eax + regs.cr3]
	push dword[eax + regs.ds]

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

; fn switch_task(regs: *const Registers)
switch_task:
	mov eax, dword[esp + 4] ; regs

	mov ebx, dword[eax + regs.cr3] ; cr3
	mov ecx, cr3
	cmp ebx, ecx
	je .get_regs ; if cr3 is kernel don't swap

	mov cr3, ebx

	.get_regs:
		mov eax, dword[eax + regs.ds] ; reload the original data segment descriptor
		mov ds, ax
		mov es, ax
		mov fs, ax
		mov gs, ax

		mov eax, KSTACK_ADDR + 1; reajust ptr with kstack
		sub eax, regs_size

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

		sti
		ret
