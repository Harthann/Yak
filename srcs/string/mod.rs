use crate::vec::Vec;
use core::{
str,
ops,
fmt
};

#[cfg(test)]
pub mod test;

pub fn test() {
	let x = String::new();
	let y = String::from("abcdef");

	crate::kprintln!("{}", x);
	crate::kprintln!("{}", y);
}

#[derive(PartialEq, Debug)]
pub struct String {
	vec: Vec<u8>
}

impl String {

	pub fn new() -> String {
		String {
			vec: Vec::new()
		}
	}

	pub fn with_capacity(capacity: usize) -> String {
		String {
			vec: Vec::with_capacity(capacity)
		}
	}

	pub fn as_str(&self) -> &str {
		self
	}

	pub fn as_mut_str(&mut self) -> &mut str {
		self
	}

	pub fn capacity(&self) -> usize {
		self.vec.capacity()
	}

	pub fn reserve(&mut self, additional: usize) {
		self.vec.reserve(additional);
	}

	pub fn push(&mut self, value: char) {
		self.vec.push(value as u8);
	}

	pub fn push_str(&mut self, value: &str) {
		todo!()
	}

	pub fn pop(&mut self) -> Option<char> {
		match self.vec.pop() {
			Some(x) => Some(x as char),
			None => None
		}
	}

	pub fn remove(&mut self, idx: usize) -> Option<char> {
		match self.vec.remove(idx) {
			Some(x) => Some(x as char),
			None => None
		}
	}

	pub fn clear(&mut self) {
		self.vec.clear();
	}

	pub fn insert(&mut self, idx: usize, ch: char) {
		todo!()
	}

	pub fn insert_str(&mut self, idx: usize, string: &str) {
		todo!()
	}
}

impl ops::Deref for String {
	type Target = str;

	#[inline]
	fn deref(&self) -> &str {
		unsafe{ str::from_utf8_unchecked(&self.vec)}
	}
}

impl ops::DerefMut for String {

	#[inline]
	fn deref_mut(&mut self) -> &mut str {
		unsafe{ str::from_utf8_unchecked_mut(&mut *self.vec)}
	}
}

impl From<&str> for String {
	
	#[inline]
	fn from(s: &str) -> String {
		String {
			vec: Vec::<u8>::into_vec(s.as_bytes())
		}
	}
}

impl fmt::Display for String {
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		fmt::Display::fmt(&**self, f)
	}
}

