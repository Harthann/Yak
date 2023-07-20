use core::ffi::CStr;
use core::mem::size_of;

use crate::io::{inb, insl, outb};
use crate::kprintln;
use crate::spin::{KMutex, Mutex};
use crate::time::sleep;
use crate::utils::arcm::Arcm;

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

static IDE_IRQ_INVOKED: Mutex<u8> = Mutex::<u8>::new(0);
pub static IDE: KMutex<IDEController> =
	KMutex::<IDEController>::new(IDEController::new());

enum IDEType {
	ATA   = 0x00,
	ATAPI = 0x01
}

#[derive(Clone, Copy)]
struct IDEChannelRegisters {
	r#type: ATAChannel, // 0 - Primary Channel, 1 - Secondary Channel
	base:   u16,        // I/O Base
	ctrl:   u16,        // ControlBase
	bmide:  u16,        // Bus Master IDE
	n_ien:  u8          // nIEN (No Interrupt)
}

impl IDEChannelRegisters {
	const fn new(channel: ATAChannel) -> Self {
		Self { r#type: channel, base: 0, ctrl: 0, bmide: 0, n_ien: 0 }
	}
}

pub struct IDEDevice {
	reserved:     u8, // 0 (Empty) or 1 (This Drive really exists)
	channel:      Option<Arcm<IDEChannelRegisters>>,
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
}

pub struct IDEController {
	devices: [IDEDevice; 4]
}

impl IDEController {
	pub const fn new() -> Self {
		Self {
			devices: [
				IDEDevice::new(),
				IDEDevice::new(),
				IDEDevice::new(),
				IDEDevice::new()
			]
		}
	}

