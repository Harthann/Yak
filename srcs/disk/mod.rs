use crate::alloc::boxed::Box;
use crate::alloc::vec::Vec;
use crate::pci::ide::IDE;

pub mod ide;
use ide::IDEDisk;

pub trait DiskIO {
	fn read_sectors(&self, numsects: u8, lba: u32, edi: u32) -> Result<(), u8>;

	fn write_sectors(
		&mut self,
		numsects: u8,
		lba: u32,
		edi: u32
	) -> Result<(), u8>;

	fn sector_size(&self) -> usize;
}

pub fn discover() -> Vec<Box<dyn DiskIO + Send>> {
	let mut found_disks = Vec::<Box<dyn DiskIO + Send>>::new();

	// Discover IDE disks
	let binding = IDE.lock();
	for i in 0..4 {
		let disk = binding.get_device(i);
		match disk {
			Some(x) => {
				let idedisk = IDEDisk::new(i as u8, x.clone());
				let diskio = Box::new(idedisk);
				found_disks.push(diskio);
			},
			None => {}
		}
	}

	found_disks
}

#[cfg(test)]
mod test {

	use super::ide::IDEDisk;
	use super::DiskIO;
	use crate::alloc::vec;
	use crate::{sys_macros, IDE};

	#[sys_macros::test_case]
	fn idedisk_read_write_sector() {
		let to_write = vec!['C' as u8; 512];
		let read_from = vec![0x0 as u8; 512];

		let device = IDE.lock().get_device(1).unwrap().clone();
		let mut idedisk = IDEDisk::new(1, device);
		let _ = idedisk.write_sectors(1, 0x0, to_write.as_ptr() as u32);
		let _ = idedisk.read_sectors(1, 0x0, read_from.as_ptr() as u32);

		assert_eq!(to_write, read_from);
	}

	#[sys_macros::test_case]
	fn idedisk_read_write_multiple_sectors() {
		let to_write = vec!['D' as u8; 1024];
		let read_from = vec![0x0 as u8; 1024];

		let device = IDE.lock().get_device(1).unwrap().clone();
		let mut idedisk = IDEDisk::new(1, device);
		let _ = idedisk.write_sectors(2, 0x0, to_write.as_ptr() as u32);
		let _ = idedisk.read_sectors(2, 0x0, read_from.as_ptr() as u32);

		assert_eq!(to_write, read_from);
	}
}
