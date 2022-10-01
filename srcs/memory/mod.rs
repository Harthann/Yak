pub mod allocator;
pub mod paging;

use crate::memory::allocator::AllocatorInit;
use crate::memory::paging::{alloc_pages_at_addr, kalloc_pages_at_addr, alloc_pages, kalloc_pages};

pub type VirtAddr = u32;
pub type PhysAddr = u32;

pub fn init_memory_addr(addr: VirtAddr, size: usize, flags: u32, kphys: bool) -> Result<VirtAddr, ()>{
	assert!(size % 4096 == 0, "size must be a multiple of 4096");
	let nb_page: usize = size / 4096;

	if kphys {
		kalloc_pages_at_addr(addr, nb_page, flags)
	} else {
		alloc_pages_at_addr(addr, nb_page, flags)
	}
}

pub fn init_memory(size: usize, flags: u32, kphys: bool) -> Result<VirtAddr, ()> {
	assert!(size % 4096 == 0, "size must be a multiple of 4096");
	let nb_page: usize = size / 4096;

	if kphys {
		kalloc_pages(nb_page, flags)
	} else {
		alloc_pages(nb_page, flags)
	}
}

enum TypeZone{
	Unassigned,
	Stack,
	Heap
}

pub struct MemoryZone {
	offset: VirtAddr,
	type_zone: TypeZone,
	size: usize,
	flags: u32,
	kphys: bool
}

impl MemoryZone {
	pub const fn new() -> Self {
		Self {
			offset: 0,
			type_zone: TypeZone::Unassigned,
			size: 0,
			flags: 0,
			kphys: false
		}
	}
}

pub trait Heap {
	fn init_addr(offset: VirtAddr, size: usize, flags: u32,
kphys: bool, allocator: &mut dyn AllocatorInit) -> MemoryZone;
	fn init(size: usize, flags: u32, kphys: bool,
allocator: &mut dyn AllocatorInit) -> MemoryZone;
}

pub trait Stack {
	fn init_addr(offset: VirtAddr, size: usize, flags: u32,
kphys: bool) -> MemoryZone;
	fn init(size: usize, flags: u32, kphys: bool) -> MemoryZone;
}

impl Heap for MemoryZone {
	fn init_addr(offset: VirtAddr, size: usize, flags: u32,
kphys: bool, allocator: &mut dyn AllocatorInit) -> MemoryZone {
		let heap: MemoryZone = MemoryZone {
			offset: offset,
			type_zone: TypeZone::Heap,
			size: size,
			flags: flags,
			kphys: kphys
		};
		init_memory_addr(offset, size, flags, kphys).expect("unable to allocate pages for heap");
		unsafe{allocator.init(offset, size)};
		heap
	}

	fn init(size: usize, flags: u32, kphys: bool, allocator: &mut dyn AllocatorInit) -> MemoryZone{
		let mut heap: MemoryZone = MemoryZone {
			offset: 0,
			type_zone: TypeZone::Heap,
			size: size,
			flags: flags,
			kphys: kphys
		};
		heap.offset = init_memory(size, flags, kphys).expect("unable to allocate pages for heap");
		unsafe{allocator.init(heap.offset, size)};
		heap
	}
}

impl Stack for MemoryZone {
	fn init_addr(offset: VirtAddr, size: usize, flags: u32,
kphys: bool) -> MemoryZone {
		let stack_bottom: VirtAddr = offset - (size - 1) as u32;

		let stack: MemoryZone = MemoryZone {
			offset: stack_bottom,
			type_zone: TypeZone::Stack,
			size: size,
			flags: flags,
			kphys: kphys
		};
		init_memory_addr(stack_bottom, size, flags, kphys).expect("unable to allocate pages for stack");
		stack
	}

	fn init(size: usize, flags: u32, kphys: bool) -> MemoryZone {
		let mut stack: MemoryZone = MemoryZone {
			offset: 0,
			type_zone: TypeZone::Stack,
			size: size,
			flags: flags,
			kphys: kphys
		};
		stack.offset = init_memory(size, flags, kphys).expect("unable to allocate pages for stack") - (size - 1) as u32;
		stack
	}
}

/* kphys => physically contiguous */
pub fn init_heap(offset: VirtAddr, size: usize, flags: u32, kphys: bool, allocator: &mut dyn AllocatorInit) -> MemoryZone {
	<MemoryZone as Heap>::init_addr(offset, size, flags, kphys, allocator)
}

pub fn init_stack(stack_top: VirtAddr, size: usize, flags: u32, kphys: bool) -> MemoryZone {
	<MemoryZone as Stack>::init_addr(stack_top, size, flags, kphys)
}
