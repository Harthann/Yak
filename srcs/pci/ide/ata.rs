use super::{IDEChannelRegisters, IDEDevice, IDE_IRQ_INVOKED};

use crate::io;

pub const SECTOR_SIZE: u32 = 512;

#[allow(non_snake_case)]
pub mod ATAStatus {
	pub const BSY: u8 = 0x80; // Busy
	pub const DRDY: u8 = 0x40; // Drive ready
	pub const DF: u8 = 0x20; // Drive write fault
	pub const DSC: u8 = 0x10; // Drive seek complete
	pub const DRQ: u8 = 0x08; // Data request ready
	pub const CORR: u8 = 0x04; // Corrected data
	pub const IDX: u8 = 0x02; // Index
	pub const ERR: u8 = 0x01; // Error
}

#[allow(non_snake_case)]
pub mod ATAError {
	pub const BBK: u8 = 0x80; // Bad block
	pub const UNC: u8 = 0x40; // Uncorrectable data
	pub const MC: u8 = 0x20; // Media changed
	pub const IDNF: u8 = 0x10; // ID mark not found
	pub const MCR: u8 = 0x08; // Media change request
	pub const ABRT: u8 = 0x04; // Command aborted
	pub const TK0NF: u8 = 0x02; // Track 0 not found
	pub const AMNF: u8 = 0x01; // No address mark
}

#[derive(Clone, Copy)]
#[allow(non_snake_case)]
pub enum ATACommand {
	ReadPio        = 0x20,
	ReadPioExt     = 0x24,
	ReadDma        = 0xc8,
	ReadDmaExt     = 0x25,
	WritePio       = 0x30,
	WritePioExt    = 0x34,
	WriteDma       = 0xca,
	WriteDmaExt    = 0x35,
	CacheFlush     = 0xe7,
	CacheFlushExt  = 0xea,
	Packet         = 0xa0,
	IdentifyPacket = 0xa1,
	Identify       = 0xec
}

#[allow(non_snake_case)]
pub mod ATAIdentify {
	pub const DEVICETYPE: usize = 0;
	pub const CYLINDERS: usize = 2;
	pub const HEADS: usize = 6;
	pub const SECTORS: usize = 12;
	pub const SERIAL: usize = 20;
	pub const MODEL: usize = 54;
	pub const CAPABILITIES: usize = 98;
	pub const FIELDVALID: usize = 106;
	pub const MAX_LBA: usize = 120;
	pub const COMMANDSETS: usize = 164;
	pub const MAX_LBA_EXT: usize = 200;
}

pub enum ATAType {
	MASTER = 0x00,
	SLAVE  = 0x01
}

#[allow(non_snake_case)]
pub mod ATAReg {
	pub const DATA: u8 = 0x00;
	pub const ERROR: u8 = 0x01;
	pub const FEATURES: u8 = 0x01;
	pub const SECCOUNT0: u8 = 0x02;
	pub const LBA0: u8 = 0x03;
	pub const LBA1: u8 = 0x04;
	pub const LBA2: u8 = 0x05;
	pub const HDDEVSEL: u8 = 0x06;
	pub const COMMAND: u8 = 0x07;
	pub const STATUS: u8 = 0x07;
	pub const SECCOUNT1: u8 = 0x08;
	pub const LBA3: u8 = 0x09;
	pub const LBA4: u8 = 0x0a;
	pub const LBA5: u8 = 0x0b;
	pub const CONTROL: u8 = 0x0c;
	pub const ALTSTATUS: u8 = 0x0c;
	pub const DEVADDRESS: u8 = 0x0d;
}

#[derive(Clone, Copy)]
pub enum ATAChannel {
	Primary   = 0x00,
	Secondary = 0x01
}

pub enum ATADirection {
	Read  = 0x00,
	Write = 0x01
}

pub struct ATA {}

