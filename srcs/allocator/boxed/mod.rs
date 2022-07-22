use crate::{BumpAllocator, LinkedListAllocator};
use core::fmt;
use core::alloc::{
GlobalAlloc,
Layout
};
use core::ops::{Deref, DerefMut};

use core::ptr::{self, Unique, NonNull};
use crate::allocator::ALLOCATOR;

pub mod test;

const GLOBAL_ALIGN: usize = 8;

#[derive(Debug, Clone)]
pub struct Box<T: ?Sized, A: GlobalAlloc + 'static = LinkedListAllocator> {
	ptr: NonNull<T>,
	alloc: &'static A,
	size: usize
}

impl<T> Box<T> {

	pub fn new(x: T) -> Self {
		unsafe{ Self::new_in(x, &ALLOCATOR) }
	}

	pub fn try_new(x:T) -> Result<Self, ()> {
		unsafe{ Self::try_new_in(x, &ALLOCATOR) }
	}


}

impl<T, A: GlobalAlloc> Box<T, A> {
	
	pub fn new_in(x: T, alloc: &'static A) -> Self {
		let size_var: usize = core::mem::size_of::<T>();
		let mut ptr = {
			if size_var == 0 {
				NonNull::dangling()
			} else {
				unsafe {
					NonNull::new(alloc.alloc(Layout::from_size_align(size_var, GLOBAL_ALIGN).unwrap()) as *mut T).expect("Allocator failed, probably OOM")
				}
			}
		};
		unsafe {
			*ptr.as_mut() = x;
		}
		Box {
			ptr: ptr,
			alloc: alloc,
			size: size_var
		}
	}

	pub fn try_new_in(x:T, alloc: &'static A) -> Result<Self, ()> {
		let mut res = {
			let size_var: usize = core::mem::size_of::<T>();
			if size_var == 0 {
				Some(NonNull::dangling())
			} else {
				unsafe {
					NonNull::new(alloc.alloc(Layout::from_size_align(size_var, GLOBAL_ALIGN).unwrap()) as *mut T)				}
			}
		};
		match res {
			None => Err(()),
			Some(T) => {
				let mut ptr = res.unwrap();
				unsafe { *ptr.as_mut() = x; }
				Ok(Box {
						ptr: ptr,
						alloc: alloc,
						size: core::mem::size_of::<T>()
				})
			}
		}
	}

	pub fn write(mut boxed: Self, value: T) -> Box<T, A> {
		*boxed = value;
		boxed
	}
}

impl<T, A: GlobalAlloc> Deref for Box<T, A> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		unsafe{ self.ptr.as_ref() }
	}
}

impl<T, A: GlobalAlloc> DerefMut for Box<T, A> {

	fn deref_mut(&mut self) -> &mut T {
		unsafe{ self.ptr.as_mut() }
	}
}

impl<T: ?Sized, A: GlobalAlloc> Drop for Box<T, A> {
	fn drop (&mut self) {
		unsafe{ self.alloc.dealloc(self.ptr.as_ptr() as *mut u8, Layout::from_size_align(self.size, GLOBAL_ALIGN).unwrap()) };
	}
}

impl<T: ?Sized, A: GlobalAlloc> AsMut<T> for Box<T,A> {
	fn as_mut(&mut self) -> &mut T {
		unsafe{ self.ptr.as_mut() }
	}
}

impl<T: ?Sized, A: GlobalAlloc> AsRef<T> for Box<T,A> {
	fn as_ref(&self) -> &T {
		unsafe{ self.ptr.as_ref() }
	}
}

impl<T: fmt::Display> fmt::Display for  Box<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:p} -> {}", self.ptr.as_ptr(), **self)
	}
}


