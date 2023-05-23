use crate::errno::ErrNo;
use crate::memory::{MemoryZone, TypeZone};
use crate::string::String;
use crate::utils::arcm::Arcm;

pub mod socket;

pub trait FileOperation {
	fn read(&self, dst: &mut [u8], length: usize) -> Result<usize, ErrNo>;
	fn write(&mut self, src: &[u8], length: usize) -> Result<usize, ErrNo>;
}

pub struct RawFileMemory {
	pub buffer: MemoryZone,
	pub offset: usize
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
			offset: 0
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

/// Contains all file information.
/// Current information are only name and FileOperation trait object
/// The op trait object can be store either by reference of Box. For the moment Box is choosen but
/// this may change in the future. To make the trait object ThreadSafe Mutex is used.
/// Arc is used to allow multiple reference on the object in a multithreaded environment
pub struct FileInfo {
	pub name: String,
	pub op:   Arcm<dyn FileOperation>
}
// Sync/Send marker to indicate rust that FileInfo is thread safe
unsafe impl Sync for FileInfo {}
unsafe impl Send for FileInfo {}

impl FileInfo {
	pub fn new(name: String, op: Arcm<dyn FileOperation>) -> Self {
		Self { name, op }
	}
}
