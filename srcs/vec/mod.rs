use core::ptr::{NonNull};
use core::alloc::{Layout, };
use crate::GLOBAL_ALIGN;
 use crate::allocator::{
Allocator,
AllocError,
Global
};

#[cfg(test)]
pub mod test;

#[derive(Clone, Debug)]
pub struct Vec<T, A: Allocator = crate::allocator::Global> {
	ptr: NonNull<T>,
	capacity: usize,
	len: usize,
	alloc: A
}

pub fn test() {
	let x = Vec::<i32>::with_capacity(5);
}

impl<T> Vec<T> {

	pub fn new() -> Vec<T> {
		Vec {
			ptr: NonNull::<T>::dangling(),
			capacity: 0,
			len: 0,
			alloc: Global
		}
	}

	pub fn with_capacity(capacity: usize) -> Vec<T> {
		Vec {
			ptr: Self::with_capacity_in(capacity, &Global),
			capacity: capacity,
			len: 0,
			alloc: Global
		}
	}
}

impl<T, A: Allocator> Vec<T,A> {

	pub fn capacity(&self) -> usize {
		self.capacity
	}

	pub fn len(&self) -> usize {
		self.len
	}


	pub fn reserve(&mut self, additional: usize) {
		match self.try_reserve(additional) {
			Ok(_) => {},
			Err(_) => panic!("Couldn't reserve more")
		};
	}

	pub fn try_reserve(&mut self, additional: usize) -> Result<(), AllocError> {
		let layout: Layout = Layout::from_size_align(self.capacity(), GLOBAL_ALIGN).unwrap();
		// self.alloc().realloc(self.as_ptr(), layout, self.capacity() + additional);
		todo!()
	}

	pub fn push(&mut self, value: T) {
		if self.len + 1 < self.capacity {
			unsafe {
				self.ptr.as_ptr()
						.add(self.len)
						.write(value);
			}
			self.len += 1;
		} else {
			todo!()
		}
	}

	pub fn pop(&mut self) -> Option<T> {
		if self.len == 0 {
			None
		} else {
			unsafe {
				self.len -= 1;
				Some(core::ptr::read(self.ptr.as_ptr().add(self.len)))
			}
		}
	}

	pub fn insert(&mut self, index: usize, element: T) {
		todo!()
	}

	pub fn remove(&mut self, index: usize) -> T {
		todo!()
	}

	pub fn as_slice(&self) -> &[T] {
		unsafe {
			NonNull::slice_from_raw_parts(self.ptr, self.len).as_ref()
		}
	}

	pub fn as_mut_slice(&mut self) -> &mut [T] {
		unsafe {
			NonNull::slice_from_raw_parts(self.ptr, self.len).as_mut()
		}
	}

	pub fn as_ptr(&self) -> *const T {
		todo!()
	}

	pub fn as_mut_ptr(&mut self) -> *mut T {
		todo!()
	}
}


impl<T, A: Allocator> Vec<T,A> {

	pub fn with_capacity_in(capacity: usize, alloc: &dyn Allocator) -> NonNull<T> {
		match Self::try_alloc(capacity, alloc) {
			Ok(x) => x,
			Err(_) => panic!("Allocation failed")
		}
	}

	fn try_alloc(capacity: usize, alloc: &dyn Allocator) -> Result<NonNull<T>, AllocError> {
		let layout = Layout::from_size_align(capacity, GLOBAL_ALIGN).unwrap();
		match alloc.allocate(layout) {
			Ok(res) => Ok(res.cast()),
			Err(_) => Err(AllocError{})
		}
	}
}

impl<T: core::fmt::Display + core::fmt::Debug, A: Allocator> core::fmt::Display for Vec<T, A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		unsafe {
		write!(f, "Vec: {}\nPtr: {:p}\nCapacity: {}\nLength: {}\nArray: {:?}\n{}"
						, '{', self.ptr, self.capacity, self.len, self.as_slice(),'}')
		}
	}
}

/* TODO!: Trait to be implemented
**	Drop, Deref, DerefMut, AsMut, AsRef, Index, IndexMut, IntoIterator
*/

