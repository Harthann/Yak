use crate::{BumpAllocator, LinkedListAllocator};
use core::fmt;
use core::alloc::{
GlobalAlloc,
Layout
};
use core::ops::{Deref, DerefMut};

use core::ptr::{self, Unique, NonNull};
use crate::allocator::{ALLOCATOR,
AllocError,
Allocator,
Global};

pub mod test;

const GLOBAL_ALIGN: usize = 8;

#[derive(Debug, Clone)]
pub struct Box<T: ?Sized, A: Allocator = Global> {
	ptr: NonNull<T>,
	alloc: A,
	layout: Layout
}

impl<T> Box<T> {

	pub fn new(x: T) -> Self {
		unsafe{ Self::new_in(x, Global) }
	}

	pub fn try_new(x:T) -> Result<Self, AllocError> {
		unsafe{ Self::try_new_in(x, Global) }
	}


	pub fn knew(x: T) -> Self {
		unsafe{ Self::knew_in(x, Global) }
	}

	pub fn ktry_new(x:T) -> Result<Self, AllocError> {
		unsafe{ Self::ktry_new_in(x, Global) }
	}

}

/* This block implement virtual memory allocation for box */
impl<T, A: Allocator> Box<T, A> {
	
	pub fn new_in(x: T, alloc: A) -> Self {
		let boxed = Self::new_uninit_in(alloc);
		Box::write(boxed, x)
	}

	pub fn new_uninit_in(alloc: A) -> Self {
		match Self::try_new_uninit_in(alloc) {
			Ok(res) => res,
			Err(_) => panic!("Allocation failed"),
		}
	}

	pub fn try_new_in(x:T, alloc: A) -> Result<Self, AllocError> {
		let mut ptr = Self::try_new_uninit_in(alloc)?;
		Ok(Box::write(ptr, x))
	}

	pub fn try_new_uninit_in(alloc: A) -> Result<Self, AllocError> {
		let mut layout: Layout = Layout::new::<T>();
		let mut res = {
			let size_var: usize = core::mem::size_of::<T>();
			if size_var == 0 {
				Some(NonNull::dangling())
			} else {
				unsafe {
					layout = Layout::from_size_align(size_var, GLOBAL_ALIGN).unwrap();
					Some(alloc.allocate(layout)?.cast()) }
			}
		};
		match res {
			None => Err(AllocError{}),
			Some(T) => {
				let mut ptr = res.unwrap();
				Ok(Box {
						ptr: ptr,
						alloc: alloc,
						layout: layout
				})
			}
		}
	}

	pub fn write(mut boxed: Self, value: T) -> Box<T, A> {
		*boxed = value;
		boxed
	}
}

/* This block implement physical memory allocation for box */
impl<T, A: Allocator> Box<T, A> {
	
	pub fn knew_in(x: T, alloc: A) -> Self {
		let boxed = Self::new_uninit_in(alloc);
		Box::write(boxed, x)
	}

	pub fn knew_uninit_in(alloc: A) -> Self {
		match Self::try_new_uninit_in(alloc) {
			Ok(res) => res,
			Err(_) => panic!("Allocation failed"),
		}
	}

	pub fn ktry_new_in(x:T, alloc: A) -> Result<Self, AllocError> {
		let mut ptr = Self::try_new_uninit_in(alloc)?;
		Ok(Box::write(ptr, x))
	}

	pub fn ktry_new_uninit_in(alloc: A) -> Result<Self, AllocError> {
		let mut layout: Layout = Layout::new::<T>();
		let mut res = {
			let size_var: usize = core::mem::size_of::<T>();
			if size_var == 0 {
				Some(NonNull::dangling())
			} else {
				unsafe {
					layout = Layout::from_size_align(size_var, GLOBAL_ALIGN).unwrap();
					Some(alloc.kallocate(layout)?.cast()) }
			}
		};
		match res {
			None => Err(AllocError{}),
			Some(T) => {
				let mut ptr = res.unwrap();
				Ok(Box {
						ptr: ptr,
						alloc: alloc,
						layout: layout
				})
			}
		}
	}
}



impl<T, A: Allocator> Deref for Box<T, A> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		unsafe{ self.ptr.as_ref() }
	}
}

impl<T, A: Allocator> DerefMut for Box<T, A> {

	fn deref_mut(&mut self) -> &mut T {
		unsafe{ self.ptr.as_mut() }
	}
}

impl<T: ?Sized, A: Allocator> Drop for Box<T, A> {
	fn drop (&mut self) {
		unsafe{ self.alloc.deallocate(self.ptr.cast(), self.layout) };
	}
}

impl<T: ?Sized, A: Allocator> AsMut<T> for Box<T,A> {
	fn as_mut(&mut self) -> &mut T {
		unsafe{ self.ptr.as_mut() }
	}
}

impl<T: ?Sized, A: Allocator> AsRef<T> for Box<T,A> {
	fn as_ref(&self) -> &T {
		unsafe{ self.ptr.as_ref() }
	}
}

impl<T: fmt::Display> fmt::Display for  Box<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:p} -> {}", self.ptr.as_ptr(), **self)
	}
}


