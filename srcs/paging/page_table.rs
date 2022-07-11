use core::fmt;

#[repr(align(4096))]
pub struct PageTable {
	pub entries: [PageTableEntry; 1024]
}

impl PageTable {
	pub const fn new() -> Self {
		Self {entries: [PageTableEntry::new(0x0); 1024]}
	}

	pub fn new_frame(&mut self, page_frame: u32) -> Result<u16, ()> {
		let mut i: usize = 0;

		while i < 1024 {
			if self.entries[i].get_present() != 1 {
				self.entries[i] = (page_frame | 3).into();
				return Ok(i as u16);
			}
			i += 1;
		}
		Err(())
	}

}

#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct PageTableEntry {
	value: u32
}

impl From<u32> for PageTableEntry {
	fn from(item: u32) -> Self {
		PageTableEntry { value: item }
	}
}

impl PageTableEntry {
	pub const fn new(value: u32) -> Self {
		Self {value: value}
	}
}

impl fmt::Display for PageTableEntry {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Present: {}
Writable: {}
User/Supervisor: {}
PWT: {}
PCD: {}
Accessed: {}
Dirty: {}
PAT: {}
Global: {}
AVL: 0x{:x}
Address: {:#010x}", self.get_present(), self.get_writable(), self.get_user_supervisor(),
self.get_pwt(), self.get_pcd(), self.get_accessed(), self.get_dirty(), self.get_pat(),
self.get_global(), self.get_avl(), self.get_address())
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

	pub fn get_address (&self) -> u32 {
		self.value & 0xfffff000
	}

	pub fn free_entry(&mut self) {
		self.value =0 ;
	}
}
