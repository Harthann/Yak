use crate::pci::ide::IDEDevice;
use super::DiskIO;

pub struct IDEDisk {
    diskno: u8,
    device: IDEDevice
}

impl DiskIO for IDEDisk {

	 fn read_sectors(
		&mut self,
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
        self.device.size as usize
    }
}
