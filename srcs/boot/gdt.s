global GDT_start
global GDT_ptr

struc segment_descriptor
	limit			resw	1 ; 16 bits
	base			resb	3 ; 24 bits
	access			resb	1 ; 8 bits
	limit_flags		resb	1 ; high 4 bits(flags) - low 4 bits (limit)
	base_end		resb	1 ; 8 bits
endstruc

section .gdt
GDT_start:
	null:
		istruc segment_descriptor
			at limit,		dw 0x0
			at base,		db 0x0, 0x0, 0x0
			at access,		db 0x0
			at limit_flags,	db 0b00000000
			at base_end,	db 0x0
		iend
	kcode:
		istruc segment_descriptor
			at limit,		dw 0xffff
			at base,		db 0x0, 0x0, 0x0
			at access,		db 0x9a
			at limit_flags,	db 0b11001111
			at base_end,	db 0x0
		iend
	kdata:
		istruc segment_descriptor
			at limit,		dw 0xffff
			at base,		db 0x0, 0x0, 0x0
			at access,		db 0x92
			at limit_flags,	db 0b11001111
			at base_end,	db 0x0
		iend
	kstack:
		istruc segment_descriptor
			at limit,		dw 0x0000
			at base,		db 0x0, 0x0, 0x0
			at access,		db 0x97
			at limit_flags,	db 0b11000000
			at base_end,	db 0x0
		iend
	ucode:
		istruc segment_descriptor
			at limit,		dw 0xffff
			at base,		db 0x0, 0x0, 0x0
			at access,		db 0xfa
			at limit_flags,	db 0b11001111
			at base_end,	db 0x0
		iend
	udata:
		istruc segment_descriptor
			at limit,		dw 0xffff
			at base,		db 0x0, 0x0, 0x0
			at access,		db 0xf2
			at limit_flags,	db 0b11001111
			at base_end,	db 0x0
		iend
	ustack:
		istruc segment_descriptor
			at limit,		dw 0x0000
			at base,		db 0x0, 0x0, 0x0
			at access,		db 0xf7
			at limit_flags,	db 0b11000000
			at base_end,	db 0x0
		iend
GDT_ptr:
	dw $ - GDT_start - 1
	dd GDT_start
