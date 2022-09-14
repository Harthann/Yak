use core::fmt;
use crate::kprintln;

mod segment_descriptor;
use segment_descriptor::SegmentDescriptor;

pub const KERNEL_BASE: usize = 0xc0000000;

extern "C" {
	pub fn gdt_start();
	pub fn gdt_desc();
}

#[repr(packed)]
pub struct GDTR {
	size: u16,
	offset: u32
}

pub unsafe fn update_gdtr() {
	let gdtr: &mut GDTR = &mut *((gdt_desc as usize + KERNEL_BASE) as *mut _);
	gdtr.offset = (gdt_start as usize + KERNEL_BASE) as u32;
}

pub struct Gdt {
	pub null:   SegmentDescriptor,
	pub kcode:	SegmentDescriptor,
	pub kdata:	SegmentDescriptor,
	pub kstack: SegmentDescriptor,
	pub ucode:	SegmentDescriptor,
	pub udata:	SegmentDescriptor,
	pub ustack: SegmentDescriptor,
	pub tss:	SegmentDescriptor
}

impl Gdt {

	#[inline]
	pub fn get() -> &'static Gdt {
		unsafe {
			&(*((gdt_start as usize + KERNEL_BASE) as *const Gdt))
		}
	}

	#[inline]
	pub fn get_mut() -> &'static mut Gdt {
		unsafe {
			&mut (*((gdt_start as usize + KERNEL_BASE) as *mut Gdt))
		}
	}

}

impl fmt::Display for Gdt {
	
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "
Segment Kcode:\n{}\n
Segment Kdata:\n{}\n
Segment Kstack:\n{}\n
Segment Ucode:\n{}\n
Segment Udata:\n{}\n
Segment Ustack:\n{}\n

Segment TSS:\n{}\n",
		self.kcode, self.kdata, self.kstack,
		self.ucode, self.udata, self.ustack,
		self.tss)
	}

}

#[allow(dead_code)]
pub fn print_gdt() {
	let mut segments: *mut SegmentDescriptor = (gdt_start as usize + KERNEL_BASE) as *mut _;
	let mut id = 0;
	let end = (gdt_desc as usize + KERNEL_BASE) as *mut SegmentDescriptor;
	while segments < end {
		let segment = unsafe{&*segments};
		kprintln!("\nSegment {}:\n{}", id, segment);
		segments = unsafe{segments.add(1)};
		id += 1;
	}
}

#[allow(dead_code)]
pub fn get_segment(index: usize) -> &'static mut SegmentDescriptor{
	let segments: *mut SegmentDescriptor = (gdt_start as usize + KERNEL_BASE) as *mut _;
	unsafe{&mut *(segments.add(index))}
}

#[allow(dead_code)]
pub fn set_segment(index:usize, base: u32, limit:u32, flag:u8, access:u8) {
	let segment = get_segment(index);

	segment.set_base(base);
	segment.set_limit(limit);
	segment.set_access(access);
	segment.set_flag(flag);
}

#[macro_export]
macro_rules! reload_tss {
	() => {
		unsafe {
			core::arch::asm("mov ax, 0x28",
							"ltr ax");
		}
	}
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
