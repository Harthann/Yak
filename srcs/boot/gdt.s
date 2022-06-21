global gdt_start
global gdt_desc

; [https://wiki.osdev.org/Global_Descriptor_Table]
;
; Every entry in the Global Descriptor Table (GDT) is a 8 bytes segment descriptor.
;
; Segment Descriptor (8 bytes)
;|-----------------------------|---------------------------|
;|63         56|55   52|51   48|47         40|39         32|
;|-------------|-------|-------|-------------|-------------|
;|Base         |Flags  |Limit  |Access Byte  |Base         |
;|31         24|3     0|19   16|7           0|23         16|
;|*****************************|***************************|
;|31                         16|15                        0|
;|-----------------------------|---------------------------|
;|Base                         |Limit                      |
;|15                          0|15                        0|
;|-----------------------------|---------------------------|
;
;Access Byte (8 bits)
;|---|-------|---|---|---|---|---|
;| 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
;|---|-------|---|---|---|---|---|
;|P  |DPL    |S  |E  |DC |RW |A  |
;|---|-------|---|---|---|---|---|
;
;[P]: Present byte (1 for any valid segment)
;[DPL]: Descriptor privilege level field
;	00 -> Kernel level (highest)
;	..
;	11 -> User application level (lowest)
;[S]: Descriptor type bit
;	0 -> System segment
;	1 -> Code or data segment
;[E]: Executable bit
;	0 -> Data segment
;	1 -> Code segment
;[DC]: Direction bit/Conforming bit
;	For data selector:
;		0 -> Segment grows up
;		1 -> Segment grows down
;	For code selector:
;		1 -> Can be executed from equal or lower privilege level (code in privilege 00 can far jump in 01)
;[RW]: Readable/Writable bit
;	For data segments:
;		Writable bit (Read always allowed in data segment)
;	For code segments:
;		Readable bit (Write always disallowed in code segment)
;[A]: Accessed bit
;	left to 0, reserved for CPU.

; 0b10011110

;Flags (4 bits)
;|---|---|---|---|
;| 3 | 2 | 1 | 0 |
;|---|---|---|---|
;|G  |DB |L  |XXX|
;|---|---|---|---|
;
;[G]: Granularity flag (size of limit value)
;	0 -> limit = 1 byte -> 20 bit address
;	1 -> limit in 4 KiB blocks (page granularity). -> 32 bit address
;[DB]: Size flag
;	0 -> 16 bit protected mode segment
;	1 -> 32 bit protected mode segment
;[L]: Long-mode code flag
;	1 -> 64 bit code segment
;[XXX]:	Reserved
; Here flags are all set to 1100 -> 

struc segment_descriptor
	limit			resw	1 ; 16 bits
	base			resb	3 ; 24 bits
	access			resb	1 ; 8 bits
	limit_flags		resb	1 ; high 4 bits(flags) - low 4 bits (limit)
	base_end		resb	1 ; 8 bits
endstruc

section .gdt
gdt_start:
	null:; null descriptor must be set to 0
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
			at access,		db 0b10011010; 0x9a (PRESENT_BYTE | KERNEL_LVL | CODE_OR_DATA | CODE_SEGMENT | READABLE_SEGMENT)
			at limit_flags,	db 0b11001111
			at base_end,	db 0x0
		iend
	kdata:
		istruc segment_descriptor
			at limit,		dw 0xffff
			at base,		db 0x0, 0x0, 0x0
			at access,		db 0b10010010; 0x92 (PRESENT_BYTE | KERNEL_LVL | CODE_OR_DATA | DATA_SEGMENT | GROWS_UP | WRITABLE_SEGMENT)
			at limit_flags,	db 0b11001111
			at base_end,	db 0x0
		iend
	kstack:
		istruc segment_descriptor
			at limit,		dw 0x0000
			at base,		db 0x0, 0x0, 0x0
			at access,		db 0b10010111;0x97 (PRESENT_BYTE | KERNEL_LVL | CODE_OR_DATA | DATA_SEGMENT | GROWS_DOWN | WRITABLE_SEGMENT | NOT_FOR_CPU)
			at limit_flags,	db 0b11000000
			at base_end,	db 0x0
		iend
	ucode:
		istruc segment_descriptor
			at limit,		dw 0xffff
			at base,		db 0x0, 0x0, 0x0
			at access,		db 0b11111010; 0xfa (PRESENT_BYTE | USER_LVL | CODE_OR_DATA | CODE_SEGMENT | READABLE_SEGMENT)
			at limit_flags,	db 0b11001111
			at base_end,	db 0x0
		iend
	udata:
		istruc segment_descriptor
			at limit,		dw 0xffff
			at base,		db 0x0, 0x0, 0x0
			at access,		db 0b11110010; 0f2 (PRESENT_BYTE | USER_LVL | CODE_OR_DATA | DATA_SEGMENT | GROWS_UP | WRITABLE_SEGMENT)
			at limit_flags,	db 0b11001111
			at base_end,	db 0x0
		iend
	ustack:
		istruc segment_descriptor
			at limit,		dw 0x0000
			at base,		db 0x0, 0x0, 0x0
			at access,		db 0b11110111; 0xf7 (PRESENT_BYTE | USER_LVL | CODE_OR_DATA | DATA_SEGMENT | GROWS_DOWN | WRITABLE_SEGMENT | NOT_FOR_CPU)
			at limit_flags,	db 0b11000000
			at base_end,	db 0x0
		iend
gdt_end:
;[bits 32]
;gdt descriptor ; (48 bits)
;|------------|------|
;|48        16|15   0|
;|------------|------|
;|offset      |size  |
;|31         0|15   0|
;|------------|------|
gdt_desc:
	offset	dw gdt_end - gdt_start
	size	dd gdt_start
