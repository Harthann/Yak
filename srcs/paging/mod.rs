#[repr(align(4096))]
pub struct PageDirectory {
	pub entries: [*mut PageTable; 1024]
}

impl PageDirectory {
	pub const fn new() -> PageDirectory {
		PageDirectory {
			entries: [0x00000002 as *mut _; 1024]
		}
	}
}

#[repr(align(4096))]
pub struct PageTable {
	pub entries: [u32; 1024]
}

impl PageTable {
	pub const fn new() -> PageTable {
		let mut new = PageTable {
			entries: [0x0 as u32; 1024]
		};
		let mut i = 0;
		while i < 1024 {
			new.entries[i] = ((i * 0x1000) | 3) as u32;
			i += 1;
		}
		new
	}
}
