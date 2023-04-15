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
extern schedule_task

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
	je .load_kernel_segments ; if cr3 is kernel don't swap

	mov cr3, eax

	.load_kernel_segments:
	load_kernel_segments

	mov eax, esp

	; (regs: &mut Registers)
	push eax
	call exception_handler
	pop eax

	call schedule_task
	; Never goes here

isr_stub_syscall dd isr_stub_128
irq_stub_0 dd irq_0
