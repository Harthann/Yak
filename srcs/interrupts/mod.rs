const GDT_OFFSET_KERNEL_CODE: u16 = 0x08; /* TODO: compute it ? */
const IDT_SIZE: usize = 32;
const IDT_MAX_DESCRIPTORS: usize = 256;

static mut ISR_STUB_TABLE: [u32; IDT_SIZE] = [0; IDT_SIZE];

extern "C" {
        static mut isr_stub_table: [u32; IDT_SIZE];
}

static mut IDT: IDT = IDT {
	idt_entries: [InterruptDescriptor {
					offset_0: 0,
					selector: 0,
					zero: 0,
					type_attr: 0,
					offset_1: 0
				}; IDT_MAX_DESCRIPTORS],
	idtr: IDTR {
		size: 0,
		offset: 0
	}
};

/* TODO: [https://wiki.osdev.org/Interrupts_tutorial]*/
#[no_mangle]
pub extern "C" fn exception_handler(i: u32) {
	panic!("Exception ! {}", i);
}

pub unsafe fn init_idt() {
	let mut i;

	IDT.idtr.offset = (&IDT.idt_entries[0] as *const _) as u32;
	IDT.idtr.size = (core::mem::size_of::<IDTR>() * IDT_MAX_DESCRIPTORS - 1) as u16;
	i = 0;
	while i < IDT_SIZE {
		ISR_STUB_TABLE[i] = exception_handler as u32;
		i += 1;
	}
	i = 0;
	while i < IDT_SIZE {
		let offset: u32 = isr_stub_table[i];
		IDT.idt_entries[i].init(offset, GDT_OFFSET_KERNEL_CODE, 0x8e);
		i += 1;
	}
	core::arch::asm!("lidt [{}]", in(reg) (&IDT.idtr as *const _) as u32);
//	core::arch::asm!("sti");
}

#[repr(C, align(16))]
pub struct IDT {
	pub idt_entries: [InterruptDescriptor; IDT_MAX_DESCRIPTORS],
	pub idtr: IDTR
}

#[repr(packed)]
pub struct IDTR {
	pub size:			u16,
	pub offset:			u32
}

#[repr(packed)]
#[derive(Copy, Clone)]
pub struct InterruptDescriptor {
	offset_0:		u16,
	selector:		u16,
	zero:			u8,
	type_attr:		u8, /* gate type, dpl and p fields */
	offset_1:		u16
}

impl InterruptDescriptor {
	pub fn init(&mut self, offset: u32, select: u16, type_attr: u8)
	{
		self.set_offset(offset);
		self.selector = select;
		self.zero = 0;
		self.type_attr = type_attr;
	}

	pub fn set_offset(&mut self, offset: u32) {
		self.offset_0 = (offset & 0x0000ffff) as u16;
		self.offset_1 = ((offset & 0xffff0000) >> 16) as u16;
	}

	pub fn set_gate_type(&mut self, gate_type: u8) {
		self.type_attr &= 0b11110000;
		self.type_attr |= gate_type & 0b00001111;
	}

	pub fn set_dpl(&mut self, dpl: u8) {
		self.type_attr &= 0b10011111;
		self.type_attr |= (dpl << 5) & 0b01100000;
	}

	pub fn set_p(&mut self, p: u8) {
		self.type_attr &= 0b10000000;
		self.type_attr |= (p << 7) & 0b10000000;
	}

	pub fn get_offset(&self) -> u32 {
		((self.offset_1 as u32) << 16) | self.offset_0 as u32
	}

	pub fn get_selector(&self) -> u16 {
		self.selector
	}

	pub fn get_gate_type(&self) -> u8 {
		self.type_attr & 0b00001111
	}

	pub fn get_dpl(&self) -> u8 {
		(self.type_attr & 0b01100000) >> 5
	}

	pub fn get_p(&self) -> u8 {
		(self.type_attr & 0b10000000) >> 7
	}
}
