#![cfg_attr(feature = "cross-compiled", no_std)]

pub mod linked_list;
pub mod bump;

mod boxed;
pub use boxed::Box;

mod global;
pub use global::Global;

pub mod kglobal;
pub use kglobal::KGlobal;

use core::alloc::{
GlobalAlloc,
Layout
};
use core::ptr::NonNull;

pub type VirtAddr = u32;
//use crate::memory::VirtAddr;

/*	Trait definitions to inialize a global allocator */
pub trait AllocatorInit: GlobalAlloc {
	unsafe fn init(&mut self, offset: VirtAddr, size: usize);
}

/* Custom trait to define Allocator design */
pub trait Allocator {
	fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, AllocError>;
	fn deallocate(&self, ptr: NonNull<u8>, layout: Layout);
	fn allocate_zeroed(&self, layout: Layout) -> Result<NonNull<u8>, AllocError>;
	fn realloc(&self, ptr: NonNull<u8>, old: Layout, new_size: usize) -> Result<NonNull<u8>, AllocError>;
}

/* Allocation error type definition */
#[derive(Copy, Clone, Debug, PartialEq)]
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