	pub fn initialize(
		&mut self,
		bar0: u32,
		bar1: u32,
		bar2: u32,
		bar3: u32,
		bar4: u32
	) -> Result<(), u8> {
		let mut ide_buf: [u8; 2048] = [0; 2048];

		let channels: [Arcm<IDEChannelRegisters>; 2] = [
			Arcm::new(IDEChannelRegisters::new(ATAChannel::Primary)),
			Arcm::new(IDEChannelRegisters::new(ATAChannel::Secondary))
		];
		// 1- Detect I/O Ports which interface IDE Controller
		channels[ATAChannel::Primary as usize].lock().base =
			(bar0 & 0xfffffffc) as u16;
		channels[ATAChannel::Primary as usize].lock().ctrl =
			(bar1 & 0xfffffffc) as u16;
		channels[ATAChannel::Secondary as usize].lock().base =
			(bar2 & 0xfffffffc) as u16;
		channels[ATAChannel::Secondary as usize].lock().ctrl =
			(bar3 & 0xfffffffc) as u16;
		channels[ATAChannel::Primary as usize].lock().bmide =
			(bar4 & 0xfffffffc) as u16;
		channels[ATAChannel::Secondary as usize].lock().bmide =
			((bar4 & 0xfffffffc) + 8) as u16;

		// 2- Disable IRQs
		IDEController::write(
			&channels[ATAChannel::Primary as usize].lock(),
			ATAReg::CONTROL,
			2
		);
		IDEController::write(
			&channels[ATAChannel::Secondary as usize].lock(),
			ATAReg::CONTROL,
			2
		);

		let mut count: usize = 0;
		// 3- Detect ATA-ATAPI Devices
		for (i, channel) in channels.iter().enumerate() {
			for j in 0..2 {
				let mut err: u8 = 0;
				let mut r#type: u8 = IDEType::ATA as u8;

				// Assuming that no drive here
				self.devices[count].reserved = 0;

				// (I) Select Drive
				IDEController::write(
					&channel.lock(),
					ATAReg::HDDEVSEL,
					0xa0 | (j << 4)
				);
				sleep(1);

				// (II) Send ATA Identify Command
				IDEController::write(
					&channel.lock(),
					ATAReg::COMMAND,
					ATACommand::Identify as u8
				);
				sleep(1);

				// (III) Polling
				// If Status = 0, No Device
				if IDEController::read(&channel.lock(), ATAReg::STATUS) == 0 {
					continue;
				}

				loop {
					let status: u8 =
						IDEController::read(&channel.lock(), ATAReg::STATUS);
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
					let cl: u8 =
						IDEController::read(&channel.lock(), ATAReg::LBA1);
					let ch: u8 =
						IDEController::read(&channel.lock(), ATAReg::LBA2);

					if (cl == 0x14 && ch == 0xeb) || (cl == 0x69 && ch == 0x96)
					{
						r#type = IDEType::ATAPI as u8;
					} else {
						// Unknown Type (may not be a device)
						continue;
					}

					IDEController::write(
						&channel.lock(),
						ATAReg::COMMAND,
						ATACommand::IdentifyPacket as u8
					);
					sleep(1);
				}

				// (V) Read Identification Space of the Device
				IDEController::read_buffer(
					&channel.lock(),
					ATAReg::DATA,
					unsafe { ide_buf.align_to_mut::<u32>().1 },
					128
				);

				// (VI) Read Device Parameters
				self.devices[count].reserved = 1;
				self.devices[count].r#type = r#type as u16;
				self.devices[count].channel = Some(channel.clone());
				self.devices[count].drive = j;
				self.devices[count].signature = u16::from_le_bytes(
					ide_buf[ATAIdentify::DEVICETYPE
						..ATAIdentify::DEVICETYPE + size_of::<u16>()]
						.try_into()
						.unwrap()
				);
				self.devices[count].capabilities = u16::from_le_bytes(
					ide_buf[ATAIdentify::CAPABILITIES
						..ATAIdentify::CAPABILITIES + size_of::<u16>()]
						.try_into()
						.unwrap()
				);
				self.devices[count].command_sets = u32::from_le_bytes(
					ide_buf[ATAIdentify::COMMANDSETS
						..ATAIdentify::COMMANDSETS + size_of::<u32>()]
						.try_into()
						.unwrap()
				);

				if self.devices[count].r#type == IDEType::ATA as u16 {
					// (VII) Get Size
					if (self.devices[count].command_sets & (1 << 26)) != 0 {
						// Device uses 48-Bit Addressing
						self.devices[count].size = u32::from_le_bytes(
							ide_buf[ATAIdentify::MAX_LBA_EXT
								..ATAIdentify::MAX_LBA_EXT + size_of::<u32>()]
								.try_into()
								.unwrap()
						);
					} else {
						// Device uses CHS or 28-Bit Addressing
						self.devices[count].size = u32::from_le_bytes(
							ide_buf[ATAIdentify::MAX_LBA
								..ATAIdentify::MAX_LBA + size_of::<u32>()]
								.try_into()
								.unwrap()
						);
					}
				} else {
					let device: &mut IDEDevice = &mut self.devices[i];
					self.devices[count].size = ATAPI::capacity(device, 0)?;
				}

				// (VIII) String indicates model of device
				for k in (0..40).step_by(2) {
					self.devices[count].model[k] =
						ide_buf[ATAIdentify::MODEL + k + 1];
					self.devices[count].model[k + 1] =
						ide_buf[ATAIdentify::MODEL + k];
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

	fn wait_irq() {
		loop {
			if *IDE_IRQ_INVOKED.lock() != 0 {
				break;
			}
		}
		*IDE_IRQ_INVOKED.lock() = 0;
	}

	fn irq() {
		*IDE_IRQ_INVOKED.lock() = 1;
	}

	fn read(channel: &IDEChannelRegisters, reg: u8) -> u8 {
		let mut result: u8 = 0;
		if reg > 0x07 && reg < 0x0c {
			IDEController::write(
				channel,
				ATAReg::CONTROL,
				0x80 | channel.n_ien
			);
		}
		if reg < 0x08 {
			result = inb(channel.base + reg as u16);
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
		result
	}

	fn write(channel: &IDEChannelRegisters, reg: u8, data: u8) {
		if reg > 0x07 && reg < 0x0c {
			IDEController::write(
				channel,
				ATAReg::CONTROL,
				0x80 | channel.n_ien
			);
		}
		if reg < 0x08 {
			outb(channel.base + reg as u16, data);
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
			IDEController::write(
				channel,
				ATAReg::CONTROL,
				0x80 | channel.n_ien
			);
		}
		if reg < 0x08 {
			insl(channel.base + reg as u16, buffer.as_mut_ptr(), quads);
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
		while (IDEController::read(channel, ATAReg::STATUS) & ATAStatus::BSY)
			!= 0
		{
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

	fn print_error(&mut self, drive: u8, mut err: u8) -> u8 {
		let device: &IDEDevice = &self.devices[drive as usize];

		if err == 0 {
			return err;
		}
		kprintln!("IDE:");
		let binding = match &device.channel {
			Some(x) => x,
			None => {
				kprintln!("- Channel non-initialized");
				return 23;
			}
		};
		let channel: &IDEChannelRegisters = &binding.lock();
		match err {
			1 => {
				kprintln!("- Device Fault");
				err = 19;
			},
			2 => {
				let st: u8 = IDEController::read(channel, ATAReg::ERROR);
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
		&mut self,
		drive: u8,
		numsects: u8,
		lba: u32,
		edi: u32
	) -> Result<(), u8> {
		let device: &mut IDEDevice = &mut self.devices[drive as usize];

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
		} else if device.r#type == IDEType::ATA as u16 {
			match ATA::access(
				ATADirection::Read as u8,
				device,
				lba,
				numsects,
				edi
			) {
				Ok(_) => {},
				Err(err) => return Err(self.print_error(drive, err))
			}
		} else if device.r#type == IDEType::ATAPI as u16 {
			for i in 0..numsects {
				match ATAPI::read(
					device,
					lba + i as u32,
					1,
					edi + i as u32 * 2048
				) {
					Ok(_) => {},
					Err(err) => return Err(self.print_error(drive, err))
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
		&mut self,
		drive: u8,
		numsects: u8,
		lba: u32,
		edi: u32
	) -> Result<(), u8> {
		let device: &mut IDEDevice = &mut self.devices[drive as usize];

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
		} else if device.r#type == IDEType::ATA as u16 {
			match ATA::access(
				ATADirection::Write as u8,
				device,
				lba,
				numsects,
				edi
			) {
				Ok(_) => {},
				Err(err) => return Err(self.print_error(drive, err))
			}
		} else if device.r#type == IDEType::ATAPI as u16 {
			// Write-Protected
			return Err(0x4);
		}
		Ok(())
	}
}

#[cfg(test)]
mod test {

	use crate::alloc::vec;
	use crate::{sys_macros, IDE};

	#[sys_macros::test_case]
	fn ide_read_write_sector() {
		let to_write = vec![b'B'; 512];
		let read_from = vec![0x0_u8; 512];

		let _ = IDE
			.lock()
			.write_sectors(1, 1, 0x0, to_write.as_ptr() as u32);
		let _ = IDE
			.lock()
			.read_sectors(1, 1, 0x0, read_from.as_ptr() as u32);

		assert_eq!(to_write, read_from);
	}

	#[sys_macros::test_case]
	fn ide_read_write_multiple_sectors() {
		let to_write = vec![b'A'; 1024];
		let read_from = vec![0x0_u8; 1024];

		let _ = IDE
			.lock()
			.write_sectors(1, 2, 0x0, to_write.as_ptr() as u32);
		let _ = IDE
			.lock()
			.read_sectors(1, 2, 0x0, read_from.as_ptr() as u32);

		assert_eq!(to_write, read_from);
	}
}
