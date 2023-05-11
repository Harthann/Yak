//! Allocator, Box and Pagination

pub mod allocator;
#[macro_use]
pub mod paging;

use crate::memory::paging::{
	alloc_pages,
	alloc_pages_at_addr,
	kalloc_pages,
	kalloc_pages_at_addr
};

pub type VirtAddr = u32;
pub type PhysAddr = u32;

pub fn init_memory_addr(
	addr: VirtAddr,
	size: usize,
	flags: u32,
	kphys: bool
) -> Result<VirtAddr, ()> {
	assert!(size % 4096 == 0, "size must be a multiple of 4096");
	let nb_page: usize = size / 4096;

	if kphys {
		kalloc_pages_at_addr(addr, nb_page, flags)
	} else {
		alloc_pages_at_addr(addr, nb_page, flags)
	}
}

pub fn init_memory(
	size: usize,
	flags: u32,
	kphys: bool
) -> Result<VirtAddr, ()> {
	assert!(size % 4096 == 0, "size must be a multiple of 4096");
	let nb_page: usize = size / 4096;

	if kphys {
		kalloc_pages(nb_page, flags)
	} else {
		alloc_pages(nb_page, flags)
	}
}

#[derive(Clone, Copy)]
pub enum TypeZone {
	Unassigned,
	Stack,
	Heap
}

#[derive(Clone, Copy)]
pub struct MemoryZone {
	pub offset:    VirtAddr,
	pub type_zone: TypeZone,
	pub size:      usize,
	pub flags:     u32,
	pub kphys:     bool
}

impl MemoryZone {
	pub const fn new() -> Self {
		Self {
			offset:    0,
			type_zone: TypeZone::Unassigned,
			size:      0,
			flags:     0,
			kphys:     false
		}
	}
}

impl MemoryZone {
	pub fn init_addr(
		offset: VirtAddr,
        ztype:  TypeZone,
		size:   usize,
		flags:  u32,
		kphys:  bool
	) -> MemoryZone {
		let mut mz: MemoryZone = MemoryZone {
			offset,
			type_zone: ztype,
			size,
			flags,
			kphys
		};
		mz.offset = init_memory_addr(offset, size, flags, kphys)
			.expect("unable to allocate pages for stack");
        // crate::dprintln!("MemoryZone create: {}", self);
		mz
	}

	pub fn init(ztype: TypeZone, size: usize, flags: u32, kphys: bool) -> MemoryZone {
		let mut stack: MemoryZone = MemoryZone {
			offset:    0,
			type_zone: ztype,
			size,
			flags,
			kphys
		};
		stack.offset = init_memory(size, flags, kphys)
			.expect("unable to allocate pages for stack");
        // crate::dprintln!("MemoryZone create: {}", self);
		stack
	}
}
