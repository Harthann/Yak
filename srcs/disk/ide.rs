use super::DiskIO;
use crate::pci::ide::IDEDevice;

pub struct IDEDisk {
	diskno: u8,
	device: IDEDevice
}

unsafe impl Send for IDEDisk {}

impl IDEDisk {
	pub const fn new(diskno: u8, device: IDEDevice) -> Self {
		Self { diskno, device }
	}
}

impl DiskIO for IDEDisk {
	fn read_sectors(&self, numsects: u8, lba: u32, edi: u32) -> Result<(), u8> {
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
		self.device.sector_size() as usize
	}
}
