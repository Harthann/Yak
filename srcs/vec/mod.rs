use core::ptr::{NonNull};
use core::alloc::{Layout, };
use crate::GLOBAL_ALIGN;
 use crate::allocator::{
Allocator,
AllocError,
Global
};

pub struct Vec<T, A: Allocator = crate::allocator::Global> {
	ptr: NonNull<T>,
	capacity: usize,
	len: usize,
}

pub fn test() {
	let x = Vec::<i32>::with_capacity(5);
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
		Self::with_capacity_in(capacity, Global);
		todo!()
	}

	pub fn capacity(&self) -> usize {
		self.capacity
	}
}


impl<T, A: Allocator> Vec<T,A> {
	
	pub const fn with_capacity_in(capacity: usize, alloc: dyn Allocator) -> NonNull<T> {
		match Self::try_alloc(capacity, alloc) {
			Ok(x) => x,
			Err(_) => panic!("Allocation failed")
		}
	}

	fn try_alloc(capacity: usize, alloc: dyn Allocator) -> Result<NonNull<T>, AllocError> {
		let layout = Layout::from_size_align(capacity, GLOBAL_ALIGN).unwrap();
		let res = alloc.allocate(layout)?.cast();
		crate::kprintln!("Test: {}", res);
	}
}
