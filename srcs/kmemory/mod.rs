pub type PhysAddr   = u32;
pub type VirtAddr   = u32;
type Sector     = u8;

/*  Hardcoded value corresponding to our paging maximum memory */
const MAX_MEM:			u64   = 1024 * 1024 * 4096;
const PAGE_SIZE:		usize = 4096;
const SECTOR_SIZE:		usize = PAGE_SIZE * 8;
const SECTOR_NUMBER:	usize = (MAX_MEM / SECTOR_SIZE as u64) as usize;

pub static mut PHYSMAP: Bitmaps = Bitmaps {
		maps: [0; SECTOR_NUMBER]
};

pub struct Bitmaps {
	maps: [Sector; SECTOR_NUMBER]
}

impl Bitmaps {
	pub const fn new() -> Bitmaps {
		Bitmaps {
			maps: [0; SECTOR_NUMBER],
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

	/*
		Get multiple page_frames that are physically next to each other, return 
		the first physical adress
	*/
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
				continue ;
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
			shift += 1;
			if shift == 8 {
				shift = 0;
				i += 1;
			}
			count -= 1;
		}
		Ok((saved_i * SECTOR_SIZE + (saved_shift as usize) * PAGE_SIZE) as PhysAddr)
	}

/*  Return the next claimable frame and return it's physical addres */
	pub fn get_page(&mut self) -> Result<PhysAddr, ()> {
		let mut i: usize = 0;
		let mut shift: u8 = 0;

		while self.maps[i] == 0xff {
			i += 1;
		}
		if i == SECTOR_NUMBER { // TODO -> if physaddr is 0 ?
			return Err(());
		}
		while (self.maps[i] >> shift) & 1 == 1 {
			shift += 1;
		}
		self.maps[i] |= 1 << shift;
		Ok((i * SECTOR_SIZE + (shift as usize) * PAGE_SIZE) as PhysAddr)
	}

/* indicates to the bitmaps a page is not used anymore */
	pub fn free_page(&mut self, addr: PhysAddr) {
		let i: usize = (addr / SECTOR_SIZE as u32) as usize;
		let shift: u8 = (addr % SECTOR_SIZE as u32 / PAGE_SIZE as u32) as u8;
		self.maps[i] &= 0xff ^ (1 << shift);
	}
}

pub fn physmap_as_mut() -> &'static mut Bitmaps {
    unsafe {
        return &mut PHYSMAP;
    }
}
