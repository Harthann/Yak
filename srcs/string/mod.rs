use core::ptr::{NonNull};

pub struct String {
	ptr: NonNull<char>,//*mut char,
	capacity: usize,
	len: usize
}

impl String {

	pub const fn new() -> String {
		String {
			ptr: NonNull::<char>::dangling(),
			capacity: 0,
			len: 0
		}
	}

	pub const fn with_capcaity(capacity: usize) -> String {
		todo!()
	}

	pub fn capacity(&self) -> usize {
		self.capacity
	}
}
