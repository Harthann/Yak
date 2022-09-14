
/*
** All reserved field can probably be merge with adjacent u16 register
** Like shown in osdev guide with tss_entry example
** https://wiki.osdev.org/Getting_to_Ring_3
*/
#[repr(packed)]
pub struct Tss {
/* Nominated as "link" in osdev, correspond to previous task */
	prev:		u16,
	reserved1:	u16,

/* Pointer to kernel stack for kernel mode switching */
	esp0:		u32,

/* Kernel stack segment for kernel mode switching */
	ss0:		u16,
	reserved2:	u16,

	esp1:		u32,
	ss1:		u16,
	reserved3:	u16,

	esp2:		u32,
	ss2:		u16,
	reserved4:	u16,

	cr3:		u32,
	eip:		u32,
	eflags:		u32,
	eax:		u32,
	ecx:		u32,
	edx:		u32,
	ebx:		u32,
	esp:		u32,
	ebp:		u32,
	esi:		u32,
	edi:		u32,

	es:			u16,
	reserved5:	u16,	

	cs:			u16,
	reserved6:	u16,	

	ss:			u16,
	reserved7:	u16,	

	ds:			u16,
	reserved8:	u16,	

	fs:			u16,
	reserved9:	u16,	

	gs:			u16,
	reserved10:	u16,	

	ldtr:		u16,
	reserved11:	u16,	

/* Presented as reserved in Task State Switching tutorial */
	trap:		u16,	
	iopb:		u16,

/*
**	Field ssp is present in osdev Task State Switching tutorial 
**	but not in the example shown higher	
**	ssp:		u32
*/
}
