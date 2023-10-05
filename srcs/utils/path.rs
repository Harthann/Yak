use crate::alloc::string::{String, ToString};
use crate::alloc::vec::Vec;

pub struct Path {
	inner: String
}

impl Path {
	pub fn new(string: &str) -> Self {
		// Cleanup multiple slashes
		let splited: Vec<&str> =
			string.split("/").filter(|s| !s.is_empty()).collect();
		let mut path = splited.join("/");
		if string.starts_with("/") {
			path.insert_str(0, "/");
		}
		Self { inner: path }
	}

	// Remove '.' and '..'
	pub fn cleanup(&mut self) {
		let mut splited: Vec<&str> = self
			.inner
			.split("/")
			.filter(|s| !s.is_empty() && s != &".")
			.collect();
		let splited_cpy = splited.clone();
		let mut index = 0;
		for elem in &mut splited_cpy.iter() {
			if elem == &".." {
				if index != 0 {
					splited.remove(index);
					splited.remove(index - 1);
					index -= 1;
				} else {
					splited.remove(index);
				}
			} else {
				index += 1;
			}
		}
		let mut path = splited.join("/");
		if self.has_root() {
			path.insert_str(0, "/");
		} else if path.is_empty() {
			path = ".".to_string();
		}
		self.inner = path;
	}

	pub fn file_name(&self) -> Option<&str> {
		let mut splited: Vec<&str> = self.inner.split("/").collect();
		splited.pop()
	}

	pub fn parent(&self) -> Option<Path> {
		let mut splited: Vec<&str> = self.inner.split("/").collect();
		splited.pop();
		let mut path = splited.join("/");
		if self.has_root() {
			path.insert_str(0, "/");
		}
		Some(Path::new(&path))
	}

	pub fn extension(&self) -> Option<&str> {
		todo!()
	}

	pub fn has_root(&self) -> bool {
		self.inner.starts_with("/")
	}

	pub fn is_absolute(&self) -> bool {
		self.has_root()
	}

	pub fn is_relative(&self) -> bool {
		!self.has_root()
	}

	pub fn is_file(&self) -> bool {
		todo!()
	}

	pub fn is_dir(&self) -> bool {
		todo!()
	}

	pub fn as_str(&self) -> &str {
		self.inner.as_str()
	}
}
