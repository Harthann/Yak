%ifndef IDT_H
%define IDT_H

extern exception_handler
extern isr_common_stub

%macro isr_err_stub 1
isr_stub_%+%1:
	cli
	push %1
	jmp isr_common_stub
%endmacro

%macro isr_no_err_stub 1
isr_stub_%+%1:
	cli
	push 0
	push %1
	jmp isr_common_stub
%endmacro

isr_no_err_stub		0
isr_no_err_stub		1
isr_no_err_stub		2
isr_no_err_stub		3
isr_no_err_stub		4
isr_no_err_stub		5
isr_no_err_stub		6
isr_no_err_stub		7
isr_err_stub		8
isr_no_err_stub		9
isr_err_stub		10
isr_err_stub		11
isr_err_stub		12
isr_err_stub		13
isr_err_stub		14
isr_no_err_stub		15
isr_no_err_stub		16
isr_err_stub		17
isr_no_err_stub		18
isr_no_err_stub		19
isr_no_err_stub		20
isr_no_err_stub		21
isr_no_err_stub		22
isr_no_err_stub		23
isr_no_err_stub		24
isr_no_err_stub		25
isr_no_err_stub		26
isr_no_err_stub		27
isr_no_err_stub		28
isr_no_err_stub		29
isr_err_stub		30
isr_no_err_stub		31
isr_no_err_stub		32
isr_no_err_stub		33
isr_no_err_stub		34
isr_no_err_stub		35
isr_no_err_stub		36
isr_no_err_stub		37
isr_no_err_stub		38
isr_no_err_stub		39
isr_no_err_stub		40
isr_no_err_stub		41
isr_no_err_stub		42
isr_no_err_stub		43
isr_no_err_stub		44
isr_no_err_stub		45
isr_no_err_stub		46
isr_no_err_stub		47

isr_no_err_stub		128 ; syscalls

%endif
