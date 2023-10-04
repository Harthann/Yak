// Group Descriptor Table

#[derive(Debug)]
pub struct GdtEntry {
	pub bitmap_block:   u32,
	pub bitmap_inode:   u32,
	pub inode_table:    u32,
	pub unalloc_block:  u16,
	pub unalloc_inodes: u16,
	pub dir_count:      u16,
	unused:             [u8; 14]
}

impl From<&[u8]> for GdtEntry {
	fn from(buffer: &[u8]) -> Self {
		if buffer.len() != 32 {
			panic!("Wrong size while reading gdt entry on Ext2");
		}
		Self {
			bitmap_block:   u32::from_le_bytes(
				buffer[0..4].try_into().unwrap()
			),
			bitmap_inode:   u32::from_le_bytes(
				buffer[4..8].try_into().unwrap()
			),
			inode_table:    u32::from_le_bytes(
				buffer[8..12].try_into().unwrap()
			),
			unalloc_block:  u16::from_le_bytes(
				buffer[12..14].try_into().unwrap()
			),
			unalloc_inodes: u16::from_le_bytes(
				buffer[14..16].try_into().unwrap()
			),
			dir_count:      u16::from_le_bytes(
				buffer[16..18].try_into().unwrap()
			),
			unused:         [0; 14]
		}
	}
}
