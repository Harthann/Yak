use super::ata::{ATACommand, ATAReg, ATAStatus};
use super::{
	IDEChannelRegisters,
	IDEType,
	CHANNELS,
	IDE,
	IDE_DEVICES,
	IDE_IRQ_INVOKED
};

use crate::io;

enum ATAPICommand {
	Capacity = 0x25,
	Read     = 0xa8,
	Eject    = 0x1b
}

pub struct ATAPI {}

impl ATAPI {
	pub unsafe fn capacity(drive: u8, lba: u32) -> Result<u32, u8> {
		let channel: &mut IDEChannelRegisters =
			&mut CHANNELS[IDE_DEVICES[drive as usize].channel as usize];
		let slavebit: u32 = IDE_DEVICES[drive as usize].drive as u32;
		let bus: u32 = channel.base as u32;
		let mut buffer: [u32; 2] = [0; 2];

		// Enable IRQs
		IDE_IRQ_INVOKED = 0;
		channel.n_ien = 0;
		IDE::write(channel, ATAReg::CONTROL, channel.n_ien);

		// (I) Setup SCSI Packet
		let packet: [u8; 12] = [
			ATAPICommand::Capacity as u8,
			0x0,
			((lba >> 24) & 0xff) as u8,
			((lba >> 16) & 0xff) as u8,
			((lba >> 8) & 0xff) as u8,
			((lba >> 0) & 0xff) as u8,
			0x0,
			0x0,
			0x0,
			0x0,
			0x0,
			0x0
		];

		// (II) Select the drive
		IDE::write(channel, ATAReg::HDDEVSEL, (slavebit << 4) as u8);

		// (III) Delay 400 nanoseconds for select to complete
		for _ in 0..4 {
			// Reading the Alternate Status port wastes 100ns
			IDE::read(channel, ATAReg::ALTSTATUS);
		}

		// (IV) Inform the Controller that we use PIO mode
		IDE::write(channel, ATAReg::FEATURES, 0);

		// (V) Tell the Controller the size of buffer (16 bytes will be returned)
		let size: usize = 0x0008;
		// Lower Byte of Sector size
		IDE::write(channel, ATAReg::LBA1, (size & 0xff) as u8);
		// Upper Byte of Sector size
		IDE::write(channel, ATAReg::LBA2, (size >> 8) as u8);

		// (VI) Send the Packet Command
		IDE::write(channel, ATAReg::COMMAND, ATACommand::Packet as u8);

		// (VII) Waiting for the driver to finish or return an error code
		IDE::polling(channel, 1)?;

		// (VIII) Sending the packet data
		io::outsw(bus as u16, packet.align_to::<u16>().1.as_ptr(), 6);

		// (IX) Receiving Data
		IDE::polling(channel, 1)?;
		io::insw(bus as u16, buffer.align_to_mut::<u16>().1.as_mut_ptr(), 4);

		// (X) Waiting for BSY & DRQ to clear
		loop {
			if (IDE::read(channel, ATAReg::STATUS)
				& (ATAStatus::BSY | ATAStatus::DRQ))
				== 0
			{
				break;
			}
		}

		// (((Last LBA + 1) * Block size) / 1024) * 2
		Ok((((buffer[0].to_be() + 1) * buffer[1].to_be()) / 1024) * 2)
	}

