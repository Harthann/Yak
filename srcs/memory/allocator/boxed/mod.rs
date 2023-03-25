use core::alloc::Layout;
use core::fmt;
use core::ops::{Deref, DerefMut};

use crate::memory::allocator::{
	AllocError,
	Allocator,
	// Global,
	KGlobal
};
use core::ptr::NonNull;

#[cfg(test)]
pub mod test;

const GLOBAL_ALIGN: usize = 8;

#[derive(Debug, Clone)]
pub struct Box<T: ?Sized, A: Allocator = KGlobal> {
	ptr:    NonNull<T>,
	alloc:  A,
	layout: Layout
}

impl<T> Box<T> {
	pub fn new(x: T) -> Box<T, KGlobal> {
		Self::new_in(x, KGlobal)
	}

	pub fn try_new(x: T) -> Result<Box<T, KGlobal>, AllocError> {
		Self::try_new_in(x, KGlobal)
	}

	pub fn knew(x: T) -> Box<T, KGlobal> {
		Box::<T, KGlobal>::new_in(x, KGlobal)
	}

	pub fn ktry_new(x: T) -> Result<Box<T, KGlobal>, AllocError> {
		Box::<T, KGlobal>::try_new_in(x, KGlobal)
	}

	pub const unsafe fn from_raw(x: *mut T) -> Box<T, KGlobal> {
		Box::<T, KGlobal>::from_raw_in(x, KGlobal)
	}
}

impl<T, A: Allocator> Box<T, A> {
	pub fn new_in(x: T, alloc: A) -> Self {
		let boxed = Self::new_uninit_in(alloc);
		Box::write(boxed, x)
	}

	pub fn new_uninit_in(alloc: A) -> Self {
		match Self::try_new_uninit_in(alloc) {
			Ok(res) => res,
			Err(_) => panic!("Allocation failed")
		}
	}

	pub fn try_new_in(x: T, alloc: A) -> Result<Self, AllocError> {
		let ptr = Self::try_new_uninit_in(alloc)?;
		Ok(Box::write(ptr, x))
	}

	pub fn try_new_uninit_in(alloc: A) -> Result<Self, AllocError> {
		let mut layout: Layout = Layout::new::<T>();
		let res = {
			let size_var: usize = core::mem::size_of::<T>();
			if size_var == 0 {
				Some(NonNull::dangling())
			} else {
				layout =
					Layout::from_size_align(size_var, GLOBAL_ALIGN).unwrap();
				Some(alloc.allocate(layout)?.cast())
			}
		};
		match res {
			None => Err(AllocError {}),
			Some(ptr) => Ok(Box { ptr: ptr, alloc: alloc, layout: layout })
		}
	}

	pub const unsafe fn from_raw_in(x: *mut T, alloc: A) -> Box<T, A> {
		Box {
			ptr:    NonNull::new_unchecked(x as *mut _),
			alloc:  alloc,
			layout: Layout::new::<T>()
		}
	}

	pub fn write(mut boxed: Self, value: T) -> Box<T, A> {
		*boxed = value;
		boxed
	}
}

impl<T, A: Allocator> Deref for Box<T, A> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		unsafe { self.ptr.as_ref() }
	}
}

impl<T, A: Allocator> DerefMut for Box<T, A> {
	fn deref_mut(&mut self) -> &mut T {
		unsafe { self.ptr.as_mut() }
	}
}

impl<T: ?Sized, A: Allocator> Drop for Box<T, A> {
	fn drop(&mut self) {
		self.alloc.deallocate(self.ptr.cast(), self.layout);
	}
}

impl<T: ?Sized, A: Allocator> AsMut<T> for Box<T, A> {
	fn as_mut(&mut self) -> &mut T {
		unsafe { self.ptr.as_mut() }
	}
}

impl<T: ?Sized, A: Allocator> AsRef<T> for Box<T, A> {
	fn as_ref(&self) -> &T {
		unsafe { self.ptr.as_ref() }
	}
}

impl<T: fmt::Display, A: Allocator> fmt::Display for Box<T, A> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:p} -> {}", self.ptr.as_ptr(), **self)
	}
}
