use core::fmt;

#[repr(align(4096))]
pub struct PageDirectory {
	pub entries: [PageDirectoryEntry; 1024]
}

impl PageDirectory {
	pub const fn new() -> Self {
		Self {entries: [PageDirectoryEntry::new(0x00000002); 1024]}
	}
}

#[derive(Copy, Clone)]
pub struct PageDirectoryEntry {
	value: u32
}

impl PageDirectoryEntry {
	pub const fn new(value: u32) -> Self {
		Self {value: value}
	}

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
			panic!();
		} else {
			((self.value & 0b01000000) >> 6) as u8
		}
	}

	pub fn get_global(&self) -> u8 {
		if self.get_ps() == 0 {
			panic!();
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

	pub fn get_address(&self) -> u32 {
		if self.get_ps() == 0 {
			self.value & 0xfffff000
		} else {
			((self.value & 0xfff00000) >> 10) | ((self.value & 0b111111110000000000000) << 19)
		}
	}

	pub fn get_pat(&self) -> u8 {
		if self.get_ps() == 0 {
			panic!();
		} else {
			((self.value & 0b1000000000000) >> 12) as u8
		}
	}

	pub fn get_rsvd(&self) -> u8 {
		if self.get_ps() == 0 {
			panic!();
		} else {
			((self.value & 0b100000000000000000000) >> 20) as u8
		}
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
self.get_address())
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
self.get_global(), self.get_avl(), self.get_pat(), self.get_rsvd(), self.get_address())
		}
	}
}

#[repr(align(4096))]
pub struct PageTable {
	pub entries: [PageTableEntry; 1024]
}

impl PageTable {
	pub const fn new() -> Self {
		Self {entries: [PageTableEntry::new(0x0); 1024]}
	}
}

#[derive(Copy, Clone)]
pub struct PageTableEntry {
	value: u32
}

impl PageTableEntry {
	pub const fn new(value: u32) -> Self {
		Self {value: value}
	}

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
