pub mod linked_list;
pub mod bump;

use core::alloc::{Layout, GlobalAlloc};

use linked_list::LinkedListAllocator;
use bump::BumpAllocator;

use crate::paging::{VirtAddr, alloc_pages_at_addr, kalloc_pages_at_addr};

const HEAP_SIZE: usize = 100 * 1024;
const KHEAP_SIZE: usize = 100 * 1024;

fn align_up(addr: VirtAddr, align: usize) -> VirtAddr {
	(addr + align as u32 - 1) & !(align as u32 - 1)
}

/*
pub fn init_heap(heap: u32, allocator) {
	let nb_page: usize = if HEAP_SIZE % 4096 == 0 {HEAP_SIZE / 4096} else {HEAP_SIZE / 4096 + 1};
	alloc_pages_at_addr(heap, nb_page);
	unsafe{allocator.init(heap as usize, HEAP_SIZE)};
	/* TESTS */
	unsafe {
		use core::alloc::GlobalAlloc;

		let res = Layout::from_size_align(8, 8);
		if res.is_ok() {
			allocator.alloc(res.unwrap());
		}
	}
}
*/

pub trait Allocator: GlobalAlloc {
	unsafe fn init(&mut self, offset: VirtAddr, size: usize);
}

pub fn init_kheap(heap: VirtAddr, allocator: &mut dyn Allocator) {
	let nb_page: usize = if KHEAP_SIZE % 4096 == 0 {KHEAP_SIZE / 4096} else {KHEAP_SIZE / 4096 + 1};
	kalloc_pages_at_addr(heap, nb_page);
	unsafe{allocator.init(heap, KHEAP_SIZE)};
	/* TESTS */
	unsafe {
		let res = Layout::from_size_align(8, 8);
		if res.is_ok() {
			allocator.alloc(res.unwrap());
		}
	}
}