impl ATA {
	pub fn access(
		direction: u8,
		device: &IDEDevice,
		lba: u32,
		numsects: u8,
		mut edi: u32
	) -> Result<(), u8> {
		let binding = match &device.channel {
			Some(x) => x,
			None => return Err(0x1)
		};
		let bind = binding.lock();
		let mut channel = bind.borrow_mut();
		let lba_mode: u8; // 0: CHS, 1: LBA28, 2: LBA48
		let dma: u8; // 0: No DMA, 1: DMA
		let mut lba_io: [u8; 6] = [0; 6];
		// Read the Drive [Master/Slave]
		let slavebit: u32 = device.drive as u32;
		// Bus Base, like 0x1f0 which is also data port
		let bus: u32 = channel.base as u32;
		// Almost every ATA drive has sector-size of 512-byte
		let words: u32 = SECTOR_SIZE / 2;
		let head: u8;

		// Disable IRQ
		*IDE_IRQ_INVOKED.lock() = 0x0;
		channel.n_ien = 0x02;
		channel.write(ATAReg::CONTROL, 0x02);

		// (I) Select one from LBA28, LBA48 or CHS
		// Sure Drive should support LBA in this case or you
		// are giving a wrong LBA
		if lba >= 0x10000000 {
			// LBA48
			lba_mode = 2;
			lba_io[0] = ((lba & 0x000000FF) >> 0) as u8;
			lba_io[1] = ((lba & 0x0000FF00) >> 8) as u8;
			lba_io[2] = ((lba & 0x00FF0000) >> 16) as u8;
			lba_io[3] = ((lba & 0xFF000000) >> 24) as u8;
			lba_io[4] = 0; // LBA28 is integer, so 32-bits are enough to access 2TB
			lba_io[5] = 0; // LBA28 is integer, so 32-bits are enough to access 2TB
			head = 0; // Lower 4-bits of HDDEVSEL are not used here
		} else if device.capabilities & 0x200 != 0 {
			// LBA48
			lba_mode = 1;
			lba_io[0] = ((lba & 0x00000FF) >> 0) as u8;
			lba_io[1] = ((lba & 0x000FF00) >> 8) as u8;
			lba_io[2] = ((lba & 0x0FF0000) >> 16) as u8;
			lba_io[3] = 0; // These Registers are not used here
			lba_io[4] = 0; // These Registers are not used here
			lba_io[5] = 0; // These Registers are not used here
			head = ((lba & 0xF000000) >> 24) as u8;
		} else {
			// CHS:
			lba_mode = 0;
			let sect: u8 = ((lba % 63) + 1) as u8;
			let cyl: u16 = ((lba + 1 - sect as u32) / (16 * 63)) as u16;
			lba_io[0] = sect;
			lba_io[1] = ((cyl >> 0) & 0xFF) as u8;
			lba_io[2] = ((cyl >> 8) & 0xFF) as u8;
			lba_io[3] = 0;
			lba_io[4] = 0;
			lba_io[5] = 0;
			// Head number is written to HDDEVSEL lower 4-bits
			head = ((lba + 1 - sect as u32) % (16 * 63) / (63)) as u8;
		}

		// (II) See if drive supports DMA or not
		dma = 0; // We don't support DMA

		// (III) Wait if the drive is busy
		while (channel.read(ATAReg::STATUS) & ATAStatus::BSY) != 0 {}

		// (IV) Select Drive from the controller
		if lba_mode == 0 {
			// Drive & CHS
			channel
				.write(ATAReg::HDDEVSEL, 0xa0 | ((slavebit as u8) << 4) | head);
		} else {
			// Drive & LBA
			channel
				.write(ATAReg::HDDEVSEL, 0xe0 | ((slavebit as u8) << 4) | head);
		}

		// (V) Write Parameters
		if lba_mode == 2 {
			channel.write(ATAReg::SECCOUNT1, 0);
			channel.write(ATAReg::LBA3, lba_io[3]);
			channel.write(ATAReg::LBA4, lba_io[4]);
			channel.write(ATAReg::LBA5, lba_io[5]);
		}
		channel.write(ATAReg::SECCOUNT0, numsects);
		channel.write(ATAReg::LBA0, lba_io[0]);
		channel.write(ATAReg::LBA1, lba_io[1]);
		channel.write(ATAReg::LBA2, lba_io[2]);

		// (VI) Select the command and send it
		// Routine that is followed:
		// If ( DMA & LBA48)   DO_DMA_EXT
		// If ( DMA & LBA28)   DO_DMA_LBA
		// If ( DMA & LBA28)   DO_DMA_CHS
		// If (!DMA & LBA48)   DO_PIO_EXT
		// If (!DMA & LBA28)   DO_PIO_LBA
		// If (!DMA & !LBA#)   DO_PIO_CHS

		let cmd = match (lba_mode, dma, direction) {
			(0, 0, 0) => ATACommand::ReadPio,
			(1, 0, 0) => ATACommand::ReadPio,
			(2, 0, 0) => ATACommand::ReadPioExt,
			(0, 1, 0) => ATACommand::ReadDma,
			(1, 1, 0) => ATACommand::ReadDma,
			(2, 1, 0) => ATACommand::ReadDmaExt,
			(0, 0, 1) => ATACommand::WritePio,
			(1, 0, 1) => ATACommand::WritePio,
			(2, 0, 1) => ATACommand::WritePioExt,
			(0, 1, 1) => ATACommand::WriteDma,
			(1, 1, 1) => ATACommand::WriteDma,
			(2, 1, 1) => ATACommand::WriteDmaExt,
			_ => todo!()
		};
		// Send the command
		channel.write(ATAReg::COMMAND, cmd as u8);

		if dma != 0 {
			if direction == 0 {
				// DMA Read
			} else {
				// DMA Write
			}
		} else {
			if direction == 0 {
				// PIO Read
				for _ in 0..numsects {
					// Polling, set error and exit if there is
					channel.polling(1)?;
					io::insw(bus as u16, edi as *mut _, words);
					edi += words * 2;
				}
			} else {
				// PIO Write
				for _ in 0..numsects {
					// Polling
					channel.polling(0)?;
					io::outsw(bus as u16, edi as *mut _, words);
					edi += words * 2;
				}
				channel.write(
					ATAReg::COMMAND,
					[
						ATACommand::CacheFlush,
						ATACommand::CacheFlush,
						ATACommand::CacheFlushExt
					][lba_mode as usize] as u8
				);
				// Polling
				channel.polling(0)?;
			}
		}
		Ok(())
	}
}
