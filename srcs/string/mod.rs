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

	#[inline]
	pub fn new() -> String {
		String {
			vec: Vec::new()
		}
	}

	#[inline]
	pub fn with_capacity(capacity: usize) -> String {
		String {
			vec: Vec::with_capacity(capacity)
		}
	}

	#[inline]
	pub fn as_str(&self) -> &str {
		self
	}

	#[inline]
	pub fn as_mut_str(&mut self) -> &mut str {
		self
	}

	#[inline]
	pub fn capacity(&self) -> usize {
		self.vec.capacity()
	}

	#[inline]
	pub fn reserve(&mut self, additional: usize) {
		self.vec.reserve(additional);
	}

	#[inline]
	pub fn push(&mut self, value: char) {
		self.vec.push(value as u8);
	}

	pub fn push_str(&mut self, value: &str) {
		for i in value.chars() {
			self.push(i);
		}
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

	#[inline]
	pub fn clear(&mut self) {
		self.vec.clear();
	}

	#[inline]
	pub fn insert(&mut self, idx: usize, ch: char) {
		self.vec.insert(idx, ch as u8);
	}

	pub fn try_insert(&mut self, idx: usize, ch: char) -> Result<(), ()> {
		todo!()
	}

	pub fn insert_str(&mut self, mut idx: usize, string: &str) {
		for i in string.chars() {
			self.vec.insert(idx, i as u8);
			idx += 1;
		}
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

