use crate::memory::{MemoryZone, TypeZone};

pub struct RawFileMemory {
	pub buffer: MemoryZone,
	pub woffset: usize,
	pub roffset: usize
}

impl RawFileMemory {
	pub fn new() -> Self {
		Self {
			buffer: MemoryZone::init(
				TypeZone::Anon,
				4096,
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


