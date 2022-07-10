pub struct InterruptDescriptor {
	offset_0:		u16,
	selector:		u16,
	zero:			u8,
	type_attr:		u8, /* gate type, dpl and p fields */
	offset_1:		u16
}

impl InterruptDescriptor {
	pub fn set_offset(&mut self, offset: u32) {
		self.offset_0 = (offset & 0x0000ffff) as u16;
		self.offset_1 = ((offset & 0xffff0000) >> 16) as u16;
	}

	pub fn set_selector(&mut self, selector: u16) {
		self.selector = selector;
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

pub struct IDTR {
	size:			u16,
	offset:			u32
}
