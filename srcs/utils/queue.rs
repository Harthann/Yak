use crate::vec::Vec;

// TODO: Change to VecDeque
pub struct Queue<T> {
	vec: Vec<T>
}

impl<T: Clone> Queue<T> {
	pub const fn new() -> Self {
		Self { vec: Vec::new() }
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

	pub fn front(&self) -> Option<&T> {
		self.get(0)
	}

	pub fn front_mut(&mut self) -> Option<&mut T> {
		self.get_mut(0)
	}

	pub fn get(&self, index: usize) -> Option<&T> {
		if index < self.len() {
			Some(&self.vec[index])
		} else {
			None
		}
	}

	pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
		if index < self.len() {
			Some(&mut self.vec[index])
		} else {
			None
		}
	}

	pub fn peek(&self) -> Option<T> {
		if !self.vec.is_empty() {
			Some(self.vec[0].clone())
		} else {
			None
		}
	}
}
