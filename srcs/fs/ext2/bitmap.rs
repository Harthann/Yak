use crate::alloc::boxed::Box;

pub struct Bitmap {
	pub map: Box<[u8]>
}

impl Bitmap {
	pub fn get_space(&self) -> (usize, usize) {
		let mut free = 0;
		let mut alloc = 0;
		for i in 0..self.map.len() {
			for j in 0..8 {
				if self.map[i] & self.mask(j) != 0 {
					alloc = alloc + 1;
				} else {
					free = free + 1;
				}
			}
		}
		(free, alloc)
	}

	pub fn get_free_node(&mut self) -> Option<usize> {
		for i in 0..self.map.len() {
			for j in 0..8 {
				if self.map[i] & self.mask(j) != self.mask(j) {
                    self.set_node(i * 8 + j);
					return Some(i * 8 + j);
				}
			}
		}
		None
	}

    pub fn mask(&self, index: usize) -> u8 {
        0b10000000 >> index
    }

	pub fn set_node(&mut self, index: usize) {
		let modulo = index % 8;
        crate::dprintln!("Beofre set node {:#010b}", self.map[index / 8]);
		self.map[index / 8] |= self.mask(modulo);
        crate::dprintln!("After set node {:#010b}", self.map[index / 8]);
	}

	pub fn unset_node(&mut self, index: usize) {
		let modulo = index % 8;
		self.map[index / 8] ^= 1 << modulo
	}

	pub fn get_node(&self, index: usize) -> bool {
		self.map[index / 8] & self.mask(index % 8) != 0
	}
}

use crate::alloc::vec::Vec;
impl From<&[u8]> for Bitmap {
	fn from(buffer: &[u8]) -> Self {
		Self {
            map: Vec::<u8>::from(buffer).into_boxed_slice()
        }
	}
}
