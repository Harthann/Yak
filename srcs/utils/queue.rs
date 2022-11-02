use crate::vec::Vec;

pub struct Queue<T> {
	vec: Vec<T>
}

impl<T: Clone> Queue<T> {
	pub const fn new() -> Self {
		Self {vec: Vec::new()}
	}

	pub fn len(&self) -> usize {
		self.vec.len()
	}

	pub fn push(&mut self, item: T) {
		self.vec.push(item)
	}

	pub fn pop(&mut self) -> T {
		self.vec.remove(0)
	}

	pub fn is_empty(&self) -> bool {
		self.vec.is_empty()
	}

	pub fn peek(&self) -> Option<T> {
		if !self.vec.is_empty() {
			Some(self.vec[0].clone())
		} else {
			None
		}
	}
}
