//! Allocator, Box and Pagination

pub mod allocator;
#[macro_use]
pub mod paging;

use crate::memory::paging::{
	alloc_pages,
	alloc_pages_at_addr,
	free_pages,
	kalloc_pages,
	kalloc_pages_at_addr
};

pub type VirtAddr = u32;
pub type PhysAddr = u32;

/// Permission aren't yet use, and will probably be seperated from Paging permission
/// Since paging flags are already stored in page directory it's probably not needed to store them
/// here as well
use crate::memory::paging::PAGE_WRITABLE;
/// Prots
pub const WRITABLE: u32 = PAGE_WRITABLE;
pub const READABLE: u32 = WRITABLE << 1;
pub const EXECUTABLE: u32 = WRITABLE << 2;

/// Flags
/// Flags starting with an underscore re ignored by linux kernel and so are useless
pub const MAP_SHARED: u32 = 1 << 0;
pub const MAP_PRIVATE: u32 = 1 << 1;
/// Valid only for 64 bits system
pub const MAP_32BIT: u32 = 1 << 2;
pub const MAP_ANON: u32 = 1 << 3;
pub const MAP_ANONYMOUS: u32 = 1 << 4;
pub const _MAP_DENYWRITE: u32 = 1 << 5;
pub const _MAP_EXECUTABLE: u32 = 1 << 6;
pub const _MAP_FILE: u32 = 1 << 7;
pub const MAP_FIXED: u32 = 1 << 8;
pub const MAP_GROWSDOWN: u32 = 1 << 9;
pub const MAP_LOCKED: u32 = 1 << 10;
pub const MAP_NONBLOCK: u32 = 1 << 11;
pub const MAP_NORESERVE: u32 = 1 << 12;
pub const MAP_POPULATE: u32 = 1 << 13;

pub fn init_memory_addr(
	addr: VirtAddr,
	size: usize,
	flags: u32,
	kphys: bool
) -> Result<VirtAddr, ()> {
	assert!(size % 4096 == 0, "size must be a multiple of 4096");
	let nb_page: usize = size / 4096;

	if kphys {
		kalloc_pages_at_addr(addr, nb_page, flags)
	} else {
		alloc_pages_at_addr(addr, nb_page, flags)
	}
}

pub fn init_memory(
	size: usize,
	flags: u32,
	kphys: bool
) -> Result<VirtAddr, ()> {
	assert!(size % 4096 == 0, "size must be a multiple of 4096");
	let nb_page: usize = size / 4096;

	if kphys {
		kalloc_pages(nb_page, flags)
	} else {
		alloc_pages(nb_page, flags)
	}
}

#[derive(Clone, Copy, Debug)]
pub enum TypeZone {
	Unassigned,
	Stack,
	Heap,
	Anon,
	File(&'static str)
}

#[derive(Clone)]
pub struct MemoryZone {
	pub name:      &'static str,
	pub offset:    VirtAddr,
	pub type_zone: TypeZone,
	pub size:      usize,
	pub flags:     u32,
	pub kphys:     bool
}

impl MemoryZone {
	pub const fn new() -> Self {
		Self {
			name:      "",
			offset:    0,
			type_zone: TypeZone::Unassigned,
			size:      0,
			flags:     0,
			kphys:     false
		}
	}

	pub fn init_addr(
		offset: VirtAddr,
		ztype: TypeZone,
		size: usize,
		flags: u32,
		kphys: bool
	) -> MemoryZone {
		let mut mz: MemoryZone = MemoryZone {
			name: "",
			offset,
			type_zone: ztype,
			size,
			flags,
			kphys
		};
		mz.name = match ztype {
			TypeZone::Heap => "heap",
			TypeZone::Stack => "stack",
			TypeZone::Unassigned => "unassigned",
			TypeZone::File(name) => name,
			_ => "Not yet named"
		};
		mz.offset = init_memory_addr(offset, size, flags, kphys)
			.expect("unable to allocate pages for stack");
		mz
	}

