use core::ffi::CStr;
use core::mem::{size_of, transmute};
use core::ptr::copy;

use crate::io::{inb, insl, outb};
use crate::kprintln;
use crate::time::sleep;

mod ata;
mod atapi;

use ata::{
	ATAChannel,
	ATACommand,
	ATADirection,
	ATAError,
	ATAIdentify,
	ATAReg,
	ATAStatus,
	ATA
};
use atapi::ATAPI;

static mut IDE_IRQ_INVOKED: u8 = 0;
pub static mut IDE: IDEController = IDEController::new();

enum IDEType {
	ATA   = 0x00,
	ATAPI = 0x01
}

#[derive(Clone, Copy)]
struct IDEChannelRegisters {
	base:  u16, // I/O Base
	ctrl:  u16, // ControlBase
	bmide: u16, // Bus Master IDE
	n_ien: u8   // nIEN (No Interrupt)
}

impl IDEChannelRegisters {
	const fn new() -> Self {
		Self { base: 0, ctrl: 0, bmide: 0, n_ien: 0 }
	}
}

#[derive(Clone, Copy)]
struct IDEDevice {
	reserved:     u8,       // 0 (Empty) or 1 (This Drive really exists)
	channel:      u8,       // 0 (Primary Channel) or 1 (Secondary Channel)
	drive:        u8,       // 0 (Master Drive) or 1 (Slave Drive)
	r#type:       u16,      // 0: ATA, 1:ATAPI
	signature:    u16,      // Drive Signature
	capabilities: u16,      // Features
	command_sets: u32,      // Command Sets Supported
	size:         u32,      // Size in Sectors
	model:        [u8; 41]  // Model in string
}

impl IDEDevice {
	const fn new() -> Self {
		Self {
			reserved:     0,
			channel:      0,
			drive:        0,
			r#type:       0,
			signature:    0,
			capabilities: 0,
			command_sets: 0,
			size:         0,
			model:        [0; 41]
		}
	}
}

pub struct IDEController {
	channels: [IDEChannelRegisters; 2],
	devices: [IDEDevice; 4]
}

impl IDEController {
	pub const fn new() -> Self {
		Self {
			channels: [IDEChannelRegisters::new(); 2],
			devices: [IDEDevice::new(); 4]
		}
	}

	pub fn initialize(
		bar0: u32,
		bar1: u32,
		bar2: u32,
		bar3: u32,
		bar4: u32
	) {
		unsafe {
			IDE.init(bar0, bar1, bar2, bar3, bar4)
				.expect("Error while reading disks");
		}
	}

