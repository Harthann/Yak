pub mod allocator;
pub mod paging;

use crate::memory::allocator::AllocatorInit;
use crate::memory::paging::{alloc_pages_at_addr, kalloc_pages_at_addr};

pub type VirtAddr = u32;
pub type PhysAddr = u32;

pub fn init_memory(addr: VirtAddr, size: usize, flags: u32, kphys: bool) -> Result<VirtAddr, ()>{
	assert!(size % 4096 == 0, "size must be a multiple of 4096");
	let nb_page: usize = size / 4096;

	if kphys {
		kalloc_pages_at_addr(addr, nb_page, flags)
	} else {
		alloc_pages_at_addr(addr, nb_page, flags)
	}
}

enum TypeZone{
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

pub trait Heap {
	fn init(offset: VirtAddr, size: usize, flags: u32,
kphys: bool, allocator: &mut dyn AllocatorInit) -> MemoryZone;
}

pub trait Stack {
	fn init(offset: VirtAddr, size: usize, flags: u32,
kphys: bool) -> MemoryZone;
}

impl Heap for MemoryZone {
	fn init(offset: VirtAddr, size: usize, flags: u32,
kphys: bool, allocator: &mut dyn AllocatorInit) -> MemoryZone {
		let heap: MemoryZone = MemoryZone {
			offset: offset,
			type_zone: TypeZone::Heap,
			size: size,
			flags: flags,
			kphys: kphys
		};
		init_memory(offset, size, flags, kphys).expect("unable to allocate pages for heap");
		unsafe{allocator.init(offset, size)};
		heap
	}
}

impl Stack for MemoryZone {
	fn init(offset: VirtAddr, size: usize, flags: u32,
kphys: bool) -> MemoryZone {
		let stack_bottom: VirtAddr = offset - (size - 1) as u32;

		let stack: MemoryZone = MemoryZone {
			offset: stack_bottom,
			type_zone: TypeZone::Stack,
			size: size,
			flags: flags,
			kphys: kphys
		};
		init_memory(stack_bottom, size, flags, kphys).expect("unable to allocate pages for stack");
		stack
	}
}

/* kphys => physically contiguous */
pub fn init_heap(offset: VirtAddr, size: usize, flags: u32, kphys: bool, allocator: &mut dyn AllocatorInit) -> MemoryZone {
	<MemoryZone as Heap>::init(offset, size, flags, kphys, allocator)
}

pub fn init_stack(stack_top: VirtAddr, size: usize, flags: u32, kphys: bool) -> MemoryZone {
	<MemoryZone as Stack>::init(stack_top, size, flags, kphys)
}
