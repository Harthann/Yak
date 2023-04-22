//! GDT and TSS setup/helpers

use crate::kprintln;
use core::fmt;
pub mod tss;
pub use tss::{init_tss, Tss};

use crate::boot::KERNEL_BASE;

const PRESENT: u8 = 0b10000000;
const USER: u8 = 0b01100000;
const DATA: u8 = 0b00010000;
const CODE: u8 = 0b00011000;
const GROWS_UP: u8 = 0b00000000;
const GROWS_DOWN: u8 = 0b00000100;
const WRITABLE: u8 = 0b00000010;
const READABLE: u8 = 0b00000010;
const NOT_FOR_CPU: u8 = 0b00000001;

#[link_section = ".gdt"]
static mut GDT: [SegmentDescriptor; 8] = [
	// null
	SegmentDescriptor {
		limit:			0x0,
		base:			[0x0, 0x0, 0x0],
		access:			0x0,
		limit_flags:	0b00000000,
		base_end:		0x0,
	},
	// kcode
	SegmentDescriptor {
		limit:			0xffff,
		base:			[0x0, 0x0, 0x0],
		access:			PRESENT | CODE | READABLE,
		limit_flags:	0b11001111,
		base_end:		0x0,
	},
	// kdata
	SegmentDescriptor {
		limit:			0xffff,
		base:			[0x0, 0x0, 0x0],
		access:			PRESENT | DATA | GROWS_UP | WRITABLE,
		limit_flags:	0b11001111,
		base_end:		0x0,
	},
	// kstack
	SegmentDescriptor {
		limit:			0x0,
		base:			[0x0, 0x0, 0x0],
		access:			PRESENT | DATA | GROWS_DOWN | WRITABLE | NOT_FOR_CPU,
		limit_flags:	0b11000000,
		base_end:		0x0,
	},
	// ucode
	SegmentDescriptor {
		limit:			0xffff,
		base:			[0x0, 0x0, 0x0],
		access:			PRESENT | USER | CODE | READABLE,
		limit_flags:	0b11001111,
		base_end:		0x0,
	},
	// udata
	SegmentDescriptor {
		limit:			0xffff,
		base:			[0x0, 0x0, 0x0],
		access:			PRESENT | USER | DATA | GROWS_UP | WRITABLE,
		limit_flags:	0b11001111,
		base_end:		0x0,
	},
	// ustack
	SegmentDescriptor {
		limit:			0x0,
		base:			[0x0, 0x0, 0x0],
		access:			PRESENT | USER | DATA | GROWS_DOWN | WRITABLE | NOT_FOR_CPU,
		limit_flags:	0b11000000,
		base_end:		0x0,
	},
	// task state
	SegmentDescriptor {
		limit:			0x0,
		base:			[0x0, 0x0, 0x0],
		access:			0b11101001,
		limit_flags:	0b00000000,
		base_end:		0x0,
	},
];

#[link_section = ".gdt"]
#[no_mangle]
pub static mut gdt_desc: GDTR = GDTR {
	size: unsafe { core::mem::size_of_val(&GDT) as u16 },
	offset: unsafe { core::ptr::addr_of!(GDT) }
};

#[repr(packed)]
pub struct GDTR {
	size:   u16,
	offset: *const [SegmentDescriptor; 8]
}

pub unsafe fn update_gdtr() {
	let gdtr: &mut GDTR = &mut *(((&gdt_desc as *const _) as usize + KERNEL_BASE) as *mut _);
	gdtr.offset = ((&GDT as *const _) as usize + KERNEL_BASE) as _;
}

#[repr(packed)]
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

#[allow(dead_code)]
pub fn get_segment(index: usize) -> &'static mut SegmentDescriptor {
	unsafe {
		let segments: *mut SegmentDescriptor =
			((&GDT as *const _) as usize + KERNEL_BASE) as *mut _;
		&mut *(segments.add(index))
	}
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
macro_rules! reload_cs {
	() => (
		core::arch::asm!(
			"mov ax, 0x10",
			"mov ds, ax",
			"mov es, ax",
			"mov fs, ax",
			"mov gs, ax",
			"mov ss, ax",
		);
	);
}

#[macro_export]
macro_rules! reload_gdt {
	() => (
		core::arch::asm!("lgdt [{}]", in(reg) ((&$crate::gdt_desc as *const _) as usize + $crate::boot::KERNEL_BASE));
		core::arch::asm!(
			"ljmp $0x08, $2f",
			"2:",
			options(att_syntax)
		);
		$crate::reload_cs!();
	);
}

#[macro_export]
macro_rules! reload_tss {
	() => {
		unsafe {
			core::arch::asm!(
				"mov ax, 0x38",
				"ltr ax"
			);
		}
	};
}
