
/*
** All reserved field can probably be merge with adjacent u16 register
** Like shown in osdev guide with tss_entry example
** https://wiki.osdev.org/Getting_to_Ring_3
*/
#[derive(Default, Copy, Clone)]
#[repr(packed)]
pub struct Tss {
/* Nominated as "link" in osdev, correspond to previous task */
	pub prev:		u16,
	pub reserved1:	u16,

/* Pointer to kernel stack for kernel mode switching */
	pub esp0:		u32,

/* Kernel stack segment for kernel mode switching */
	pub ss0:		u16,
	pub reserved2:	u16,

	pub esp1:		u32,
	pub ss1:		u16,
	pub reserved3:	u16,

	pub esp2:		u32,
	pub ss2:		u16,
	pub reserved4:	u16,

	pub cr3:		u32,
	pub eip:		u32,
	pub eflags:		u32,
	pub eax:		u32,
	pub ecx:		u32,
	pub edx:		u32,
	pub ebx:		u32,
	pub esp:		u32,
	pub ebp:		u32,
	pub esi:		u32,
	pub edi:		u32,

	pub es:			u16,
	pub reserved5:	u16,	

	pub cs:			u16,
	pub reserved6:	u16,	

	pub ss:			u16,
	pub reserved7:	u16,	

	pub ds:			u16,
	pub reserved8:	u16,	

	pub fs:			u16,
	pub reserved9:	u16,	

	pub gs:			u16,
	pub reserved10:	u16,	

	pub ldtr:		u16,
	pub reserved11:	u16,	

/* Presented as reserved in Task State Switching tutorial */
	pub trap:		u16,	
	pub iopb:		u16,

/*
**	Field ssp is present in osdev Task State Switching tutorial 
**	but not in the example shown higher	
**	ssp:		u32
*/
}

