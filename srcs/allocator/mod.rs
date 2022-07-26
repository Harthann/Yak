pub mod linked_list;
pub mod bump;

mod boxed;
pub use boxed::Box;

mod global;
use global::Global;

pub mod kglobal;
pub use kglobal::KGlobal;

use core::alloc::{
GlobalAlloc,
Layout
};
use core::ptr::NonNull;
use crate::ALLOCATOR;

use crate::paging::{VirtAddr, alloc_pages_at_addr, kalloc_pages_at_addr, PAGE_WRITABLE, PAGE_USER};

/*	Trait definitions to inialize a global allocator */
pub trait AllocatorInit: GlobalAlloc {
	unsafe fn init(&mut self, offset: VirtAddr, size: usize);
}

/* Custom trait to define Allocator design */
pub trait Allocator {
	fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, AllocError>;
	fn deallocate(&self, ptr: NonNull<u8>, layout: Layout);
	fn allocate_zeroed(&self, layout: Layout) -> Result<NonNull<u8>, AllocError>;
}

/* Allocation error type definition */
#[derive(Copy, Clone, Debug)]
pub struct AllocError;


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

pub fn init_heap(heap: VirtAddr, size: usize, allocator: &mut dyn AllocatorInit) {
	let nb_page: usize = size / 4096 + (size % 4096 != 0) as usize;
	alloc_pages_at_addr(heap, nb_page, PAGE_WRITABLE | PAGE_USER).expect("unable to allocate pages for heap");
	unsafe{allocator.init(heap, size)};
}

pub fn init_kheap(heap: VirtAddr, size: usize, allocator: &mut dyn AllocatorInit) {
	let nb_page: usize = size / 4096 + (size % 4096 != 0) as usize;
	kalloc_pages_at_addr(heap, nb_page, PAGE_WRITABLE).expect("unable to allocate pages for kheap");
	unsafe{allocator.init(heap, size)};
}
