use crate::{BumpAllocator};
use core::alloc::{
GlobalAlloc,
Layout
};
use core::ptr::{self, Unique, NonNull};


#[lang = "owned_box"]
#[fundamental]
pub struct Box<T: ?Sized> {
	ptr: NonNull<T>
}

impl<T> Box<T> {
	
	pub fn new(x: T) -> Self {
		let ptr: = {
			let size_var: usize = core::mem::size_of::<T>();
			if size_var == 0 {
				NonNull::dangling()
			}
			crate::allocator::ALLOCATOR.alloc(Layout::from_size_align(size_var, 8).unwrap()) as *mut T
		};
		Box {
			ptr: ptr
		}
	}
}
