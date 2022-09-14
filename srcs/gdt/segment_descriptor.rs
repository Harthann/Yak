use core::fmt;

pub struct SegmentDescriptor {
	limit:			u16,
	base:			[u8; 3],
	access:			u8,
	limit_flags:	u8,
	base_end:		u8
}

impl SegmentDescriptor {
	#[allow(dead_code)]
	pub fn set_limit(&mut self, limit: u32) {
		self.limit = (limit & 0x0000ffff) as u16;
		self.limit_flags &= 0xf0;
		self.limit_flags |= ((limit & 0x000f0000) >> 16) as u8;
	}

	#[allow(dead_code)]
	pub fn set_base(&mut self, base: u32) {
		self.base[0] = (base & 0x000000ff) as u8;
		self.base[1] = ((base & 0x0000ff00) >> 8) as u8;
		self.base[2] = ((base & 0x00ff0000) >> 16) as u8;
		self.base_end = ((base & 0xff000000) >> 24) as u8;
	}

	#[allow(dead_code)]
	pub fn set_flag(&mut self, flag: u8) {
		self.limit_flags &= 0x0f;
		self.limit_flags |= (flag & 0x0f) << 4;
	}

	#[allow(dead_code)]
	pub fn set_access(&mut self, access: u8) {
		self.access = access;
	}

	#[allow(dead_code)]
	pub fn get_base(&self) -> u32 {
		let mut base:u32 = self.base_end.into();
		base = base << 8 | (self.base[2] as u32);
		base = base << 8 | (self.base[1] as u32);
		base = base << 8 | (self.base[0] as u32);
		base
	}
}

impl fmt::Display for SegmentDescriptor {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "base: {:#010x}
limit: {:#06x}
access: {:#010b}
limit_flags: {:#06b}",
	self.get_base(), self.limit, self.access, self.limit_flags)
	}
}
