pub type PhysAddr   = u32;
pub type VirtAddr   = u32;
type Sector     = u8;

/*  Hardcoded value corresponding to our paging maximum memory */
const MAX_MEM:		  u64   = 1024 * 1024 * 4096;
const PAGE_SIZE:		usize = 4096;
const SECTOR_SIZE:	  usize = PAGE_SIZE * 8;
const SECTOR_NUMBER:	usize = (MAX_MEM / SECTOR_SIZE as u64) as usize;

pub static mut PHYSMAP: bitmaps = bitmaps {
		maps: [0; 131072]
};

pub struct bitmaps {
	maps: [Sector; 131072]
}

impl bitmaps {
	pub const fn new() -> bitmaps {
		bitmaps {
			maps: [0;131072],
		}
	}

/*  This function claim a specific page and return it or null if already claimed */
	pub fn claim(&mut self, addr: PhysAddr) -> PhysAddr {
		let i: usize = addr as usize / SECTOR_SIZE;
		let shift: u8 = ((addr as usize % SECTOR_SIZE) / PAGE_SIZE) as u8;

		if self.maps[i] & 1 << shift != 0 {
			return 0x0;
		}
		self.maps[i as usize] |= 1 << shift;
		(i * SECTOR_SIZE + (shift as usize) * PAGE_SIZE ) as PhysAddr
	}

/*  This function only aim to claim starting memory and thus ignore if memory is already claim */
	pub fn claim_range(&mut self, addr: PhysAddr, range: usize) -> PhysAddr {
		let mut i: usize = 0;
		while i < range {
			self.claim(addr + (i * PAGE_SIZE) as u32);
			i += 1;
		}
		addr
	}

/*  Return the next claimable frame and return it's physical addres */
	pub fn get_page(&mut self) -> PhysAddr {
		let mut i: usize = 0;
		let mut shift: u8 = 0;

		while self.maps[i] == 0xff {
			i += 1;
		}
		if i == 131072 {
			return 0x0;
		}
		while (self.maps[i] >> shift) & 1 == 1 {
			shift += 1;
		}
		self.maps[i as usize] |= 1 << shift;
		(i * SECTOR_SIZE + (shift as usize) * PAGE_SIZE) as PhysAddr
	}

/* indicates to the bitmaps a page is not used anymore */
	pub fn free_page(&mut self, addr: PhysAddr) {
		let i: usize = (addr / SECTOR_SIZE as u32) as usize;
		let shift: u8 = (addr % SECTOR_SIZE as u32 / PAGE_SIZE as u32) as u8;
		self.maps[i] &= 0xff ^ (1 << shift);
	}
}

pub fn get_map_mut() -> &'static mut bitmaps {
    unsafe {
        return &mut PHYSMAP;
    }
}
