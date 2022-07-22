use crate::{BumpAllocator};
use core::fmt;
use core::alloc::{
GlobalAlloc,
Layout
};
use core::ops::{Deref};

use core::ptr::{self, Unique, NonNull};
use crate::allocator::ALLOCATOR;

#[derive(Debug, Clone, Copy)]
pub struct Box<T: ?Sized, A: GlobalAlloc + 'static = BumpAllocator>(NonNull<T>, &'static A);
// {
//	ptr: NonNull<T>,
//	allocator: *mut A
//}

impl<T> Box<T> {

	pub fn new(x: T) -> Self {
		unsafe{ Self::new_in(x, &ALLOCATOR) }
	}

	pub fn try_new(x:T) -> Result<Self, ()> {
		unsafe{ Self::try_new_in(x, &ALLOCATOR) }
	}

	pub fn as_ptr(&self) -> *const T {
		self.0.as_ptr()
	}

}

impl<T, A: GlobalAlloc> Box<T, A> {
	
	pub fn new_in(x: T, alloc: &'static A) -> Self {
		let mut ptr = {
			let size_var: usize = core::mem::size_of::<T>();
			if size_var == 0 {
				NonNull::dangling()
			} else {
				unsafe {
					NonNull::new(alloc.alloc(Layout::from_size_align(size_var, 8).unwrap()) as *mut T).expect("Allocator failed, probably OOM")
				}
			}
		};
		unsafe {
			*ptr.as_mut() = x;
		}
		Box (ptr,alloc)
	}

	pub fn try_new_in(x:T, alloc: &'static A) -> Result<Self, ()> {
		let mut res = {
			let size_var: usize = core::mem::size_of::<T>();
			if size_var == 0 {
				Some(NonNull::dangling())
			} else {
				unsafe {
					NonNull::new(alloc.alloc(Layout::from_size_align(size_var, 8).unwrap()) as *mut T)				}
			}
		};
		match res {
			None => Err(()),
			Some(T) => {
				let mut ptr = res.unwrap();
				unsafe { *ptr.as_mut() = x; }
				Ok(Box (ptr,alloc))
			}
		}
	}
}


impl<T, A: GlobalAlloc> Deref for Box<T, A> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		unsafe{ self.0.as_ref() }
	}
}


impl<T: fmt::Display> fmt::Display for  Box<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:p} -> {}", self.as_ptr(), **self)
	}
}
