pub mod ide;

pub trait DiskIO {
	fn read_sectors(
		&mut self,
		numsects: u8,
		lba: u32,
		edi: u32
	) -> Result<(), u8>;

	fn write_sectors(
		&mut self,
		numsects: u8,
		lba: u32,
		edi: u32
	) -> Result<(), u8>;

    fn sector_size(&self) -> usize;
}


