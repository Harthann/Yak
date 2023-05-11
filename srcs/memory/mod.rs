//! Allocator, Box and Pagination

pub mod allocator;
#[macro_use]
pub mod paging;

use crate::memory::paging::{
	alloc_pages,
	alloc_pages_at_addr,
	kalloc_pages,
	kalloc_pages_at_addr,
    free_pages
};

pub type VirtAddr = u32;
pub type PhysAddr = u32;

/// Permission aren't yet use, and will probably be seperated from Paging permission
/// Since paging flags are already stored in page directory it's probably not needed to store them
/// here as well
use crate::memory::paging::{PAGE_WRITABLE, PAGE_PRESENT};
pub const PRESENT:    u32 = PAGE_PRESENT;
pub const WRITABLE:   u32 = PAGE_WRITABLE;
pub const READABLE:   u32 = PAGE_WRITABLE << 1;
pub const EXECUTABLE: u32 = PAGE_WRITABLE << 2;
pub const SHARED:     u32 = PAGE_WRITABLE << 3;

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

#[derive(Clone, Copy)]
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
        ztype:  TypeZone,
		size:   usize,
		flags:  u32,
		kphys:  bool
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
            TypeZone::Heap       => "heap",
            TypeZone::Stack      => "stack",
            TypeZone::Unassigned => "unassigned",
            _                    => "Not yet named"
        };
		mz.offset = init_memory_addr(offset, size, flags, kphys)
			.expect("unable to allocate pages for stack");
		mz
	}

	pub fn init(ztype: TypeZone, size: usize, flags: u32, kphys: bool) -> MemoryZone {
		let mut mz: MemoryZone = MemoryZone {
            name: "",
			offset:    0,
			type_zone: ztype,
			size,
			flags,
			kphys
		};
        mz.name = match ztype {
            TypeZone::Heap       => "heap",
            TypeZone::Stack      => "stack",
            TypeZone::Unassigned => "unassigned",
            _                    => "Not yet named"
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
        let readable:   char = if self.flags & READABLE   == READABLE   { 'R' } else { '-' };
        let writable:   char = if self.flags & WRITABLE   == WRITABLE   { 'W' } else { '-' };
        let executable: char = if self.flags & EXECUTABLE == EXECUTABLE { 'X' } else { '-' };
        let shared:     char = if self.flags & SHARED     == SHARED     { 'S' } else { '-' };
		write!(f, "{:#10x} {:#10x} {}{}{}{} [ {} ]",
               self.offset, self.size, readable, writable, executable, shared, self.name)
	}
}

use core::ops::Drop;
impl Drop for MemoryZone {
	fn drop(&mut self) {
        let mut npages = self.size / 4096;
        // If memory size isn't aligned to page size this means more memory is allocated
        if self.size % 4096 != 0 { npages += 1 }
        free_pages(self.offset, npages);
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
        //assert_eq!(used_pages, physmap_as_mut().used);
    }

    #[sys_macros::test_case]
    fn memory_zone_as_slice() {
        let mut mz = MemoryZone::init(TypeZone::Anon, 0x1000, super::WRITABLE, false);
        for i in 0..255 {
            mz[i as usize] = i;
        }
        for i in 0..255 {
            assert_eq!(mz[i as usize], i);
        }

    }
}
