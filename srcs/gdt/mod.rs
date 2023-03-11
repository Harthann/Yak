//! GDT and TSS setup/helpers

use crate::kprintln;
use core::fmt;
pub mod tss;
pub use tss::{init_tss, Tss};

pub const KERNEL_BASE: usize = 0xc0000000;

extern "C" {
	pub fn gdt_start();
	pub fn gdt_desc();
}

#[repr(packed)]
pub struct GDTR {
	size:   u16,
	offset: u32
}

pub unsafe fn update_gdtr() {
	let gdtr: &mut GDTR = &mut *((gdt_desc as usize + KERNEL_BASE) as *mut _);
	gdtr.offset = (gdt_start as usize + KERNEL_BASE) as u32;
}

#[derive(Debug)]
pub struct SegmentDescriptor {
	limit:       u16,
	base:        [u8; 3],
	access:      u8,
	limit_flags: u8,
	base_end:    u8
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
		let mut base: u32 = self.base_end.into();
		base = base << 8 | (self.base[2] as u32);
		base = base << 8 | (self.base[1] as u32);
		base = base << 8 | (self.base[0] as u32);
		base
	}
}

impl fmt::Display for SegmentDescriptor {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(
			f,
			"base: {:#010x}
limit: {:#06x}
access: {:#010b}
limit_flags: {:#06b}",
			self.get_base(),
			self.limit,
			self.access,
			self.limit_flags
		)
	}
}

#[allow(dead_code)]
pub fn print_gdt() {
	let mut segments: *mut SegmentDescriptor =
		(gdt_start as usize + KERNEL_BASE) as *mut _;
	let mut id = 0;
	let end = (gdt_desc as usize + KERNEL_BASE) as *mut SegmentDescriptor;
	while segments < end {
		let segment = unsafe { &*segments };
		kprintln!("\nSegment {}:\n{}", id, segment);
		segments = unsafe { segments.add(1) };
		id += 1;
	}
}

#[allow(dead_code)]
pub fn get_segment(index: usize) -> &'static mut SegmentDescriptor {
	let segments: *mut SegmentDescriptor =
		(gdt_start as usize + KERNEL_BASE) as *mut _;
	unsafe { &mut *(segments.add(index)) }
}

#[allow(dead_code)]
pub fn set_segment(index: usize, base: u32, limit: u32, flag: u8, access: u8) {
	let segment = get_segment(index);

	segment.set_base(base);
	segment.set_limit(limit);
	segment.set_access(access);
	segment.set_flag(flag);
}

#[macro_export]
macro_rules! reload_gdt {
	() => (
		core::arch::asm!("lgdt [{}]", in(reg) (gdt_desc as usize + KERNEL_BASE));
		core::arch::asm!("ljmp $0x08, $2f",
			"2:",
			"movw $0x10, %ax",
			"movw %ax, %ds",
			"movw %ax, %es",
			"movw %ax, %fs",
			"movw %ax, %gs",
			"movw %ax, %ss", options(att_syntax));
	);
}

#[macro_export]
macro_rules! reload_tss {
	() => {
		unsafe {
			core::arch::asm!("mov ax, 0x38", "ltr ax");
		}
	};
}
