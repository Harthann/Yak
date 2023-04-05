%include "boot.h"
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

	xor eax, eax
	mov ax, ds      ; Lower 16-bits of eax = ds.
	push eax        ; save the data segment descriptor

	mov eax, page_directory - KERNEL_BASE
	mov ebx, cr3
	cmp eax, ebx
	je .get_kernel_kstack ; if cr3 is kernel don't swap

	mov cr3, eax
	jmp .kernel_ds

	.get_kernel_kstack:
	mov eax, esp
	mov esp, KSTACK_ADDR + 1 - 0x1000; take the lower kstack to handle exception

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

	.kernel_ds:
	mov ax, 0x10    ; load the kernel data segment descriptor
	mov ds, ax
	mov es, ax
	mov fs, ax
	mov gs, ax

	mov eax, esp
	push eax        ; push pointer to regs

	call exception_handler
	; Never goes here

isr_stub_syscall dd isr_stub_128
irq_stub_0 dd irq_0
