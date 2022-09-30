global switch_task

; fn switch_task(reg_from: *const Registers, reg_to: *const Registers)
switch_task:
	pusha
	pushf

	mov eax, cr3
	push eax

	mov eax, dword[esp + 44] ; reg_from

	mov dword[eax + 4], ebx
	mov dword[eax + 8], ecx
	mov dword[eax + 12], edx
	mov dword[eax + 16], esi
	mov dword[eax + 20], edi

	mov ebx, dword[esp + 36] ; eax
	mov ecx, dword[esp + 40] ; eip
	mov edx, dword[esp + 20] ; esp
	add edx, 4 ; rm return value

	mov esi, dword[esp + 16] ; ebp
	mov edi, dword[esp + 4] ; eflags

	mov dword[eax], ebx
	mov dword[eax + 24], edx
	mov dword[eax + 28], esi
	mov dword[eax + 32], ecx
	mov dword[eax + 36], edi

	pop ebx ; cr3

	mov dword[eax + 40], ebx
	push ebx

	mov eax, dword[esp + 48] ; reg_to

	mov ebx, dword[eax + 4]
	mov ecx, dword[eax + 8]
	mov edx, dword[eax + 12]
	mov esi, dword[eax + 16]
	mov edi, dword[eax + 20]
	mov ebp, dword[eax + 28]

	push eax

	mov eax, dword[eax + 36]
	push eax
	popf

	pop eax
	mov esp, dword[eax + 24]

	push eax
	mov eax, dword[eax + 40] ; cr3

	mov cr3, eax
	pop eax
	push eax ; ?????

	mov eax, dword[eax + 32] ; eip

	xchg eax, dword[esp]

	mov eax, dword[eax]

	sti
	ret
