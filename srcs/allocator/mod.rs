pub mod linked_list;
pub mod bump;
pub mod boxed;

use core::alloc::GlobalAlloc;
use crate::ALLOCATOR;

use crate::paging::{VirtAddr, alloc_pages_at_addr, kalloc_pages_at_addr};

pub trait Allocator: GlobalAlloc {
	unsafe fn init(&mut self, offset: VirtAddr, size: usize);
}

/*
	Align is a power of 2 so if we substract 1 his binary representation contain
	only 1 (0b1111..). We can then AND it with addr to get the right alignment.
	(add it with the addr to get the next alignment - align_up())
*/
fn align_down(addr: VirtAddr, align: usize) -> VirtAddr {
	if align.is_power_of_two() {
		addr & !(align as u32 - 1)
	} else if align == 0 {
		addr
	} else {
		panic!("`align` must be a power of 2");
	}
}

fn align_up(addr: VirtAddr, align: usize) -> VirtAddr {
	(addr + align as u32 - 1) & !(align as u32 - 1)
}

pub fn init_heap(heap: VirtAddr, size: usize, allocator: &mut dyn Allocator) {
	let nb_page: usize = size / 4096 + (size % 4096 != 0) as usize;
	alloc_pages_at_addr(heap, nb_page).expect("unable to allocate pages for heap");
	unsafe{allocator.init(heap, size)};
}

pub fn init_kheap(heap: VirtAddr, size: usize,  allocator: &mut dyn Allocator) {
	let nb_page: usize = size / 4096 + (size % 4096 != 0) as usize;
	kalloc_pages_at_addr(heap, nb_page).expect("unable to allocate pages for kheap");
	unsafe{allocator.init(heap, size)};
}