	pub fn init(
		ztype: TypeZone,
		size: usize,
		flags: u32,
		kphys: bool
	) -> MemoryZone {
		let mut mz: MemoryZone = MemoryZone {
			name: "",
			offset: 0,
			type_zone: ztype,
			size,
			flags,
			kphys
		};
		mz.name = match ztype {
			TypeZone::Heap => "heap",
			TypeZone::Stack => "stack",
			TypeZone::Unassigned => "unassigned",
			TypeZone::File(name) => name,
			_ => "Not yet named"
		};
		mz.offset = init_memory(size, flags, kphys)
			.expect("unable to allocate pages for stack");
		mz
	}
}
/// Can be templated to make slices of differents data type easily
impl MemoryZone {
	pub fn to_slice(&self) -> &[u8] {
		unsafe {
			core::slice::from_raw_parts(self.offset as *const u8, self.size)
		}
	}

	pub fn to_slice_mut(&mut self) -> &mut [u8] {
		unsafe {
			core::slice::from_raw_parts_mut(self.offset as *mut u8, self.size)
		}
	}

	/// Add pages to the memory zone, growing upward for most zones. And downward for stacks type
	/// Should return an error if failed
	pub fn grow(&mut self) -> Result<(), ()> {
		todo!()
	}

	/// Remap the memory zone in the virtual address space using new_addr as a hint or strict addr
	/// if flags REMAP_FIXED is set.
	pub fn remap(
		&mut self,
		_new_addr: VirtAddr,
		_new_size: usize,
		_flags: u32
	) -> Result<VirtAddr, ()> {
		todo!()
	}

	/// Change protection of a memory zone
	pub fn protect(&mut self, _prot: u32) {
		todo!()
	}
}

use core::ops::Deref;
impl Deref for MemoryZone {
	type Target = [u8];
	fn deref(&self) -> &Self::Target {
		self.to_slice()
	}
}

use core::ops::DerefMut;
impl DerefMut for MemoryZone {
	fn deref_mut(&mut self) -> &mut [u8] {
		self.to_slice_mut()
	}
}

use core::fmt;
impl fmt::Display for MemoryZone {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let readable: char = if self.flags & READABLE == READABLE {
			'R'
		} else {
			'-'
		};
		let writable: char = if self.flags & WRITABLE == WRITABLE {
			'W'
		} else {
			'-'
		};
		let executable: char = if self.flags & EXECUTABLE == EXECUTABLE {
			'X'
		} else {
			'-'
		};
		let shared: char = if self.flags & MAP_SHARED == MAP_SHARED {
			'S'
		} else {
			'-'
		};
		write!(
			f,
			"{:#10x} {:#10x} {}{}{}{} [ {} ]",
			self.offset,
			self.size,
			readable,
			writable,
			executable,
			shared,
			self.name
		)
	}
}

use core::ops::Drop;
impl Drop for MemoryZone {
	fn drop(&mut self) {
		match self.type_zone {
			TypeZone::Unassigned => { /* Memory not allocated do nothing */ },
			_ => {
				let mut npages = self.size / 4096;
				// If memory size isn't aligned to page size this means more memory is allocated
				if self.size % 4096 != 0 {
					npages += 1
				}
				free_pages(self.offset, npages);
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::{MemoryZone, TypeZone};
	use crate::memory::paging::bitmap::physmap_as_mut;
	#[sys_macros::test_case]
	fn base_memory_zone() {
		let used_pages = physmap_as_mut().used;
		let mut mz = MemoryZone::init(TypeZone::Anon, 0x1000, 0, false);
		assert_eq!(used_pages + 1, physmap_as_mut().used);
		drop(&mz);
		// assert_eq!(used_pages, physmap_as_mut().used);
	}

	#[sys_macros::test_case]
	fn memory_zone_as_slice() {
		let mut mz =
			MemoryZone::init(TypeZone::Anon, 0x1000, super::WRITABLE, false);
		for i in 0..255 {
			mz[i as usize] = i;
		}
		for i in 0..255 {
			assert_eq!(mz[i as usize], i);
		}
	}

	#[sys_macros::test_case]
	fn memory_zone_for_files() {
		let mut mz = MemoryZone::init(
			TypeZone::File("This is my file name"),
			0x1000,
			super::WRITABLE,
			false
		);
		assert_eq!("This is my file name", mz.name);
	}
}
