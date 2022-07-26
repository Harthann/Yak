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

/* kphys => physically contiguous */
pub fn init_heap(heap: VirtAddr, size: usize, flags: u32, kphys: bool, allocator: &mut dyn AllocatorInit) -> VirtAddr {
	init_memory(heap, size, flags, kphys).expect("unable to allocate pages for heap");
	unsafe{allocator.init(heap, size)};
	heap
}

pub fn init_stack(stack_top: VirtAddr, size: usize, flags: u32, kphys: bool) -> VirtAddr {
	let stack_bottom: VirtAddr = stack_top - (size - 1) as u32;
	init_memory(stack_bottom, size, flags, kphys).expect("unable to allocate pages for stack");
	stack_top
}
