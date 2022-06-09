%include "kfs.inc"

section .text:
	global _start

_start:
	mov rdi, 1
	lea rsi, [rel helloworld]
	mov rdx, 14
	mov rax, 1
	syscall


helloworld db `Hello World !\n`, 0x0
