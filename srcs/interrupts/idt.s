%include "idt.h"
%include "task.h"

global isr_stub_table
global isr_stub_syscall
global irq_stub_0

extern STACK_TASK_SWITCH
extern next_task
extern regs

extern JIFFIES
extern SYSTEM_FRACTION
extern TIME_ELAPSED
extern irq_0

isr_stub_table:
	%assign i 0
	%rep	48
		dd isr_stub_%+i
	%assign i i+1
	%endrep

isr_stub_syscall dd isr_stub_128
irq_stub_0 dd irq_0
