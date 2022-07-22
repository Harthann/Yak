use crate::{BumpAllocator};
use core::alloc::{
GlobalAlloc,
Layout
};
use core::ptr::{self, Unique, NonNull};

#[derive(Debug, Clone, Copy)]
pub struct Box<T: ?Sized> {
	ptr: NonNull<T>
}

impl<T> Box<T> {
	
	pub fn new(x: T) -> Self {
		let ptr = {
			let size_var: usize = core::mem::size_of::<T>();
			if size_var == 0 {
				NonNull::dangling()
			} else {
				unsafe {
					NonNull::new(crate::allocator::ALLOCATOR.alloc(Layout::from_size_align(size_var, 1).unwrap()) as *mut T).unwrap()
				}
			}
		};
		Box {
			ptr: ptr
		}
	}
}
