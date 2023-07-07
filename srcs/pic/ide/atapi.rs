use core::arch::asm;

use super::ata::{ATACommand, ATAReg, ATAStatus};
use super::{IDEType, CHANNELS, IDE, IDE_DEVICES, IDE_IRQ_INVOKED};

static mut ATAPI_PACKET: [u8; 12] = [0xa8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

enum ATAPICommand {
	Read  = 0xa8,
	Eject = 0x1b
}

pub struct ATAPI {}

impl ATAPI {
	pub unsafe fn read(
		drive: u8,
		lba: u32,
		numsects: u8,
		mut edi: u32
	) -> u8 {
		let channel: u32 = IDE_DEVICES[drive as usize].channel as u32;
		let slavebit: u32 = IDE_DEVICES[drive as usize].drive as u32;
		let bus: u32 = CHANNELS[channel as usize].base as u32;
		// Sector Size
		// ATAPI drives have a sector size of 2048 bytes
		let words: u32 = 1024;

		// Enable IRQs
		IDE_IRQ_INVOKED = 0;
		CHANNELS[channel as usize].n_ien = 0;
		IDE::write(
			channel as u8,
			ATAReg::CONTROL,
			CHANNELS[channel as usize].n_ien
		);

		// (I) Setup SCSI Packet
		ATAPI_PACKET[0] = ATAPICommand::Read as u8;
		ATAPI_PACKET[1] = 0x0;
		ATAPI_PACKET[2] = ((lba >> 24) & 0xff) as u8;
		ATAPI_PACKET[3] = ((lba >> 16) & 0xff) as u8;
		ATAPI_PACKET[4] = ((lba >> 8) & 0xff) as u8;
		ATAPI_PACKET[5] = ((lba >> 0) & 0xff) as u8;
		ATAPI_PACKET[6] = 0x0;
		ATAPI_PACKET[7] = 0x0;
		ATAPI_PACKET[8] = 0x0;
		ATAPI_PACKET[9] = numsects;
		ATAPI_PACKET[10] = 0x0;
		ATAPI_PACKET[11] = 0x0;

		// (II) Select the drive
		IDE::write(channel as u8, ATAReg::HDDEVSEL, (slavebit << 4) as u8);

		// (III) Delay 400 nanoseconds for select to complete
		for _ in 0..4 {
			// Reading the Alternate Status port wastes 100ns
			IDE::read(channel as u8, ATAReg::ALTSTATUS);
		}

		// (IV) Inform the Controller that we use PIO mode
		IDE::write(channel as u8, ATAReg::FEATURES, 0);

		// (V) Tell the Controller the size of buffer
		// Lower Byte of Sector size
		IDE::write(channel as u8, ATAReg::LBA1, ((words * 2) & 0xff) as u8);
		// Upper Byte of Sector size
		IDE::write(channel as u8, ATAReg::LBA2, ((words * 2) >> 8) as u8);

		// (VI) Send the Packet Command
		IDE::write(channel as u8, ATAReg::COMMAND, ATACommand::Packet as u8);

		// (VII) Waiting for the driver to finish or return an error code
		let err: u8 = IDE::polling(channel as u8, 1);
		if err != 0 {
			return err;
		}

		// (VIII) Sending the packet data
		asm!(
			"push esi",
			"mov esi, {esi}",
			"rep outsw", // Send Packet Data
			"pop esi",
			in("ecx") 6,
			in("edx") bus,
			esi = in(reg) ATAPI_PACKET.as_ptr()
		);

		// (IX) Receiving Data
		for _ in 0..numsects {
			IDE::wait_irq();
			let err: u8 = IDE::polling(channel as u8, 1);
			if err != 0 {
				return err;
			}
			asm!(
				"rep insw",
				in("ecx") words,
				in("edx") bus,
				in("edi") edi
			);
			edi += words * 2;
		}

		// (X) Waiting for an IRQ
		IDE::wait_irq();

		// (XI) Waiting for BSY & DRQ to clear
		loop {
			if (IDE::read(channel as u8, ATAReg::STATUS)
				& (ATAStatus::BSY | ATAStatus::DRQ))
				== 0
			{
				break;
			}
		}

		0
	}

	pub unsafe fn eject(drive: u8) -> u8 {
		let channel: u32 = IDE_DEVICES[drive as usize].channel as u32;
		let slavebit: u32 = IDE_DEVICES[drive as usize].drive as u32;
		let bus: u32 = CHANNELS[channel as usize].base as u32;
		// Sector size in words
		// 		let words: u32 = 2048 / 2;
		let mut err: u8;

		// 1- Check if the drive presents
		if drive > 3 || IDE_DEVICES[drive as usize].reserved == 0 {
			err = 0x1;
		// 2- Check if drive isn't ATAPI
		} else if IDE_DEVICES[drive as usize].r#type == IDEType::ATA as u16 {
			err = 20;
		// 3- Eject ATAPI Driver
		} else {
			// Enable IRQs
			IDE_IRQ_INVOKED = 0x0;
			CHANNELS[channel as usize].n_ien = 0x0;
			IDE::write(
				channel as u8,
				ATAReg::CONTROL,
				CHANNELS[channel as usize].n_ien
			);

			// (I) Setup SCSI Packet
			ATAPI_PACKET[0] = 0x0;
			ATAPI_PACKET[1] = 0x0;
			ATAPI_PACKET[2] = 0x0;
			ATAPI_PACKET[3] = 0x2;
			ATAPI_PACKET[4] = 0x0;
			ATAPI_PACKET[5] = 0x0;
			ATAPI_PACKET[6] = 0x0;
			ATAPI_PACKET[7] = 0x0;
			ATAPI_PACKET[8] = 0x0;
			ATAPI_PACKET[9] = 0x0;
			ATAPI_PACKET[10] = 0x0;
			ATAPI_PACKET[11] = 0x0;

			// (II) Select the Drive
			IDE::write(channel as u8, ATAReg::HDDEVSEL, (slavebit << 4) as u8);

			// (III) Delay 400 nanosecond for select to complete
			for _ in 0..4 {
				// Reading Alternate Status Port Wastes 100ns
				IDE::read(channel as u8, ATAReg::ALTSTATUS);
			}

			// (IV) Send the Packet Command
			IDE::write(
				channel as u8,
				ATAReg::COMMAND,
				ATACommand::Packet as u8
			);

			// (V) Waiting for the driver to finish or invoke an error
			// Polling and stop if error
			err = IDE::polling(channel as u8, 1);
			if err != 0 {
				return err;
			}

			// (VI) Sending the packet data
			asm!(
				"push esi",
				"mov esi, {esi}",
				"rep outsw",
				"pop esi",
				in("ecx") 6,
				in("edx") bus,
				esi = in(reg) ATAPI_PACKET.as_ptr()
			);
			IDE::wait_irq();
			// Polling and get error code
			err = IDE::polling(channel as u8, 1);
			if err == 3 {
				// DRQ is not needed here
				err = 0;
			}
			err = IDE::print_error(drive as u32, err);
		}
		err
	}
}
