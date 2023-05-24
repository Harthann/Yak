use crate::memory::PhysAddr;

type Sector = u8;

// Hardcoded value corresponding to our paging maximum memory
const MAX_MEM: u64 = 1024 * 1024 * 4096;
pub const PAGE_SIZE: usize = 4096;
const SECTOR_SIZE: usize = PAGE_SIZE * 8;
const SECTOR_NUMBER: usize = (MAX_MEM / SECTOR_SIZE as u64) as usize;

pub static mut PHYSMAP: Bitmaps = Bitmaps::new();

pub struct Bitmaps {
	maps:     [Sector; SECTOR_NUMBER],
	pub used: usize
}

impl Bitmaps {
	pub const fn new() -> Bitmaps {
		Bitmaps { maps: [0; SECTOR_NUMBER], used: 0 }
	}

	// This function claim a specific page and return it or null if already claimed
	pub fn claim(&mut self, addr: PhysAddr) -> Result<PhysAddr, usize> {
		let i: usize = addr as usize / SECTOR_SIZE;
		let shift: u8 = ((addr as usize % SECTOR_SIZE) / PAGE_SIZE) as u8;

		if self.maps[i] & 1 << shift != 0 {
			return Err(i);
		}
		self.maps[i as usize] |= 1 << shift;
		self.used += 1;
		Ok((i * SECTOR_SIZE + (shift as usize) * PAGE_SIZE) as PhysAddr)
	}

	// This function only aim to claim starting memory and thus ignore if memory is already claim
	pub fn claim_range(
		&mut self,
		addr: PhysAddr,
		range: usize
	) -> Result<PhysAddr, usize> {
		let mut i: usize = 0;
		while i < range {
			self.claim(addr + (i * PAGE_SIZE) as u32)?;
			i += 1;
		}
		Ok(addr)
	}

	// Get multiple page_frames that are physically next to each other, return
	// the first physical adress
	pub fn get_pages(&mut self, nb: usize) -> Result<PhysAddr, ()> {
		let mut i: usize = 0;
		let mut saved_i: usize = 0;
		let mut shift: u8 = 0;
		let mut saved_shift: u8 = 0;
		let mut count: usize = 0;

		while count < nb && i < SECTOR_NUMBER {
			if self.maps[i] == 0xff || shift == 8 {
				shift = 0;
				i += 1;
				continue;
			}
			if (self.maps[i] >> shift) & 1 == 0 {
				if count == 0 {
					saved_i = i;
					saved_shift = shift;
				}
				count += 1;
			} else {
				count = 0;
			}
			shift += 1;
		}
		if i == SECTOR_NUMBER {
			return Err(());
		}
		i = saved_i;
		shift = saved_shift;
		while count > 0 {
			self.maps[i] |= 1 << shift;
			self.used += 1;
			shift += 1;
			if shift == 8 {
				shift = 0;
				i += 1;
			}
			count -= 1;
		}
		Ok((saved_i * SECTOR_SIZE + (saved_shift as usize) * PAGE_SIZE)
			as PhysAddr)
	}

	// Return the next claimable frame and return it's physical addres
	pub fn get_page(&mut self) -> Result<PhysAddr, ()> {
		let mut i: usize = 0;
		let mut shift: u8 = 0;

		while self.maps[i] == 0xff {
			i += 1;
		}
		if i == SECTOR_NUMBER {
			// TODO -> if physaddr is 0 ?
			return Err(());
		}
		while (self.maps[i] >> shift) & 1 == 1 {
			shift += 1;
		}
		self.maps[i] |= 1 << shift;
		self.used += 1;
		Ok((i * SECTOR_SIZE + (shift as usize) * PAGE_SIZE) as PhysAddr)
	}

	// indicates to the bitmaps a page is not used anymore
	pub fn free_page(&mut self, addr: PhysAddr) {
		let i: usize = (addr / SECTOR_SIZE as u32) as usize;
		let shift: u8 = (addr % SECTOR_SIZE as u32 / PAGE_SIZE as u32) as u8;
		// If pages is in fact used, free it
		if self.maps[i] & (1 << shift) == (1 << shift) {
			self.used -= 1;
			self.maps[i] &= 0xff ^ (1 << shift);
		}
	}
}

pub fn physmap_as_mut() -> &'static mut Bitmaps {
	unsafe {
		return &mut PHYSMAP;
	}
}

