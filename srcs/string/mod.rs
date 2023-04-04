//! String implementation

use crate::memory::allocator::AllocError;
use crate::vec::Vec;
use core::{fmt, ops, str};

#[cfg(test)]
pub mod test;

pub fn test() {
	let x = String::new();
	let y = String::from("abcdef");

	crate::kprintln!("{}", x);
	crate::kprintln!("{}", y);
}

#[derive(Debug, Clone)]
pub struct String {
	vec: Vec<u8>
}

impl String {
	#[inline]
	pub const fn new() -> String {
		String { vec: Vec::new() }
	}

	#[inline]
	pub fn with_capacity(capacity: usize) -> String {
		String { vec: Vec::with_capacity(capacity) }
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

	// Insertion functions wrapper of vec container
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

	#[inline]
	pub fn insert(&mut self, idx: usize, ch: char) {
		self.vec.insert(idx, ch as u8);
	}

	pub fn insert_str(&mut self, mut idx: usize, string: &str) {
		for i in string.chars() {
			self.vec.insert(idx, i as u8);
			idx += 1;
		}
	}

	#[inline]
	pub fn try_reserve(&mut self, additional: usize) -> Result<(), AllocError> {
		self.vec.try_reserve(additional)
	}

	#[inline]
	pub fn try_push(&mut self, value: char) -> Result<(), AllocError> {
		self.vec.try_push(value as u8)
	}

	pub fn try_push_str(&mut self, value: &str) -> Result<(), AllocError> {
		for i in value.chars() {
			self.try_push(i)?;
		}
		Ok(())
	}

	#[inline]
	pub fn try_insert(
		&mut self,
		idx: usize,
		ch: char
	) -> Result<(), AllocError> {
		self.vec.try_insert(idx, ch as u8)
	}

	pub fn try_insert_str(
		&mut self,
		mut idx: usize,
		string: &str
	) -> Result<(), AllocError> {
		for i in string.chars() {
			self.vec.try_insert(idx, i as u8)?;
			idx += 1;
		}
		Ok(())
	}

	// Deletion function wrapper
	pub fn pop(&mut self) -> Option<char> {
		match self.vec.pop() {
			Some(x) => Some(x as char),
			None => None
		}
	}

	pub fn remove(&mut self, idx: usize) -> char {
		self.vec.remove(idx) as char
	}

	#[inline]
	pub fn clear(&mut self) {
		self.vec.clear();
	}
}

impl ops::Deref for String {
	type Target = str;

	#[inline]
	fn deref(&self) -> &str {
		unsafe { str::from_utf8_unchecked(&self.vec) }
	}
}

impl ops::DerefMut for String {
	#[inline]
	fn deref_mut(&mut self) -> &mut str {
		unsafe { str::from_utf8_unchecked_mut(&mut *self.vec) }
	}
}

impl From<&str> for String {
	#[inline]
	fn from(s: &str) -> String {
		String { vec: Vec::<u8>::into_vec(s.as_bytes()) }
	}
}

impl fmt::Display for String {
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		fmt::Display::fmt(&**self, f)
	}
}

// Equality implementation between two String
impl PartialEq for String {
	#[inline]
	fn eq(&self, other: &String) -> bool {
		PartialEq::eq(&self[..], &other[..])
	}
	#[inline]
	fn ne(&self, other: &String) -> bool {
		PartialEq::ne(&self[..], &other[..])
	}
}

// Equality implementation betwee String and str either lhs or rhs
impl PartialEq<str> for String {
	#[inline]
	fn eq(&self, other: &str) -> bool {
		PartialEq::eq(&self[..], &other[..])
	}
	#[inline]
	fn ne(&self, other: &str) -> bool {
		PartialEq::ne(&self[..], &other[..])
	}
}

impl PartialEq<String> for str {
	#[inline]
	fn eq(&self, other: &String) -> bool {
		PartialEq::eq(&self[..], &other[..])
	}
	#[inline]
	fn ne(&self, other: &String) -> bool {
		PartialEq::ne(&self[..], &other[..])
	}
}

// Same as previous implementation but takes ref to str instead
impl PartialEq<&str> for String {
	#[inline]
	fn eq(&self, other: &&str) -> bool {
		PartialEq::eq(&self[..], &other[..])
	}
	#[inline]
	fn ne(&self, other: &&str) -> bool {
		PartialEq::ne(&self[..], &other[..])
	}
}

impl PartialEq<String> for &str {
	#[inline]
	fn eq(&self, other: &String) -> bool {
		PartialEq::eq(&self[..], &other[..])
	}
	#[inline]
	fn ne(&self, other: &String) -> bool {
		PartialEq::ne(&self[..], &other[..])
	}
}

// Write and ToString impl are taken from rust source code
impl fmt::Write for String {
	#[inline]
	fn write_str(&mut self, s: &str) -> fmt::Result {
		self.push_str(s);
		Ok(())
	}

	#[inline]
	fn write_char(&mut self, c: char) -> fmt::Result {
		self.push(c);
		Ok(())
	}
}

// ToString impl for different type
pub trait ToString {
	fn to_string(&self) -> String;
}

impl<T: fmt::Display + ?Sized> ToString for T {
	// A common guideline is to not inline generic functions. However,
	// removing `#[inline]` from this method causes non-negligible regressions.
	// See <https://github.com/rust-lang/rust/pull/74852>, the last attempt
	// to try to remove it.
	#[inline]
	default fn to_string(&self) -> String {
		let mut buf = String::new();
		let mut formatter = core::fmt::Formatter::new(&mut buf);
		// Bypass format_args!() to avoid write_str with zero-length strs
		fmt::Display::fmt(self, &mut formatter)
			.expect("a Display implementation returned an error unexpectedly");
		buf
	}
}

impl ToString for str {
	#[inline]
	fn to_string(&self) -> String {
		String::from(self)
	}
}
