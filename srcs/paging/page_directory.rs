use core::fmt;

use crate::kmemory;

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

	pub fn set_entry(&mut self, index: usize, value: u32) {
		self.entries[index] = value.into();
	}

	pub fn kget_page_frames_at_addr(&mut self, vaddr: VirtAddr, nb: usize) -> Result<VirtAddr, ()> {
		let pd_index: usize = ((vaddr & 0xffc00000) >> 22) as usize;
		let pt_index: usize = ((vaddr & 0x3ff000) >> 12) as usize;
		
		if self.entries[pd_index].get_present() == 0 {
			self.claim_index_page_tables(pd_index, (nb / 1024) + 1)?;
		}
		self.kclaim_index_page_frames(pd_index, pt_index, nb)?;
		Ok(vaddr)
	}

	pub fn get_page_frames_at_addr(&mut self, vaddr: VirtAddr, nb: usize) -> Result<VirtAddr, ()> {
		let pd_index: usize = ((vaddr & 0xffc00000) >> 22) as usize;
		let pt_index: usize = ((vaddr & 0x3ff000) >> 12) as usize;
		
		if self.entries[pd_index].get_present() == 0 {
			self.claim_index_page_tables(pd_index, (nb / 1024) + 1)?;
		}
		self.claim_index_page_frames(pd_index, pt_index, nb)?;
		Ok(vaddr)
	}

	/*
		Get page frames that are virtually and physically adjacents, return the
		virtual address of the first one
	*/
	pub fn kget_page_frames(&mut self, nb: usize) -> Result<VirtAddr, ()> {
		let mut available: usize = 0;
		let mut i: usize = 768; // map kernel page if physically neighbour > 0xc0000000
		let mut i_saved: usize = 0;
		let mut j: usize = 0;

		if nb == 0 {
			return Err(());
		}
		while i < 1023 && available != nb { /* 1023 reserved for page_table def */
			if self.entries[i].get_present() == 1 {
				j = 0;
				while j < 1024 && available != nb {
					if self.get_page_table(i).entries[j].get_present() == 0 {
						if available == 0 {
							i_saved = i;
						}
						available += 1;
					} else {
						available = 0;
					}
					j += 1;
				}
			}
			i += 1;
		}
		if available != nb {
			i_saved = self.claim_page_tables((nb / 1024) + 1)?;
			j = 0;
		} else {
			j -= nb;
		}
		let vaddr: VirtAddr = get_vaddr!(i_saved, j);
		self.kclaim_index_page_frames(i_saved, j, nb)?;
		Ok(vaddr)
	}

	/*
		Claim 'nb' page frames (by lowest index), pages must be virtually
		adjacents
	*/
	pub fn get_page_frames(&mut self, nb: usize) -> Result<VirtAddr, ()> {
		let mut available: usize = 0;
		let mut i: usize = 0;
		let mut i_saved: usize = 1;
		let mut j: usize = 0;

		if nb == 0 {
			return Err(());
		}
		while i < 1023 && available != nb { /* 1023 reserved for page_table def */
			if self.entries[i].get_present() == 1 {
				j = 0;
				while j < 1024 && available != nb {
					if self.get_page_table(i).entries[j].get_present() == 0 {
						if available == 0 {
							i_saved = i;
						}
						available += 1;
					} else {
						available = 0;
					}
					j += 1;
				}
			}
			i += 1;
		}
		if available != nb {
			i_saved = self.claim_page_tables((nb / 1024) + 1)?;
			j = 0;
		} else {
			j -= nb;
		}
		let vaddr: VirtAddr = get_vaddr!(i_saved, j);
		self.claim_index_page_frames(i_saved, j, nb)?;
		Ok(vaddr)
	}

	/* Claim a page frame (by lowest index) */
	pub fn get_page_frame(&mut self) -> Result<VirtAddr, ()> {
		let paddr = kmemory::physmap_as_mut().get_page()?;
		let mut i: usize = 0;

		while i < 1023 { /* 1023 reserved for page_table def */
			if self.entries[i].get_present() == 1 {
				let res = self.get_page_table(i).new_frame(paddr);
				if res.is_ok() {
					return Ok(get_vaddr!(i, res.unwrap()));
				}
			}
			i += 1;
		}
		i = self.claim_page_table()?;
		Ok(get_vaddr!(i, self.get_page_table(i).new_frame(paddr)?))
	}

	/*
		Claim a range of page frames based on 'nb' size and specified index with
		adjacent physical addresses
	*/
	fn kclaim_index_page_frames(&mut self, mut pd_index: usize, mut pt_index: usize, nb: usize) -> Result <(), ()> {
		let mut paddr: PhysAddr = kmemory::physmap_as_mut().get_pages(nb)?;

		let mut i: usize = 0;
		while i < nb {
			if pt_index == 1024 {
				pt_index = 0;
				pd_index += 1;
			}
			self.get_page_table(pd_index).new_index_frame(pt_index, paddr);
			pt_index += 1;
			i += 1;
			paddr += 4096;
		}
		Ok(())
	}

	/* Claim a range of page frames based on 'nb' size and specified index */
	fn claim_index_page_frames(&mut self, mut pd_index: usize, mut pt_index: usize, nb: usize) -> Result <(), ()> {
		let mut i: usize = 0;

		while i < nb {
			if pt_index == 1024 {
				pt_index = 0;
				pd_index += 1;
			}
			let paddr: PhysAddr = kmemory::physmap_as_mut().get_page()?;
			self.get_page_table(pd_index).new_index_frame(pt_index, paddr);
			pt_index += 1;
			i += 1;
		}
		Ok(())
	}

	fn claim_index_page_table(&mut self, index: usize) -> Result<usize, ()> {
		unsafe {
			let pd_paddr: PhysAddr = page_directory.get_vaddr() - KERNEL_BASE as PhysAddr;
			let pt_paddr: PhysAddr = pd_paddr + (index as u32 + 1) * 0x1000;
			let page_tab: &mut PageTable = page_directory.get_page_table(1023);
			page_tab.set_entry(index, pt_paddr | 3);
			crate::refresh_tlb!();
			let new: &mut PageTable = page_directory.get_page_table(index);
			new.clear();
			self.entries[index] = (pt_paddr | 3).into();
			crate::refresh_tlb!();
			Ok(index)
		}
	}

	/* Claim a new page table and return index of the new page */
	fn claim_page_table(&mut self) -> Result<usize, ()> {
			let mut i: usize = 0;

			while i < 1024 {
				if self.entries[i].get_present() == 0 {
					return self.claim_index_page_table(i);
				}
				i += 1;
			}
			todo!();
			//Err(())
	}

	/*
		Claim 'nb' new page tables adjacent and return the lowest index of those
		pages.
	*/
	fn claim_page_tables(&mut self, nb: usize) -> Result<usize, ()> {
			if nb == 1 {
				return self.claim_page_table();
			}
			let mut i: usize = 0;

			while i < 1024 {
				if self.entries[i].get_present() == 0 {
					let mut j: usize = i + 1;
					while j < 1024 && self.entries[j].get_present() == 0 && j - i != nb{
						j += 1;
					}
					if j - i == nb && j < 1024 {
						while i < j {
							self.claim_index_page_table(i);
							i += 1;
						}
						return Ok(i - nb);
					}
				}
				i += 1;
			}
			todo!();
			//Err(())
	}

	fn claim_index_page_tables(&mut self, index: usize, nb: usize) -> Result<usize, ()> {
		if nb == 1 {
			return self.claim_index_page_table(index);
		}
		let mut count: usize = 0;

		while count < nb {
			self.claim_index_page_table(index + count)?;
			count += 1;
		}
		Ok(index)
	}

	pub fn remove_page_frames(&mut self, mut vaddr: VirtAddr, nb: usize) {
		if vaddr & 0xfff != 0 {
			return ; /* Not aligned */
		}
		let mut i: usize = 0;

		while i < nb {
			self.remove_page_frame(vaddr);
			vaddr += 4096;
			i += 1;
		}
	}


	/* Remove a page frame at a specified virtual address */
	pub fn remove_page_frame(&mut self, vaddr: VirtAddr) {
		unsafe {
			if vaddr & 0xfff != 0 {
				return ; /* Not aligned */
			}
			let paddr: PhysAddr = crate::get_paddr!(vaddr);
			let pd_index: usize = ((vaddr & 0xffc00000) >> 22) as usize;
			let i: usize = ((vaddr & 0x3ff000) >> 12) as usize;
			let page_table: &mut PageTable = page_directory.get_page_table(pd_index);
			page_table.set_entry(i, 0);
			// if last page_frame, free the page_table
			let mut i = 0;
			while i < 1024 {
				if page_table.entries[i].value != 0 {
					break ;
				}
				i += 1;
			}
			if i == 1024 {
				self.remove_page_table(pd_index);
			}
			kmemory::physmap_as_mut().free_page(paddr);
		}
	}

	/* Remove a page table at specified index */
	pub fn remove_page_table(&mut self, index: usize) -> Result<(), ()> {
		unsafe {
			if self.entries[index].get_present() == 1 {
				let page_table: &mut PageTable = self.get_page_table(index);;
				page_table.clear();
				self.entries[index] = (0x00000002 as u32).into();
				crate::refresh_tlb!();
				return Ok(());
			} else {
				return Err(());
			}
		}
	}

	/*
		Get the page_table at the specified index
		The page table 0 index every page_table
	*/
	pub fn get_page_table(&self, index: usize) -> &mut PageTable {
		unsafe{&mut *(get_vaddr!(1023, index) as *mut _)}
	}

	/* Return the virtual address of the page directory */
	pub fn get_vaddr(&self) -> VirtAddr {
		self as *const Self as VirtAddr
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

impl fmt::Display for PageDirectoryEntry {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		if self.get_ps() == 0 {
			write!(f, "{:#010x} - P: {} | R/W: {} | U/S: {} | PWT: {} | PCD: {} | A: {} | PS: {} | AVL: {:#010x} | Address: {:#010x}", self.get_vaddr(), self.get_present(), self.get_writable(), self.get_user_supervisor(),
self.get_pwt(), self.get_pcd(), self.get_accessed(), self.get_ps(), self.get_avl(),
self.get_paddr())
		} else {
			write!(f, "P: {} | R/W: {} | U/S: {} | PWT: {} | PCD: {} | A: {} | D: {} | PS: {} | G: {} | AVL: {:#010x} | PAT: {} | RSVD: {} | Address: {:#010x}", self.get_present(), self.get_writable(), self.get_user_supervisor(),
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
			return 0;
		} else {
			((self.value & 0b01000000) >> 6) as u8
		}
	}

	pub fn get_global(&self) -> u8 {
		if self.get_ps() == 0 {
			return 0;
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

	pub fn get_pat(&self) -> u8 {
		if self.get_ps() == 0 {
			return 0;
		} else {
			((self.value & 0b1000000000000) >> 12) as u8
		}
	}

	pub fn get_rsvd(&self) -> u8 {
		if self.get_ps() == 0 {
			return 0;
		} else {
			((self.value & 0b100000000000000000000) >> 20) as u8
		}
	}

	pub fn get_paddr(&self) -> PhysAddr {
		if self.get_ps() == 0 {
			self.value & 0xfffff000
		} else {
			((self.value & 0xfff00000) >> 10) | ((self.value & 0b111111110000000000000) << 19)
		}
	}

	pub fn get_vaddr(&self) -> VirtAddr {
		self as *const Self as VirtAddr
	}
}
