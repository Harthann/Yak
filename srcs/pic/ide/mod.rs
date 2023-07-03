use crate::io::{inb, outb, insl};

mod ata;
mod atapi;

enum IDEType {
	ATA   = 0x00,
	ATAPI = 0x01
}

#[derive(Clone, Copy)]
struct IDEChannelRegisters {
	base:  u16, // I/O Base
	ctrl:  u16, // ControlBase
	bmide: u16, // Bus Master IDE
	n_ien: u8 // nIEN (No Interrupt)
}

static mut CHANNELS: [IDEChannelRegisters; 2] = [IDEChannelRegisters {
	base: 0,
	ctrl: 0,
	bmide: 0,
	n_ien: 0
}; 2];

static mut IDE_BUF: [u8; 2048] = [0; 2048];
static mut IDE_IRQ_INVOKED: u8 = 0;
static mut ATAPI_PACKET: [u8; 12] = [0xA8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

#[derive(Clone, Copy)]
struct IDEDevice {
	reserved:     u8, // 0 (Empty) or 1 (This Drive really exists)
	channel:      u8, // 0 (Primary Channel) or 1 (Secondary Channel)
	drive:        u8, // 0 (Master Drive) or 1 (Slave Drive)
	r#type:       u16, // 0: ATA, 1:ATAPI
	signature:    u16, // Drive Signature
	capabilities: u16, // Features
	command_sets: u32, // Command Sets Supported
	size:         u32, // Size in Sectors
	model:        [u8; 41] // Model in string
}

static mut IDE_DEVICES: [IDEDevice; 4] = [IDEDevice {
	reserved: 0,
	channel: 0,
	drive: 0,
	r#type: 0,
	signature: 0,
	capabilities: 0,
	command_sets: 0,
	size: 0,
	model: [0; 41]
}; 4];

struct IDE {}

impl IDE {
	unsafe fn read(channel: u8, reg: u8) -> u8 {
		let mut result: u8 = 0;
		if reg > 0x07 && reg < 0x0c {
			IDE::write(	
				channel,
				ata::ATARegOffset::CONTROL as u8,
				0x80 | CHANNELS[channel as usize].n_ien
			);
		}
		if reg < 0x08 {
			result = inb(CHANNELS[channel as usize].base + reg as u16 - 0x00);
		} else if reg < 0x0c {
			result = inb(CHANNELS[channel as usize].base + reg as u16 - 0x06);
		} else if reg < 0x0e {
			result = inb(CHANNELS[channel as usize].ctrl + reg as u16 - 0x0a);
		} else if reg < 0x16 {
			result = inb(CHANNELS[channel as usize].bmide + reg as u16 - 0x0e);
		}
		if reg > 0x07 && reg < 0x0c {
			IDE::write(
				channel,
				ata::ATARegOffset::CONTROL as u8,
				CHANNELS[channel as usize].n_ien
			);
		}
		return result;
	}

	unsafe fn write(channel: u8, reg: u8, data: u8) {
		if reg > 0x07 && reg < 0x0c {
			IDE::write(
				channel,
				ata::ATARegOffset::CONTROL as u8,
				0x80 | CHANNELS[channel as usize].n_ien
			);
		}
		if reg < 0x08 {
			outb(CHANNELS[channel as usize].base + reg as u16 - 0x00, data);
		} else if reg < 0x0c {
			outb(CHANNELS[channel as usize].base + reg as u16 - 0x06, data);
		} else if reg < 0x0e {
			outb(CHANNELS[channel as usize].ctrl + reg as u16 - 0x0a, data);
		} else if reg < 0x16 {
			outb(CHANNELS[channel as usize].bmide + reg as u16 - 0x0e, data);
		}
		if reg > 0x07 && reg < 0x0c {
			IDE::write(
				channel,
				ata::ATARegOffset::CONTROL as u8,
				CHANNELS[channel as usize].n_ien
			);
		}
	}

	unsafe fn read_buffer(channel: u8, reg: u8, buffer: &mut [u32], quads: u32) {
		if reg > 0x07 && reg < 0x0c {
			IDE::write(
				channel,
				ata::ATARegOffset::CONTROL as u8,
				0x80 | CHANNELS[channel as usize].n_ien
			);
		}
		if reg < 0x08 {
			insl(CHANNELS[channel as usize].base + reg as u16 - 0x00, buffer, quads);
		} else if reg < 0x0c {
			insl(CHANNELS[channel as usize].base + reg as u16 - 0x06, buffer, quads);
		} else if reg < 0x0e {
			insl(CHANNELS[channel as usize].ctrl + reg as u16 - 0x0a, buffer, quads);
		} else if reg < 0x16 {
			insl(CHANNELS[channel as usize].bmide + reg as u16 - 0x0e, buffer, quads);
		}
		if reg > 0x07 && reg < 0x0c {
			IDE::write(
				channel,
				ata::ATARegOffset::CONTROL as u8,
				CHANNELS[channel as usize].n_ien
			);
		}
	}
}
