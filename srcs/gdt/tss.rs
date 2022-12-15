
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

pub static mut TSS: Tss = Tss::new();

pub fn init_tss(stack_addr: u32) {
	unsafe {
		TSS.esp0 = stack_addr;
		TSS.ss0 = 0x18;
		TSS.iopb = core::mem::size_of::<Tss>() as u16;
/*	Page directory entry for virt addr */
		crate::gdt::set_segment(7, (&TSS as *const Tss) as u32, TSS.iopb as u32, 0x40, 0x89);
	}
}

impl Tss {
	pub const fn new() -> Tss {
		Tss {
			prev:		0,
			reserved1:	0,
	 		esp0:		0,
	 		ss0:		0,
	 		reserved2:	0,
	 		esp1:		0,
	 		ss1:		0,
	 		reserved3:	0,
	 		esp2:		0,
	 		ss2:		0,
	 		reserved4:	0,
	 		cr3:		0,
	 		eip:		0,
	 		eflags:		0,
	 		eax:		0,
	 		ecx:		0,
	 		edx:		0,
	 		ebx:		0,
	 		esp:		0,
	 		ebp:		0,
	 		esi:		0,
	 		edi:		0,
	 		es:			0,
	 		reserved5:	0,
	 		cs:			0,
	 		reserved6:	0,
	 		ss:			0,
	 		reserved7:	0,
	 		ds:			0,
	 		reserved8:	0,
	 		fs:			0,
	 		reserved9:	0,
	 		gs:			0,
	 		reserved10:	0,
	 		ldtr:		0,
	 		reserved11:	0,
	 		trap:		0,
	 		iopb:		0
		}
	}
}
