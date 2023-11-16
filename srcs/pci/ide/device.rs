use crate::utils::arcm::Arcm;
pub use super::channel::IDEChannelRegisters;
use crate::kprintln;
use core::ffi::CStr;
use super::ata::{
    ATAError,
	ATADirection,
	ATAReg,
	ATA
};
use super::atapi::{self, ATAPI};
use super::IDEType;
use core::cell::RefCell;


#[derive(Clone)]
pub struct IDEDevice {
	pub reserved:     u8, // 0 (Empty) or 1 (This Drive really exists)
	pub channel:      Option<Arcm<RefCell<IDEChannelRegisters>>>,
	pub drive:        u8,       // 0 (Master Drive) or 1 (Slave Drive)
	pub r#type:       u16,      // 0: ATA, 1:ATAPI
	pub signature:    u16,      // Drive Signature
	pub capabilities: u16,      // Features
	pub command_sets: u32,      // Command Sets Supported
	pub size:         u32,      // Size in Sectors
	pub model:        [u8; 41]  // Model in string
}

impl IDEDevice {
	pub const fn new() -> Self {
		Self {
			reserved:     0,
			channel:      None,
			drive:        0,
			r#type:       0,
			signature:    0,
			capabilities: 0,
			command_sets: 0,
			size:         0,
			model:        [0; 41]
		}
	}

	pub fn print_error(&self, mut err: u8) -> u8 {

		if err == 0 {
			return err;
		}
		kprintln!("IDE:");
		let binding = match &self.channel {
			Some(x) => x,
			None => {
				kprintln!("- Channel non-initialized");
				return 23;
			}
		};
        let bind = binding.lock();
		let channel: &mut IDEChannelRegisters = &mut bind.borrow_mut();
		match err {
			1 => {
				kprintln!("- Device Fault");
				err = 19;
			},
			2 => {
				let st: u8 = channel.read(ATAReg::ERROR);
				if (st & ATAError::AMNF) != 0 {
					kprintln!("- No Address Mark Found");
					err = 7;
				}
				if (st & ATAError::ABRT) != 0 {
					kprintln!("- Command Aborted");
					err = 20;
				}
				if ((st & ATAError::TK0NF) != 0)
					| ((st & ATAError::MCR) != 0)
					| ((st & ATAError::MC) != 0)
				{
					kprintln!("- No Media or Media Error");
					err = 3;
				}
				if (st & ATAError::IDNF) != 0 {
					kprintln!("- ID mark not Found");
					err = 21;
				}
				if (st & ATAError::UNC) != 0 {
					kprintln!("- Uncorrectable Data Error");
					err = 22;
				}
				if (st & ATAError::BBK) != 0 {
					kprintln!("- Bad Sectors");
					err = 13;
				}
			},
			3 => {
				kprintln!("- Reads Nothing");
				err = 23;
			},
			4 => {
				kprintln!("- Write Protected");
				err = 8;
			},
			_ => {}
		}
		kprintln!(
			"    - [{} {}] {}",
			["Primary", "Secondary"][channel.r#type as usize],
			["Master", "Slave"][self.drive as usize],
			CStr::from_bytes_until_nul(&self.model)
				.unwrap()
				.to_str()
				.unwrap()
		);
		err
	}

	/// Read sector from a device
	///
	/// Parameters:
	/// + numsects: number of sectors to be read. If 0, the ATA controller will now we want 256 sectors
	/// + lba: LBA address --> index of the sector, which allows us to acces disks up to 2TB
	/// + edi: adress of the buffer we want to fill
	pub fn read_sectors(
		&self,
		numsects: u8,
		lba: u32,
		edi: u32
	) -> Result<(), u8> {

		// 1- Check if the drive presents
		if self.reserved == 0 {
			// Drive not found
			return Err(0x1);
		// 2- Check if inputs are valid
		} else if (lba + numsects as u32 > self.size)
			&& (self.r#type == IDEType::ATA as u16)
		{
			// Seeking to invalid position
			return Err(0x2);
		// 3- Read in PIO Mode through Polling & IRQs
		} else {
			if self.r#type == IDEType::ATA as u16 {
				match ATA::access(
					ATADirection::Read as u8,
					self,
					lba,
					numsects,
					edi
				) {
					Ok(_) => {},
					Err(err) => return Err(self.print_error(err))
				}
			} else if self.r#type == IDEType::ATAPI as u16 {
				for i in 0..numsects {
					match ATAPI::read(
						self,
						lba + i as u32,
						1,
						edi + i as u32 * atapi::SECTOR_SIZE
					) {
						Ok(_) => {},
						Err(err) => return Err(self.print_error(err))
					}
				}
			}
		}
		Ok(())
	}

	/// Write sector from a device
	///
	/// Parameters:
	/// + numsects: number of sectors to write. If 0, the ATA controller will now we want 256 sectors
	/// + lba: LBA address --> index of the sector, which allows us to access disks up to 2TB
	/// + edi: adress of the buffer we want to fill
	pub fn write_sectors(
		&mut self,
		numsects: u8,
		lba: u32,
		edi: u32
	) -> Result<(), u8> {

		// 1- Check if the drive presents
		if self.reserved == 0 {
			// Drive not found
			return Err(0x1);
		// 2- Check if inputs are valid
		} else if (lba + numsects as u32 > self.size)
			&& (self.r#type == IDEType::ATA as u16)
		{
			return Err(0x2);
		// 3- Read in PIO Mode through Polling & IRQs
		} else {
			if self.r#type == IDEType::ATA as u16 {
				match ATA::access(
					ATADirection::Write as u8,
					self,
					lba,
					numsects,
					edi
				) {
					Ok(_) => {},
					Err(err) => return Err(self.print_error(err))
				}
			} else if self.r#type == IDEType::ATAPI as u16 {
				// Write-Protected
				return Err(0x4);
			}
		}
		Ok(())
	}



}


