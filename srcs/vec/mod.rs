use core::ptr::{NonNull};
use core::alloc::{Layout, };
use crate::GLOBAL_ALIGN;
 use crate::memory::allocator::{
Allocator,
AllocError,
Global
};

#[cfg(test)]
pub mod test;

#[derive(Clone, Debug)]
pub struct Vec<T, A: Allocator = Global> {
	ptr: Option<NonNull<T>>,
	capacity: usize,
	len: usize,
	alloc: A
}

pub fn test() {
	use crate::kprintln;
	
let mut x: Vec<u32> = Vec::new();

	kprintln!("x: {}\nx.capacity: {}\nx.len: {}", x, x.capacity(), x.len());
	x.reserve(10);
	kprintln!("x.capacity: {}\nx.len: {}", x.capacity(), x.len());
}

impl<T> Vec<T> {

	pub fn new() -> Vec<T> {
		Vec {
			ptr: None,
			capacity: 0,
			len: 0,
			alloc: Global
		}
	}

	pub fn with_capacity(capacity: usize) -> Vec<T> {
		Vec {
			ptr: Some(Self::with_capacity_in(capacity, &Global)),
			capacity: capacity,
			len: 0,
			alloc: Global
		}
	}
}

impl<T, A: Allocator> Vec<T,A> {

	pub fn raw_size(&self) -> usize {
		self.capacity * core::mem::size_of::<T>()
	}

	pub fn capacity(&self) -> usize {
		self.capacity
	}

	pub fn len(&self) -> usize {
		self.len
	}

	pub fn allocator(&self) -> &A {
		&self.alloc
	}

	pub fn realloc(&mut self, new_size: usize) -> Result<(), AllocError> {
		if self.ptr.is_none() {
			self.ptr = Some(Self::try_alloc(new_size, self.allocator())?);
			self.capacity = new_size;
			if new_size < self.len() { self.len = self.capacity }
			Ok(())
		} else {
			let layout = Self::layout(self.capacity());
			match self.allocator().realloc(self.ptr.unwrap().cast(), layout, new_size) {
				Ok(ptr) => { self.ptr = Some(ptr.cast()); Ok(())},
				Err(x) => Err(x)
			}
		}
	}

	pub fn reserve(&mut self, additional: usize) {
		match self.try_reserve(self.capacity + additional) {
			Ok(_) => {},
			Err(_) => panic!("Couldn't reserve more")
		};
	}

	pub fn try_reserve(&mut self, additional: usize) -> Result<(), AllocError> {
		self.realloc(self.capacity() + additional)
	}

	pub fn push(&mut self, value: T) {
		if self.len + 1 < self.capacity && self.ptr.is_some() {
			unsafe {
				self.ptr.unwrap().as_ptr()
						.add(self.len)
						.write(value);
			}
			self.len += 1;
		} else if self.ptr.is_none() {
			self.reserve(8);
		} else {
			self.reserve(self.capacity() + 8);
		}
	}

	pub fn pop(&mut self) -> Option<T> {
		if self.len == 0 {
			None
		} else {
			unsafe {
				self.len -= 1;
				Some(core::ptr::read(self.as_ptr().add(self.len)))
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
			if self.ptr.is_some() {
				NonNull::slice_from_raw_parts(self.ptr.unwrap(), self.len).as_ref()
			} else {
				&[]
			}
		}
	}

	pub fn as_mut_slice(&mut self) -> &mut [T] {
		unsafe {
			if self.ptr.is_some() {
				NonNull::slice_from_raw_parts(self.ptr.unwrap(), self.len).as_mut()
			} else {
				&mut []
			}
		}
	}

	pub fn as_ptr(&self) -> *const T {
		if self.ptr.is_some() {
			self.ptr.unwrap().as_ptr()
		} else {
			core::ptr::null()
		}
	}

	pub fn as_mut_ptr(&mut self) -> *mut T {
		if self.ptr.is_some() {
			self.ptr.unwrap().as_ptr()
		} else {
			core::ptr::null_mut()
		}
	}
}

impl<T, A: Allocator> Vec<T,A> {

	pub fn with_capacity_in(capacity: usize, alloc: &dyn Allocator) -> NonNull<T> {
		match Self::try_alloc(capacity, alloc) {
			Ok(x) => x,
			Err(_) => panic!("Allocation failed")
		}
	}

	pub fn layout(size: usize) -> Layout {
		Layout::from_size_align(size * core::mem::size_of::<T>(), GLOBAL_ALIGN).unwrap()
	}

	fn try_alloc(capacity: usize, alloc: &dyn Allocator) -> Result<NonNull<T>, AllocError> {
		match alloc.allocate(Self::layout(capacity)) {
			Ok(res) => Ok(res.cast()),
			Err(_) => Err(AllocError{})
		}
	}
}

impl<T: core::fmt::Display + core::fmt::Debug, A: Allocator> core::fmt::Display for Vec<T, A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		write!(f, "Vec: {}\nPtr: {:p}\nCapacity: {}\nLength: {}\nArray: {:?}\n{}"
						, '{', self.as_ptr(), self.capacity, self.len, self.as_slice(),'}')
	}
}

/* TODO!: Trait to be implemented
**	Drop, Deref, DerefMut, AsMut, AsRef, Index, IndexMut, IntoIterator
*/
impl<T, A: Allocator> Drop for Vec<T,A> {
	fn drop(&mut self) {
		if self.ptr.is_some() {
			self.allocator()
				.deallocate(self.ptr.unwrap().cast(), Layout::from_size_align(self.capacity(), GLOBAL_ALIGN).unwrap());
		}
	}
}
