#[allow(non_snake_case)]
pub mod ATAStatus {
	pub const BSY: u8  = 0x80; // Busy
	pub const DRDY: u8 = 0x40; // Drive ready
	pub const DF: u8   = 0x20; // Drive write fault
	pub const DSC: u8  = 0x10; // Drive seek complete
	pub const DRQ: u8  = 0x08; // Data request ready
	pub const CORR: u8 = 0x04; // Corrected data
	pub const IDX: u8  = 0x02; // Index
	pub const ERR: u8  = 0x01; // Error
}

#[allow(non_snake_case)]
pub mod ATAError {
	pub const BBK: u8   = 0x80; // Bad block
	pub const UNC: u8   = 0x40; // Uncorrectable data
	pub const MC: u8    = 0x20; // Media changed
	pub const IDNF: u8  = 0x10; // ID mark not found
	pub const MCR: u8   = 0x08; // Media change request
	pub const ABRT: u8  = 0x04; // Command aborted
	pub const TK0NF: u8 = 0x02; // Track 0 not found
	pub const AMNF: u8  = 0x01; // No address mark
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
	IdentifyPacket = 0xa1
}

#[allow(non_snake_case)]
pub mod ATACmdIdentifyOffset {
	pub const DEVICETYPE: usize   = 0;
	pub const CYLINDERS: usize    = 2;
	pub const HEADS: usize        = 6;
	pub const SECTORS: usize      = 12;
	pub const SERIAL: usize       = 20;
	pub const MODEL: usize        = 54;
	pub const CAPABILITIES: usize = 98;
	pub const FIELDVALID: usize   = 106;
	pub const MAX_LBA: usize      = 120;
	pub const COMMANDSETS:usize   = 164;
	pub const MAX_LBA_EXT: usize  = 200;
}

pub enum ATAType {
	MASTER = 0x00,
	SLAVE  = 0x01
}

#[allow(non_snake_case)]
pub mod ATARegOffset {
	pub const DATA: usize       = 0x00;
	pub const ERROR: usize      = 0x01;
	pub const FEATURES: usize   = 0x01;
	pub const SECCOUNT0: usize  = 0x02;
	pub const LBA0: usize       = 0x03;
	pub const LBA1: usize       = 0x04;
	pub const LBA2: usize       = 0x05;
	pub const HDDEVSEL: usize   = 0x06;
	pub const COMMAND: usize    = 0x07;
	pub const STATUS: usize     = 0x07;
	pub const SECCOUNT1: usize  = 0x08;
	pub const LBA3: usize       = 0x09;
	pub const LBA4: usize       = 0x0a;
	pub const LBA5: usize       = 0x0b;
	pub const CONTROL: usize    = 0x0c;
	pub const ALTSTATUS: usize  = 0x0c;
	pub const DEVADDRESS: usize = 0x0d;
}

pub enum ATAChannel {
	Primary    = 0x00,
	Secondary  = 0x01
}

pub enum ATADirection {
	Read  = 0x00,
	Write = 0x01
}
