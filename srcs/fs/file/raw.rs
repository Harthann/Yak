use crate::memory::paging::bitmap::PAGE_SIZE;
use crate::memory::{MemoryZone, TypeZone};

/// Base representation of a file in Memory
/// Currently doesn't implement FileOperation but it probably should
pub struct RawFileMemory {
	pub buffer:  MemoryZone,
	pub woffset: usize,
	pub roffset: usize
}

impl RawFileMemory {
	/// Create a default Anonymous file with 1 PAGE of memory
	pub fn new() -> Self {
		Self {
			buffer:  MemoryZone::init(
				TypeZone::Anon,
				PAGE_SIZE,
				crate::memory::WRITABLE,
				false
			),
			woffset: 0,
			roffset: 0
		}
	}
}

use core::ops::Deref;
impl Deref for RawFileMemory {
	type Target = MemoryZone;
	fn deref(&self) -> &Self::Target {
		&self.buffer
	}
}

use core::ops::DerefMut;
impl DerefMut for RawFileMemory {
	fn deref_mut(&mut self) -> &mut MemoryZone {
		&mut self.buffer
	}
}
