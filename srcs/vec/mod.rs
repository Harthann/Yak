use core::ptr::{NonNull};
use core::alloc::{Layout, GlobalAlloc};

pub struct Vec<T, A: GlobalAlloc = crate::allocator::GlobAlloc> {
	ptr: NonNull<T>,
	capacity: usize,
	len: usize,
}

impl<T> Vec<T> {

	pub const fn new() -> Vec<T> {
		Vec {
			ptr: NonNull::<char>::dangling(),
			capacity: 0,
			len: 0
		}
	}

	pub const fn with_capcaity(capacity: usize) -> Vec<T> {
		todo!()
	}

	pub fn capacity(&self) -> usize {
		self.capacity
	}
}

