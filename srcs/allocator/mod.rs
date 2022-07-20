pub mod linked_list;
pub mod bump;

use core::alloc::Layout;

use linked_list::LinkedListAllocator;
use bump::BumpAllocator;

use crate::paging::alloc_pages_at_addr;
use crate::ALLOCATOR;

extern "C" {
	fn heap();
}

const HEAP_SIZE: usize = 100 * 1024;

pub fn init_heap() {
	alloc_pages_at_addr(heap as u32, HEAP_SIZE / 4096);
	unsafe{ALLOCATOR.init(heap as usize, HEAP_SIZE)};
	unsafe {
		use core::alloc::GlobalAlloc;

		let res = Layout::from_size_align(8, 8);
		if res.is_ok() {
			ALLOCATOR.alloc(res.unwrap());
		}
	}
}
