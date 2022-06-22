use core::mem::size_of;
use core::fmt;

use crate::println;

pub struct SegmentDescriptor {
	pub limit:			u16,
	pub base:			[u8; 3],
	pub access:			u8,
	pub limit_flags:	u8,
	pub base_end:		u8
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
