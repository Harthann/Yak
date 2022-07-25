pub mod linked_list;
pub mod bump;
pub mod boxed;

extern "Rust" {
    // These are the magic symbols to call the global allocator.  rustc generates
    // them to call `__rg_alloc` etc. if there is a `#[global_allocator]` attribute
    // (the code expanding that attribute macro generates those functions), or to call
    // the default implementations in libstd (`__rdl_alloc` etc. in `library/std/src/alloc.rs`)
    // otherwise.
    // The rustc fork of LLVM also special-cases these function names to be able to optimize them
    // like `malloc`, `realloc`, and `free`, respectively.
    #[rustc_allocator]
    #[rustc_allocator_nounwind]
    fn __rust_alloc(size: usize, align: usize) -> *mut u8;
    #[rustc_allocator_nounwind]
    fn __rust_dealloc(ptr: *mut u8, size: usize, align: usize);
    #[rustc_allocator_nounwind]
    fn __rust_realloc(ptr: *mut u8, old_size: usize, align: usize, new_size: usize) -> *mut u8;
    #[rustc_allocator_nounwind]
    fn __rust_alloc_zeroed(size: usize, align: usize) -> *mut u8;
}


use core::alloc::{Layout, GlobalAlloc};
use crate::ALLOCATOR;

use linked_list::LinkedListAllocator;
use bump::BumpAllocator;

pub type GlobAlloc = LinkedListAllocator;

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
	alloc_pages_at_addr(heap, nb_page);
	unsafe{allocator.init(heap, size)};
}

pub fn init_kheap(heap: VirtAddr, size: usize,  allocator: &mut dyn Allocator) {
	let nb_page: usize = size / 4096 + (size % 4096 != 0) as usize;
	kalloc_pages_at_addr(heap, nb_page);
	unsafe{allocator.init(heap, size)};
}