	pub fn init(
		&mut self,
		bar0: u32,
		bar1: u32,
		bar2: u32,
		bar3: u32,
		bar4: u32
	) -> Result<(), u8> {
		let mut ide_buf: [u8; 2048] = [0; 2048];

		// 1- Detect I/O Ports which interface IDE Controller
		self.channels[ATAChannel::Primary as usize].base =
			(bar0 & 0xfffffffc) as u16;
		self.channels[ATAChannel::Primary as usize].ctrl =
			(bar1 & 0xfffffffc) as u16;
		self.channels[ATAChannel::Secondary as usize].base =
			(bar2 & 0xfffffffc) as u16;
		self.channels[ATAChannel::Secondary as usize].ctrl =
			(bar3 & 0xfffffffc) as u16;
		self.channels[ATAChannel::Primary as usize].bmide =
			((bar4 & 0xfffffffc) + 0) as u16;
		self.channels[ATAChannel::Secondary as usize].bmide =
			((bar4 & 0xfffffffc) + 8) as u16;

		// 2- Disable IRQs
		IDEController::write(&self.channels[ATAChannel::Primary as usize], ATAReg::CONTROL, 2);
		IDEController::write(
			&self.channels[ATAChannel::Secondary as usize],
			ATAReg::CONTROL,
			2
		);

		let mut count: usize = 0;
		// 3- Detect ATA-ATAPI Devices
		for i in 0..2 {
			for j in 0..2 {
				let mut err: u8 = 0;
				let mut r#type: u8 = IDEType::ATA as u8;
				let channel: &IDEChannelRegisters = &self.channels[i];

				// Assuming that no drive here
				self.devices[count].reserved = 0;

				// (I) Select Drive
				IDEController::write(channel, ATAReg::HDDEVSEL, 0xa0 | (j << 4));
				sleep(1);

				// (II) Send ATA Identify Command
				IDEController::write(
					channel,
					ATAReg::COMMAND,
					ATACommand::Identify as u8
				);
				sleep(1);

				// (III) Polling
				// If Status = 0, No Device
				if IDEController::read(channel, ATAReg::STATUS) == 0 {
					continue;
				}

				loop {
					let status: u8 = IDEController::read(channel, ATAReg::STATUS);
					if (status & ATAStatus::ERR) != 0 {
						err = 1;
						break;
					}
					if ((status & ATAStatus::BSY) == 0)
						&& ((status & ATAStatus::DRQ) != 0)
					{
						break;
					}
				}

				// (IV) Probe for ATAPI Devices
				if err != 0 {
					let cl: u8 = IDEController::read(channel, ATAReg::LBA1);
					let ch: u8 = IDEController::read(channel, ATAReg::LBA2);

					if cl == 0x14 && ch == 0xeb {
						r#type = IDEType::ATAPI as u8;
					} else if cl == 0x69 && ch == 0x96 {
						r#type = IDEType::ATAPI as u8;
					} else {
						// Unknown Type (may not be a device)
						continue;
					}

					IDEController::write(
						channel,
						ATAReg::COMMAND,
						ATACommand::IdentifyPacket as u8
					);
					sleep(1);
				}

				// (V) Read Identification Space of the Device
				IDEController::read_buffer(
					channel,
					ATAReg::DATA,
					unsafe {ide_buf.align_to_mut::<u32>().1},
					128
				);

				// (VI) Read Device Parameters
				self.devices[count].reserved = 1;
				self.devices[count].r#type = r#type as u16;
				self.devices[count].channel = i as u8;
				self.devices[count].drive = j;
				unsafe {
					copy(
						ide_buf.as_ptr().offset(ATAIdentify::DEVICETYPE as isize),
						transmute(&mut self.devices[count].signature),
						size_of::<u16>()
					);
					copy(
						ide_buf.as_ptr().offset(ATAIdentify::CAPABILITIES as isize),
						transmute(&mut self.devices[count].capabilities),
						size_of::<u16>()
					);
					copy(
						ide_buf.as_ptr().offset(ATAIdentify::COMMANDSETS as isize),
						transmute(&mut self.devices[count].command_sets),
						size_of::<u32>()
					);
				}

				if self.devices[count].r#type == IDEType::ATA as u16 {
					// (VII) Get Size
					if (self.devices[count].command_sets & (1 << 26)) != 0 {
						// Device uses 48-Bit Addressing
						unsafe {
							copy(
								ide_buf
									.as_ptr()
									.offset(ATAIdentify::MAX_LBA_EXT as isize),
								transmute(&mut self.devices[count].size),
								size_of::<u32>()
							);
						}
					} else {
						// Device uses CHS or 28-Bit Addressing
						unsafe {
							copy(
								ide_buf
									.as_ptr()
									.offset(ATAIdentify::MAX_LBA as isize),
								transmute(&mut self.devices[count].size),
								size_of::<u32>()
							);
						}
					}
				} else {
					self.devices[count].size = ATAPI::capacity(i as u8, 0)?;
				}

				// (VIII) String indicates model of device
				for k in (0..40).step_by(2) {
					self.devices[count].model[k] =
						ide_buf[ATAIdentify::MODEL as usize + k + 1];
					self.devices[count].model[k + 1] =
						ide_buf[ATAIdentify::MODEL as usize + k];
				}
				self.devices[count].model[40] = 0;

				count += 1;
			}
		}

		// 4- Print Summary
		for i in 0..4 {
			if self.devices[i].reserved == 1 {
				kprintln!(
					"Found {} Drive {:.2}MB - {}",
					["ATA", "ATAPI"][self.devices[i].r#type as usize],
					self.devices[i].size as f32 / 1024.0 / 2.0,
					CStr::from_bytes_until_nul(&self.devices[i].model)
						.unwrap()
						.to_str()
						.unwrap()
				);
			}
		}

		Ok(())
	}

	unsafe fn wait_irq() {
		loop {
			if IDE_IRQ_INVOKED != 0 {
				break;
			}
		}
		IDE_IRQ_INVOKED = 0;
	}

	unsafe fn irq() {
		IDE_IRQ_INVOKED = 1;
	}

