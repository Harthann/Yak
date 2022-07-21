pub mod linked_list;
pub mod bump;
pub mod boxed;

use core::alloc::Layout;

use linked_list::LinkedListAllocator;
use bump::BumpAllocator;

use crate::paging::alloc_pages_at_addr;
use crate::ALLOCATOR;

extern "C" {
	fn heap();
}

const HEAP_SIZE: usize = 100 * 1024;

#[lang = "exchange_malloc"]
#[no_mangle]
pub fn allocate(size: usize, _align: usize) -> *mut u8 {
	crate::kprintln!("Received allocation of {} bytes, aligned {}", size, _align);
	0xffff as *mut u8
}

#[no_mangle]
pub fn deallocate(ptr: *mut u8, size: usize, _align: usize) {
	crate::kprintln!("Received deallocation of {} bytes, aligned {} at {:p}", size, _align, ptr);
}

pub fn init_heap() {
	let nb_page: usize = if HEAP_SIZE % 4096 == 0 {HEAP_SIZE / 4096} else {HEAP_SIZE / 4096 + 1};
	alloc_pages_at_addr(heap as u32, nb_page);
	unsafe{ALLOCATOR.init(heap as usize, HEAP_SIZE)};
	unsafe {
		use core::alloc::GlobalAlloc;

		let res = Layout::from_size_align(8, 8);
		if res.is_ok() {
			ALLOCATOR.alloc(res.unwrap());
		}
	}
}
