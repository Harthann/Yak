mod ata;
mod atapi;

enum IDEType {
	ATA   = 0x00,
	ATAPI = 0x01
}

struct IDEChannelRegisters {
	base: u16, // I/O Base
	ctrl: u16, // ControlBase
	bmide: u16, // Bus Master IDE
	n_ien: u8 // nIEN (No Interrupt)
}