use core::fmt;
impl fmt::Debug for Bitmaps {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("Bitmaps")
			.field("Page size (bytes)", &PAGE_SIZE)
			.field("Sector size (bytes)", &SECTOR_SIZE)
			.field("Sector nb", &SECTOR_NUMBER)
			.field("Used pages", &self.used)
			.finish()
	}
}

#[cfg(test)]
mod test {
	use crate::memory::paging::bitmap::{
		physmap_as_mut,
		PAGE_SIZE,
		SECTOR_SIZE
	};
	use crate::memory::PhysAddr;

	#[test_case]
	fn bitmap_claim() {
		use crate::page_directory;
		crate::print_fn!();
		let physmap = physmap_as_mut();
		let mut x: usize = 0x100000;
		let used = physmap.used;

		unsafe {
			let pd_addr = page_directory.get_vaddr() & 0x3ff000 as PhysAddr;
			let nmb_claim_pages = ((pd_addr / 0x1000) + 1024) as u32;

			// At start the kernel claim kernel code and memory pages to initialize the bitmap
			// claim occur at adress 0x0 to 1MiB then from it to pd_addr / 0x1000 + 1024
			assert_eq!(physmap.claim(0x0), Err(0));
			assert_eq!(used, physmap.used);
			loop {
				match physmap.claim(x as u32) {
					Err(index) => {
						assert_eq!(index, x / SECTOR_SIZE);
						assert_eq!(used, physmap.used);
					},
					Ok(addr) => {
						assert!(
							addr >= 0x100000
								+ nmb_claim_pages * PAGE_SIZE as u32
						);
						break;
					}
				}
				x += PAGE_SIZE;
			}
			assert_eq!(used + 1, physmap.used);
			physmap.free_page(x as u32);
			assert_eq!(used, physmap.used);
		}
	}

	#[test_case]
	fn bitmap_claim_range() {
		use crate::page_directory;
		crate::print_fn!();
		let physmap = physmap_as_mut();
		let mut x: usize = 0x100000;
		let mut used = physmap.used;

		unsafe {
			let pd_addr = page_directory.get_vaddr() & 0x3ff000 as PhysAddr;
			let nmb_claim_pages = ((pd_addr / 0x1000) + 1024) as usize;

			let res = physmap.claim_range(x as u32, nmb_claim_pages);
			assert_eq!(res, Err(x as usize / SECTOR_SIZE as usize));
			assert_eq!(used, physmap.used);

			x = physmap.get_page().unwrap() as usize;
			physmap.free_page(x as u32);

			let res = physmap.claim_range(x as u32, 10);
			assert_eq!(res, Ok(x as u32));
			assert_eq!(used + 10, physmap.used);
		}

		for i in 0..10 {
			used = physmap.used;
			physmap.free_page((x + i * PAGE_SIZE) as u32);
			assert_eq!(used - 1, physmap.used);
		}
	}

	#[test_case]
	fn bitmap_get_page() {
		crate::print_fn!();
		let physmap = physmap_as_mut();
		let mut addresses: [u32; 50] = [0; 50];
		let mut used;

		for i in 0..50 {
			used = physmap.used;
			match physmap.get_page() {
				Err(index) => {
					panic!("Failed to get pages at index: {:?}", index)
				},
				Ok(addr) => addresses[i] = addr
			}
			assert_eq!(used + 1, physmap.used);
		}
		for i in addresses {
			used = physmap.used;
			physmap.free_page(i);
			assert_eq!(used - 1, physmap.used);
		}
	}

	#[test_case]
	fn bitmap_get_pages() {
		crate::print_fn!();
		let physmap = physmap_as_mut();
		let mut used = physmap.used;

		let addr = match physmap.get_pages(50) {
			Err(index) => panic!("Failed to get pages at index: {:?}", index),
			Ok(addr) => addr
		};
		assert_eq!(used + 50, physmap.used);
		for i in 0..50 {
			used = physmap.used;
			physmap.free_page((addr + i * PAGE_SIZE as u32) as u32);
			assert_eq!(used - 1, physmap.used);
		}
		// Here page is already free so the counter shouldn't be decremented
		used = physmap.used;
		physmap.free_page(addr);
		assert_eq!(used, physmap.used);
	}
}
