%macro check_int 1
int_%1:
	cmp byte[esp + 4], %1
	jne %%next
	int %1
	jmp end
	%%next:
%endmacro
