use core::arch::asm;
use core::mem::size_of;
use core::fmt;
use crate::println;

pub struct SegmentDescriptor {
	limit:			u16,
	base:			[u8; 3],
	access:			u8,
	limit_flags:	u8,
	base_end:		u8
}

impl SegmentDescriptor {
	pub fn set_limit(&mut self, limit: u32) {
		self.limit = (limit & 0x0000ffff) as u16;
		self.limit_flags &= 0xf0;
		self.limit_flags |= ((limit & 0x000f0000) >> 16) as u8;
	}

	pub fn set_base(&mut self, base: u32) {
		self.base[0] = (base & 0x000000ff) as u8;
		self.base[1] = ((base & 0x0000ff00) >> 8) as u8;
		self.base[2] = ((base & 0x00ff0000) >> 16) as u8;
		self.base_end = ((base & 0xff000000) >> 24) as u8;
	}

	pub fn set_flag(&mut self, flag: u8) {
		self.limit_flags &= 0x0f;
		self.limit_flags |= (flag & 0x0f) << 4;
	}

	pub fn set_access(&mut self, access: u8) {
		self.access = access;
	}
}

impl fmt::Display for SegmentDescriptor {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "limit: {:#06x}
base: {:#04x?}
access: {:#b}
limit_flags: {:#b}
base_end: {:#04x}", self.limit, self.base, self.access, self.limit_flags, self.base_end)
	}
}

extern "C" {
	fn gdt_start();
	fn gdt_desc();
}

pub fn print_gdt() {
	let mut segments: *mut SegmentDescriptor = 0x800 as *mut _;
	let mut tmp = (gdt_start as *const ()) as usize;
	let end = (gdt_desc as *const ()) as usize;
	while tmp < end {
		let segment = unsafe{&*segments};
		println!("\nSegment:\n{}", segment);
		segments = unsafe{segments.add(1)};
		tmp += size_of::<SegmentDescriptor>();
	}
}

pub fn get_segment(index: usize) -> &'static mut SegmentDescriptor{
	let segments: *mut SegmentDescriptor = 0x800 as *mut _;
	unsafe{&mut *(segments.add(index))}
}

#[no_mangle]
pub extern "C" fn load_gdt() {
	let addr = (gdt_desc as *const ()) as usize;
	unsafe{asm!("lgdt [{0}]", in(reg) addr);}
}

#[no_mangle]
pub extern "C" fn reload_segments() {
	unsafe{asm!("ljmp $0x08, $2f",
				"2:",
				"movw $0x10, %ax",
				"movw %ax, %ds",
				"movw %ax, %es",
				"movw %ax, %fs",
				"movw %ax, %gs",
				"movw %ax, %ss", options(att_syntax))};
}
