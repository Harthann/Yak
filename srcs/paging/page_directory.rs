use core::fmt;
use crate::paging::phys_addr;
use crate::paging::virt_addr;
use crate::paging::page_table::PageTable;

use crate::paging::KERNEL_BASE;

use crate::page_directory;

#[repr(align(4096))]
pub struct PageDirectory {
	pub entries: [PageDirectoryEntry; 1024]
}

//pub fn get_paddr() -> u32 {
//}

pub fn get_vaddr(pd_index: usize, pt_index: usize) -> u32 {
	(pd_index << 22 | pt_index << 12) as u32
}

impl PageDirectory {
	pub const fn new() -> Self {
		Self {entries: [PageDirectoryEntry::new(0x00000002); 1024]}
	}

	pub fn new_page_frame(&self, page_frame: phys_addr) -> Result<virt_addr, ()> {
		let mut i: usize = 0;

		while i < 1024 {
			if self.entries[i].get_present() == 1 {
				let res = self.entries[i].to_page_table().new_frame(page_frame);
				if  res.is_ok() {
					return Ok(((i as virt_addr) << 22) | ((res.unwrap() as virt_addr) << 12));
				}
			}
			i += 1;
		}
		todo!();
		Err(())
	}

	pub fn new_page_table(&mut self) -> &mut PageTable {
		let mut i: usize = 0;

		while i < 1024 {
			if self.entries[i].get_present() == 0 {
				break ;
			}
			i += 1;
		}
		let paddr: u32 = ((((page_directory as *mut usize) as usize) - KERNEL_BASE + (i + 1) * 0x1000) | 3) as u32;
		unsafe{
			let mut page_tab: &mut PageTable = &mut *(0x003ff000 as *mut _);
			page_tab.entries[1022] = paddr.into();
			let mut new: &mut PageTable = &mut *(get_vaddr(0, 1022) as *mut _);
			new.reset(paddr);
			page_tab.entries[1022] = (0x0 as u32).into();
			self.entries[i] = (paddr | 3).into();
			new
		}
	}
}

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

	pub fn to_page_table(&self) -> &mut PageTable {
		unsafe{ &mut *(self.get_paddr() as *mut _)}
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

	pub fn get_paddr(&self) -> u32 {
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
}
