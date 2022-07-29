use core::ptr::{NonNull};
use core::alloc::{Layout, };
use core::ops;
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

	x.push(1);
	kprintln!("X: {:?}", x.as_slice());
	x.reverse();
	kprintln!("{}", x == [4, 3, 2, 1]);
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
				Ok(ptr) => { self.ptr = Some(ptr.cast());
							self.capacity = new_size;
							Ok(())
				},
				Err(x) => Err(x)
			}
		}
	}

	pub fn reserve(&mut self, additional: usize) {
		match self.try_reserve(additional) {
			Ok(_) => {},
			Err(_) => panic!("Couldn't reserve more")
		};
	}

	pub fn try_reserve(&mut self, additional: usize) -> Result<(), AllocError> {
		self.realloc(self.capacity() + additional)
	}

	pub fn push(&mut self, value: T) {
		if self.len + 1 > self.capacity {
			self.reserve(8);
		} else if self.ptr.is_none() {
			self.reserve(8);
		}
		unsafe{ self.ptr.unwrap().as_ptr()
						.add(self.len)
						.write(value); }
		self.len += 1;
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
		if self.len + 1 > self.capacity {
			self.reserve(8);
		} else if self.ptr.is_none() {
			self.reserve(8);
		}

		unsafe{
			core::ptr::copy(self.as_ptr().add(index),
							self.as_mut_ptr().add(index + 1),
							self.len() - index);
			core::ptr::write(self.as_mut_ptr().add(index), element);
		}
		self.len += 1;
	}

	pub fn extend_from_slice(&mut self, elements: &[T]) {
		crate::kprintln!("{} {} {}", self.len, self.capacity, elements.len());
		if self.len + elements.len() > self.capacity {
			self.reserve(self.capacity() + elements.len());
		}
		unsafe{
			core::ptr::copy(elements.as_ptr(),
							self.as_mut_ptr().add(self.len()),
							elements.len());
			self.len += elements.len();
		}
	}

	pub fn remove(&mut self, index: usize) -> Option<T> {
		if self.len > index {
			None
		} else {
			unsafe{
				let erased = core::ptr::read(self.as_ptr().add(index));
				core::ptr::copy(self.as_ptr().add(index + 1),
								self.as_mut_ptr().add(index),
								self.len() - index);
				self.len -= 1;
				Some(erased)
			}
		}
	}

	pub fn as_slice(&self) -> &[T] {
		self
	}

	pub fn as_mut_slice(&mut self) -> &mut [T] {
		self
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

/* Drop will simplu deallocate our vector from the heap using the allocator */
impl<T, A: Allocator> Drop for Vec<T,A> {
	fn drop(&mut self) {
		if self.ptr.is_some() {
			self.allocator()
				.deallocate(self.ptr.unwrap().cast(), Self::layout(self.capacity()));
		}
	}
}

impl<T, A: Allocator> AsRef<[T]> for Vec<T, A> {
	fn as_ref(&self) -> &[T] {
		self
	}
}


impl<T, A: Allocator> AsMut<[T]> for Vec<T, A> {
	fn as_mut(&mut self) -> &mut [T] {
		self
	}
}


/* The deref trait allow us to dereference our vector to a slice
** Doing so rust understand if we try to access slices method
** and dereference the vector for us
*/
impl<T, A: Allocator> ops::Deref for Vec<T, A> {
	type Target = [T];

	fn deref(&self) -> &[T] {
		unsafe {
			if self.ptr.is_some() {
				NonNull::slice_from_raw_parts(self.ptr.unwrap(), self.len).as_mut()
			} else {
				&mut []
			}
		}
	}
}

impl<T, A: Allocator> ops::DerefMut for Vec<T, A> {

	fn deref_mut(&mut self) -> &mut [T] {
		unsafe {
			if self.ptr.is_some() {
				NonNull::slice_from_raw_parts(self.ptr.unwrap(), self.len).as_mut()
			} else {
				&mut []
			}
		}
	}
}

/* Macro to implement multiple PartialEq easily
** Taken from the rust source code
*/
macro_rules! __impl_slice_eq1 {
    ([$($vars:tt)*] $lhs:ty, $rhs:ty $(where $ty:ty: $bound:ident)?) => {
        impl<T, U, $($vars)*> PartialEq<$rhs> for $lhs
        where
            T: PartialEq<U>,
            $($ty: $bound)?
        {
            #[inline]
            fn eq(&self, other: &$rhs) -> bool { self[..] == other[..] }
            #[inline]
            fn ne(&self, other: &$rhs) -> bool { self[..] != other[..] }
        }
    }
}

/* Implement to compare two vector */
__impl_slice_eq1! { [A1: Allocator, A2: Allocator] Vec<T, A1>, Vec<U, A2> }

/*	Implement to compare Vec with ref slice */
__impl_slice_eq1! { [A: Allocator] Vec<T, A>, &[U] }
__impl_slice_eq1! { [A: Allocator] Vec<T, A>, &mut [U]}
__impl_slice_eq1! { [A: Allocator] &[T], Vec<U, A>}
__impl_slice_eq1! { [A: Allocator] &mut [T], Vec<U, A>}


/*	Implement to compare Vec with slice */
__impl_slice_eq1! { [A: Allocator] Vec<T, A>, [U] }
__impl_slice_eq1! { [A: Allocator] [T], Vec<U, A> }

/* Implement to compare Vec with known size array/slice */
__impl_slice_eq1! { [A: Allocator, const N: usize] Vec<T, A>, [U; N] }
__impl_slice_eq1! { [A: Allocator, const N: usize] [T; N], Vec<U, A> }

