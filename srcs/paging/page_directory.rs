use core::fmt;

use crate::paging::PhysAddr;
use crate::paging::VirtAddr;
use crate::paging::PageTable;

use crate::paging::KERNEL_BASE;
use crate::get_vaddr;

use crate::paging::page_directory;

#[repr(transparent)]
pub struct PageDirectory {
	pub entries: [PageDirectoryEntry; 1024]
}

impl PageDirectory {
	pub const fn new() -> Self {
		Self {entries: [PageDirectoryEntry::new(0x00000002); 1024]}
	}

	pub fn new_page_frame(&mut self, page_frame: PhysAddr) -> Result<VirtAddr, ()> {
		/* TODO: Check for alignment */
		let mut i: usize = 0;

		while i < 1024 {
			if self.entries[i].get_present() == 1 {
				let res = self.get_page_table(i).new_frame(page_frame);
				// reserved page 0 for swap
				if res.is_ok() && i != 0 {
					return Ok(get_vaddr!(i, res.unwrap() as usize));
				}
			}
			i += 1;
		}
		// Create a new_page_table for the new frame
		// TODO: if new_page_table not set
		i = self.new_page_table();
		let res = self.get_page_table(i).new_frame(page_frame);
		if res.is_ok() && i != 0 {
			return Ok(get_vaddr!(i, res.unwrap() as usize));
		}
		todo!();
		Err(())
	}

	pub fn remove_page_frame(&mut self, page_frame: VirtAddr) {
		unsafe {
			/* TODO: Check for alignment */
			let pd_index: usize = (page_frame & 0xffc00000 >> 22) as usize;
			let i: usize = (page_frame & 0x3ff000 >> 12) as usize;
			let page_table: &mut PageTable = page_directory.get_page_table(pd_index);
			page_table.entries[i] = 0.into();
		}
	}

	/* Return index of the new page */
	pub fn new_page_table(&mut self) -> usize {
		unsafe {
			let mut i: usize = 0;

			while i < 1024 {
				if self.entries[i].get_present() == 0 {
					let pd_paddr: PhysAddr = page_directory.get_vaddr() - KERNEL_BASE as PhysAddr;
					let pt_paddr: PhysAddr = pd_paddr + (i as u32 + 1) * 0x1000;
					let page_tab: &mut PageTable = page_directory.get_page_table(0);
					page_tab.entries[i % 1023] = (pt_paddr | 3).into();
					let new: &mut PageTable = &mut *(get_vaddr!(0, i % 1023) as *mut _);
					new.clear();
					new.entries[1023] = (pt_paddr | 3).into();
					self.entries[i] = (pt_paddr | 3).into();
					page_tab.entries[i % 1023] = 0.into();
					return i;
				}
				i += 1;
			}
			todo!();
		}
	}

	pub fn remove_page_table(&mut self, index: usize) {
		unsafe {
			if self.entries[index].get_present() == 1 {
				let page_table: &mut PageTable = &mut *(get_vaddr!(index, 1023) as *mut _);
				page_table.clear();
				self.entries[index] = (0x00000002 as u32).into();
			} else {
				todo!();
			}
		}
	}

	pub fn get_page_table(&self, index: usize) -> &mut PageTable {
		unsafe{&mut *(get_vaddr!(index, 1023) as *mut _)}
	}

	pub fn get_vaddr(&self) -> VirtAddr {
		(&*self as *const _) as VirtAddr
	}
}

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct PageDirectoryEntry {
	value: u32
}

impl From<u32> for PageDirectoryEntry {
	fn from(item: u32) -> Self {
		PageDirectoryEntry { value: item }
	}
}

impl PageDirectoryEntry {
	pub const fn new(value: u32) -> Self {
		Self {value: value}
	}
}

impl fmt::Display for PageDirectoryEntry {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		if self.get_ps() == 0 {
			write!(f, "Present: {}
Writable: {}
User/Supervisor: {}
PWT: {}
PCD: {}
Accessed: {}
PS: {}
AVL: 0x{:x}
Address: {:#010x}", self.get_present(), self.get_writable(), self.get_user_supervisor(),
self.get_pwt(), self.get_pcd(), self.get_accessed(), self.get_ps(), self.get_avl(),
self.get_paddr())
		} else {
			write!(f, "Present: {}
Writable: {}
User/Supervisor: {}
PWT: {}
PCD: {}
Accessed: {}
Dirty: {}
PS: {}
Global: {}
AVL: 0x{:x}
PAT: {}
RSVD: {}
Address: {:#010x}", self.get_present(), self.get_writable(), self.get_user_supervisor(),
self.get_pwt(), self.get_pcd(), self.get_accessed(), self.get_dirty(), self.get_ps(),
self.get_global(), self.get_avl(), self.get_pat(), self.get_rsvd(), self.get_paddr())
		}
	}
}

impl PageDirectoryEntry {
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

	pub fn get_ps(&self) -> u8 {
		((self.value & 0b10000000) >> 7) as u8
	}

	pub fn get_dirty(&self) -> u8 {
		if self.get_ps() == 0 {
			todo!();
		} else {
			((self.value & 0b01000000) >> 6) as u8
		}
	}

	pub fn get_global(&self) -> u8 {
		if self.get_ps() == 0 {
			todo!();
		} else {
			((self.value & 0b100000000) >> 8) as u8
		}
	}

	pub fn get_avl(&self) -> u8 {
		if self.get_ps() == 0 {
			((self.value & 0b111000000000) >> 9) as u8
		} else {
			(((self.value & 0b111100000000) >> 8) | ((self.value & 0b01000000) >> 2)) as u8
		}
	}

	pub fn get_paddr(&self) -> PhysAddr {
		if self.get_ps() == 0 {
			self.value & 0xfffff000
		} else {
			((self.value & 0xfff00000) >> 10) | ((self.value & 0b111111110000000000000) << 19)
		}
	}

	pub fn get_pat(&self) -> u8 {
		if self.get_ps() == 0 {
			todo!();
		} else {
			((self.value & 0b1000000000000) >> 12) as u8
		}
	}

	pub fn get_rsvd(&self) -> u8 {
		if self.get_ps() == 0 {
			todo!();
		} else {
			((self.value & 0b100000000000000000000) >> 20) as u8
		}
	}

	pub fn get_vaddr(&self) -> VirtAddr {
		(&*self as *const _) as VirtAddr
	}
}
