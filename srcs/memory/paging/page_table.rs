use core::fmt;

use crate::page_directory;
use crate::memory::{PhysAddr, VirtAddr};
use crate::PAGE_WRITABLE;

extern "C" {
	fn _start_rodata();
}

#[repr(transparent)]
pub struct PageTable {
	pub entries: [PageTableEntry; 1024]
}

impl PageTable {
	pub fn init(&mut self) {
		let mut i: usize = 0;

		let page_directory_entry: usize = unsafe{page_directory.get_vaddr() as usize};
		while i < 1024 {
			if i == 0 /* gdt */ ||
(i >= (_start_rodata as usize & 0x3ff000) >> 12 &&
i <= (page_directory_entry & 0x3ff000) >> 12) || i == (0xb8000 >> 12) /* VGA_BUFFER */ {
				self.entries[i] = ((i * 0x1000) as u32 | PAGE_WRITABLE | 1).into();
			}
			else if i < (_start_rodata as usize & 0x3ff000) >> 12 {
				self.entries[i] = ((i * 0x1000) as u32 | 1).into();
			} else {
				self.entries[i] = 0x0.into();
			}
			i += 1;
		}
	}

	pub fn set_entry(&mut self, index: usize, value: u32) {
		self.entries[index] = value.into();
	}

	pub fn clear(&mut self) {
		let mut i: usize = 0;

		while i < 1024 {
			self.entries[i] = (0x0 as u32).into();
			i += 1;
		}
	}

	pub fn new_index_frame(&mut self, index: usize, paddr: PhysAddr, flags: u32) {
		self.entries[index] = (paddr | flags | 1).into();
	}

	pub fn new_frame(&mut self, paddr: PhysAddr, flags: u32) -> Result<u16, ()> {
		let mut i: usize = 0;

		while i < 1024 {
			if self.entries[i].get_present() != 1 {
				self.entries[i] = (paddr | flags | 1).into();
				return Ok(i as u16);
			}
			i += 1;
		}
		Err(())
	}

	pub fn get_vaddr(&self) -> VirtAddr {
		self as *const Self as VirtAddr
	}
}

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct PageTableEntry {
	pub value: u32
}

impl From<u32> for PageTableEntry {
	fn from(item: u32) -> Self {
		PageTableEntry { value: item }
	}
}

impl fmt::Display for PageTableEntry {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:#010x} - P: {} | R/W: {} | U/S: {} | PWT: {} | PCD: {} | A: {} | D: {} | PAT: {} | G: {} | AVL: {:#010x} | Address: {:#010x}", self.get_vaddr(), self.get_present(), self.get_writable(), self.get_user_supervisor(),
self.get_pwt(), self.get_pcd(), self.get_accessed(), self.get_dirty(), self.get_pat(),
self.get_global(), self.get_avl(), self.get_paddr())
	}
}

impl PageTableEntry {
	pub fn get_present(&self) -> u8 {
		(self.value & 0b00000001) as u8
	}

	pub fn get_writable(&self) -> u8 {
		((self.value & 0b00000010) >> 1) as u8
	}

	pub fn get_user_supervisor(&self) -> u8 {
		((self.value & 0b00000100) >> 2) as u8
	}

	pub fn get_pwt(&self) -> u8 {
		((self.value & 0b00001000) >> 3) as u8
	}

	pub fn get_pcd(&self) -> u8 {
		((self.value & 0b00010000) >> 4) as u8
	}

	pub fn get_accessed(&self) -> u8 {
		((self.value & 0b00100000) >> 5) as u8
	}

	pub fn get_dirty(&self) -> u8 {
		((self.value & 0b01000000) >> 6) as u8
	}

	pub fn get_pat(&self) -> u8 {
		((self.value & 0b10000000) >> 7) as u8
	}

	pub fn get_global(&self) -> u8 {
		((self.value & 0b100000000) >> 8) as u8
	}

	pub fn get_avl(&self) -> u8 {
		((self.value & 0b111000000000) >> 9) as u8
	}

	pub fn get_paddr(&self) -> PhysAddr {
		self.value & 0xfffff000
	}

	pub fn get_vaddr(&self) -> VirtAddr {
		self as *const Self as VirtAddr
	}
}