	fn read(channel: &IDEChannelRegisters, reg: u8) -> u8 {
		let mut result: u8 = 0;
		if reg > 0x07 && reg < 0x0c {
			IDEController::write(channel, ATAReg::CONTROL, 0x80 | channel.n_ien);
		}
		if reg < 0x08 {
			result = inb(channel.base + reg as u16 - 0x00);
		} else if reg < 0x0c {
			result = inb(channel.base + reg as u16 - 0x06);
		} else if reg < 0x0e {
			result = inb(channel.ctrl + reg as u16 - 0x0a);
		} else if reg < 0x16 {
			result = inb(channel.bmide + reg as u16 - 0x0e);
		}
		if reg > 0x07 && reg < 0x0c {
			IDEController::write(channel, ATAReg::CONTROL, channel.n_ien);
		}
		return result;
	}

	fn write(channel: &IDEChannelRegisters, reg: u8, data: u8) {
		if reg > 0x07 && reg < 0x0c {
			IDEController::write(channel, ATAReg::CONTROL, 0x80 | channel.n_ien);
		}
		if reg < 0x08 {
			outb(channel.base + reg as u16 - 0x00, data);
		} else if reg < 0x0c {
			outb(channel.base + reg as u16 - 0x06, data);
		} else if reg < 0x0e {
			outb(channel.ctrl + reg as u16 - 0x0a, data);
		} else if reg < 0x16 {
			outb(channel.bmide + reg as u16 - 0x0e, data);
		}
		if reg > 0x07 && reg < 0x0c {
			IDEController::write(channel, ATAReg::CONTROL, channel.n_ien);
		}
	}

	fn read_buffer(
		channel: &IDEChannelRegisters,
		reg: u8,
		buffer: &mut [u32],
		quads: u32
	) {
		if reg > 0x07 && reg < 0x0c {
			IDEController::write(channel, ATAReg::CONTROL, 0x80 | channel.n_ien);
		}
		if reg < 0x08 {
			insl(channel.base + reg as u16 - 0x00, buffer.as_mut_ptr(), quads);
		} else if reg < 0x0c {
			insl(channel.base + reg as u16 - 0x06, buffer.as_mut_ptr(), quads);
		} else if reg < 0x0e {
			insl(channel.ctrl + reg as u16 - 0x0a, buffer.as_mut_ptr(), quads);
		} else if reg < 0x16 {
			insl(channel.bmide + reg as u16 - 0x0e, buffer.as_mut_ptr(), quads);
		}
		if reg > 0x07 && reg < 0x0c {
			IDEController::write(channel, ATAReg::CONTROL, channel.n_ien);
		}
	}

	fn polling(
		channel: &IDEChannelRegisters,
		advanced_check: u32
	) -> Result<(), u8> {
		// (I) Delay 400 nanosecond for BSY to be set
		// Reading port wastes 100ns
		for _ in 0..4 {
			IDEController::read(channel, ATAReg::ALTSTATUS);
		}

		// (II) Wait for BSY to be cleared
		while (IDEController::read(channel, ATAReg::STATUS) & ATAStatus::BSY as u8) != 0 {
			// Wait for BSY to be zero
		}

		if advanced_check != 0 {
			// Read Status Register
			let state: u8 = IDEController::read(channel, ATAReg::STATUS);

			// (III) Check for errors
			if (state & ATAStatus::ERR) != 0 {
				return Err(2);
			}

			// (IV) Check if device fault
			if (state & ATAStatus::DF) != 0 {
				return Err(1);
			}

			// (V) Check DRQ
			// BSY = 0; DF = 0; Err = 0; So we should check for DRQ now
			if (state & ATAStatus::DRQ) == 0 {
				return Err(3);
			}
		}
		// No Error
		Ok(())
	}

