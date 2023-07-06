use super::{IDE_DEVICES, CHANNELS, IDE_IRQ_INVOKED, IDE};

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

pub enum ATAChannel {
	Primary   = 0x00,
	Secondary = 0x01
}

pub enum ATADirection {
	Read  = 0x00,
	Write = 0x01
}

struct ATA {}

impl ATA {
	unsafe fn access(direction: u8, drive: u8, lba: u32, numsects: u8, selector: u16, edi: u32) -> u8 {
		let lba_mode: u8; // 0: CHS, 1: LBA28, 2: LBA48
		let dma: u8; // 0: No DMA, 1: DMA
		let mut lba_io: [u8; 6] = [0; 6];
		let channel: u32 = IDE_DEVICES[drive as usize].channel as u32; // Read the channel
		let slavebit: u32 = IDE_DEVICES[drive as usize].drive as u32; // Read the Drive [Master/Slave]
		let bus: u32 = CHANNELS[channel as usize].base as u32; // Bus Base, like 0x1f0 which is also data port
		let words: u32 = 256; // Almost every ATA drive has sector-size of 512-byte
		let head: u8;
		let err: u8;

		// Disable IRQ
		IDE_IRQ_INVOKED = 0x0;
		CHANNELS[channel as usize].n_ien = IDE_IRQ_INVOKED + 0x02;
		IDE::write(channel as u8, ATAReg::CONTROL, CHANNELS[channel as usize].n_ien);

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
		} else if IDE_DEVICES[drive as usize].capabilities & 0x200 != 0 {
			// LBA48
			lba_mode = 1;
			lba_io[0] = ((lba & 0x00000FF) >> 0) as u8;
			lba_io[1] = ((lba & 0x000FF00) >> 8) as u8;
			lba_io[2] = ((lba & 0x0FF0000) >> 16) as u8;
			lba_io[3] = 0; // These Registers are not used here
			lba_io[4] = 0; // These Registers are not used here
			lba_io[5] = 0; // These Registers are not used here
			head  = ((lba & 0xF000000) >> 24) as u8;
		} else {
			// CHS:
			lba_mode = 0;
			let sect: u8 = ((lba % 63) + 1) as u8;
			let cyl: u16 = ((lba + 1  - sect as u32) / (16 * 63)) as u16;
			lba_io[0] = sect;
			lba_io[1] = ((cyl >> 0) & 0xFF) as u8;
			lba_io[2] = ((cyl >> 8) & 0xFF) as u8;
			lba_io[3] = 0;
			lba_io[4] = 0;
			lba_io[5] = 0;
			// Head number is written to HDDEVSEL lower 4-bits
			head = ((lba + 1  - sect as u32) % (16 * 63) / (63)) as u8;
		}

		// (II) See if drive supports DMA or not
		dma = 0; // We don't support DMA

		// (III) Wait if the drive is busy
		while (IDE::read(channel as u8, ATAReg::STATUS) & ATAStatus::BSY) != 0 {
		}

		// (IV) Select Drive from the controller
		if lba_mode == 0 { // Drive & CHS
			IDE::write(channel as u8, ATAReg::HDDEVSEL, 0xa0 | ((slavebit as u8) << 4) | head);
		} else { // Drive & LBA
			IDE::write(channel as u8, ATAReg::HDDEVSEL, 0xe0 | ((slavebit as u8) << 4) | head);
		}

		// (V) Write Parameters
		if lba_mode == 2 {
			IDE::write(channel as u8, ATAReg::SECCOUNT1, 0);
			IDE::write(channel as u8, ATAReg::LBA3, lba_io[3]);
			IDE::write(channel as u8, ATAReg::LBA4, lba_io[4]);
			IDE::write(channel as u8, ATAReg::LBA5, lba_io[5]);
		}
		IDE::write(channel as u8, ATAReg::SECCOUNT0, numsects);
		IDE::write(channel as u8, ATAReg::LBA0, lba_io[0]);
		IDE::write(channel as u8, ATAReg::LBA1, lba_io[1]);
		IDE::write(channel as u8, ATAReg::LBA2, lba_io[2]);

		0
	}
}
