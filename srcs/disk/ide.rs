use crate::pci::ide::{
    self,
    IDEDevice
};
use super::DiskIO;

pub struct IDEDisk {
    diskno: u8,
    device: IDEDevice
}

unsafe impl Send for IDEDisk {}

impl IDEDisk {
    pub const fn new(diskno: u8, device: IDEDevice) -> Self {
        Self {diskno, device}
    }
}

impl DiskIO for IDEDisk {

	 fn read_sectors(
		&self,
		numsects: u8,
		lba: u32,
		edi: u32
	) -> Result<(), u8> {
         self.device.read_sectors(numsects, lba, edi)
	}

    fn write_sectors(
		&mut self,
		numsects: u8,
		lba: u32,
		edi: u32
	) -> Result<(), u8> {
        self.device.write_sectors(numsects, lba, edi)
    }

    fn sector_size(&self) -> usize {
		match self.device.r#type {
			x if x == ide::IDEType::ATA as u16 => ide::ata::SECTOR_SIZE as usize,
			x if x == ide::IDEType::ATAPI as u16 => ide::atapi::SECTOR_SIZE as usize,
			_ => {
				panic!("Unrecognized disk.")
			}
        }
    }
}