	fn print_error(drive: u8, mut err: u8) -> u8 {
		let device: &IDEDevice = unsafe {IDE.get_device(drive)};
		if err == 0 {
			return err;
		}
		kprintln!("IDE:");
		match err {
			1 => {
				kprintln!("- Device Fault");
				err = 19;
			},
			2 => {
				let st: u8 = IDEController::read(
					unsafe {IDE.get_channel(drive)},
					ATAReg::ERROR
				);
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
			["Primary", "Secondary"]
				[device.channel as usize],
			["Master", "Slave"][device.drive as usize],
			CStr::from_bytes_until_nul(&device.model)
				.unwrap()
				.to_str()
				.unwrap()
		);
		err
	}

	/// Read sector from a drive
	///
	/// Parameters:
	/// + drive: drive number which can be from 0 to 3
	/// + numsects: number of sectors to be read. If 0, the ATA controller will now we want 256 sectors
	/// + lba: LBA address --> index of the sector, which allows us to acces disks up to 2TB
	/// + edi: adress of the buffer we want to fill
	pub fn read_sectors(
		drive: u8,
		numsects: u8,
		lba: u32,
		edi: u32
	) -> Result<(), u8> {
		let device: &IDEDevice = unsafe {IDE.get_device(drive)};
		// 1- Check if the drive presents
		if drive > 3 || device.reserved == 0 {
			// Drive not found
			return Err(0x1);
		// 2- Check if inputs are valid
		} else if (lba + numsects as u32 > device.size)
			&& (device.r#type == IDEType::ATA as u16)
		{
			// Seeking to invalid position
			return Err(0x2);
		// 3- Read in PIO Mode through Polling & IRQs
		} else {
			if device.r#type == IDEType::ATA as u16 {
				match ATA::access(
					ATADirection::Read as u8,
					drive,
					lba,
					numsects,
					edi
				) {
					Ok(_) => {},
					Err(err) => return Err(IDEController::print_error(drive, err))
				}
			} else if device.r#type
				== IDEType::ATAPI as u16
			{
				for i in 0..numsects {
					match ATAPI::read(
						drive as u8,
						lba + i as u32,
						1,
						edi + i as u32 * 2048
					) {
						Ok(_) => {},
						Err(err) => {
							return Err(IDEController::print_error(drive, err))
						},
					}
				}
			}
		}
		Ok(())
	}

	/// Read sector from a drive
	///
	/// Parameters:
	/// + drive: drive number which can be from 0 to 3
	/// + numsects: number of sectors to write. If 0, the ATA controller will now we want 256 sectors
	/// + lba: LBA address --> index of the sector, which allows us to access disks up to 2TB
	/// + edi: adress of the buffer we want to fill
	pub fn write_sectors(
		drive: u8,
		numsects: u8,
		lba: u32,
		edi: u32
	) -> Result<(), u8> {
		let device: &IDEDevice = unsafe {IDE.get_device(drive)};
		// 1- Check if the drive presents
		if drive > 3 || device.reserved == 0 {
			// Drive not found
			return Err(0x1);
		// 2- Check if inputs are valid
		} else if (lba + numsects as u32 > device.size)
			&& (device.r#type == IDEType::ATA as u16)
		{
			return Err(0x2);
		// 3- Read in PIO Mode through Polling & IRQs
		} else {
			if device.r#type == IDEType::ATA as u16 {
				match ATA::access(
					ATADirection::Write as u8,
					drive,
					lba,
					numsects,
					edi
				) {
					Ok(_) => {},
					Err(err) => return Err(IDEController::print_error(drive, err))
				}
			} else if device.r#type
				== IDEType::ATAPI as u16
			{
				// Write-Protected
				return Err(0x4);
			}
		}
		Ok(())
	}

	fn get_device(&mut self, drive: u8) -> &mut IDEDevice {
		&mut self.devices[drive as usize]
	}

	fn get_channel(&mut self, drive: u8) -> &mut IDEChannelRegisters {
		&mut self.channels[self.devices[drive as usize].channel as usize]
	}
}

#[cfg(test)]
mod test {

	use crate::alloc::vec;
	use crate::{sys_macros, IDEController};

	#[sys_macros::test_case]
	fn ide_read_write_sector() {
		let to_write = vec!['B' as u8; 512];
		let read_from = vec![0x0 as u8; 512];

		let _ = IDEController::write_sectors(1, 1, 0x0, to_write.as_ptr() as u32);
		let _ = IDEController::read_sectors(1, 1, 0x0, read_from.as_ptr() as u32);

		assert_eq!(to_write, read_from);
	}

	#[sys_macros::test_case]
	fn ide_read_write_multiple_sectors() {
		let to_write = vec!['A' as u8; 1024];
		let read_from = vec![0x0 as u8; 1024];

		let _ = IDEController::write_sectors(1, 2, 0x0, to_write.as_ptr() as u32);
		let _ = IDEController::read_sectors(1, 2, 0x0, read_from.as_ptr() as u32);

		assert_eq!(to_write, read_from);
	}
}