	pub unsafe fn read(
		drive: u8,
		lba: u32,
		numsects: u8,
		mut edi: u32
	) -> Result<(), u8> {
		let channel: &mut IDEChannelRegisters =
			&mut CHANNELS[IDE_DEVICES[drive as usize].channel as usize];
		let slavebit: u32 = IDE_DEVICES[drive as usize].drive as u32;
		let bus: u32 = channel.base as u32;
		// Sector Size
		// ATAPI drives have a sector size of 2048 bytes
		let words: u32 = 1024;

		// Enable IRQs
		IDE_IRQ_INVOKED = 0;
		channel.n_ien = 0;
		IDE::write(channel, ATAReg::CONTROL, channel.n_ien);

		// (I) Setup SCSI Packet
		let packet: [u8; 12] = [
			ATAPICommand::Read as u8,
			0x0,
			((lba >> 24) & 0xff) as u8,
			((lba >> 16) & 0xff) as u8,
			((lba >> 8) & 0xff) as u8,
			((lba >> 0) & 0xff) as u8,
			0x0,
			0x0,
			0x0,
			numsects,
			0x0,
			0x0
		];

		// (II) Select the drive
		IDE::write(channel, ATAReg::HDDEVSEL, (slavebit << 4) as u8);

		// (III) Delay 400 nanoseconds for select to complete
		for _ in 0..4 {
			// Reading the Alternate Status port wastes 100ns
			IDE::read(channel, ATAReg::ALTSTATUS);
		}

		// (IV) Inform the Controller that we use PIO mode
		IDE::write(channel, ATAReg::FEATURES, 0);

		// (V) Tell the Controller the size of buffer
		// Lower Byte of Sector size
		IDE::write(channel, ATAReg::LBA1, ((words * 2) & 0xff) as u8);
		// Upper Byte of Sector size
		IDE::write(channel, ATAReg::LBA2, ((words * 2) >> 8) as u8);

		// (VI) Send the Packet Command
		IDE::write(channel, ATAReg::COMMAND, ATACommand::Packet as u8);

		// (VII) Waiting for the driver to finish or return an error code
		IDE::polling(channel, 1)?;

		// (VIII) Sending the packet data
		io::outsw(bus as u16, packet.align_to::<u16>().1.as_ptr(), 6);

		// (IX) Receiving Data
		for _ in 0..numsects {
			IDE::wait_irq();
			IDE::polling(channel, 1)?;
			io::insw(bus as u16, edi as *mut _, words);
			edi += words * 2;
		}

		// (X) Waiting for an IRQ
		IDE::wait_irq();

		// (XI) Waiting for BSY & DRQ to clear
		loop {
			if (IDE::read(channel, ATAReg::STATUS)
				& (ATAStatus::BSY | ATAStatus::DRQ))
				== 0
			{
				break;
			}
		}

		Ok(())
	}

	pub unsafe fn eject(drive: u8) -> Result<(), u8> {
		let channel: &mut IDEChannelRegisters =
			&mut CHANNELS[IDE_DEVICES[drive as usize].channel as usize];
		let slavebit: u32 = IDE_DEVICES[drive as usize].drive as u32;
		let bus: u32 = channel.base as u32;

		// 1- Check if the drive presents
		if drive > 3 || IDE_DEVICES[drive as usize].reserved == 0 {
			return Err(0x1);
		// 2- Check if drive isn't ATAPI
		} else if IDE_DEVICES[drive as usize].r#type == IDEType::ATA as u16 {
			return Err(20);
		// 3- Eject ATAPI Driver
		} else {
			// Enable IRQs
			IDE_IRQ_INVOKED = 0x0;
			channel.n_ien = 0x0;
			IDE::write(channel, ATAReg::CONTROL, channel.n_ien);

			// (I) Setup SCSI Packet
			let packet: [u8; 12] = [
				ATAPICommand::Eject as u8,
				0x0,
				0x0,
				0x2,
				0x0,
				0x0,
				0x0,
				0x0,
				0x0,
				0x0,
				0x0,
				0x0
			];

			// (II) Select the Drive
			IDE::write(channel, ATAReg::HDDEVSEL, (slavebit << 4) as u8);

			// (III) Delay 400 nanosecond for select to complete
			for _ in 0..4 {
				// Reading Alternate Status Port Wastes 100ns
				IDE::read(channel, ATAReg::ALTSTATUS);
			}

			// (IV) Send the Packet Command
			IDE::write(channel, ATAReg::COMMAND, ATACommand::Packet as u8);

			// (V) Waiting for the driver to finish or invoke an error
			// Polling and stop if error
			IDE::polling(channel, 1)?;

			// (VI) Sending the packet data
			io::outsw(bus as u16, packet.align_to::<u16>().1.as_ptr(), 6);

			IDE::wait_irq();
			// Polling and get error code
			match IDE::polling(channel, 1) {
				Err(err) if err != 3 => return Err(err),
				_ => {}
			}
		}
		Ok(())
	}
}
