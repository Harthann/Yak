use crate::io::{inb, insl, outb};
use super::ata::ATAChannel;
use super::ATAStatus;
use super::ATAReg;

#[derive(Clone, Copy)]
pub struct IDEChannelRegisters {
	pub r#type: ATAChannel, // 0 - Primary Channel, 1 - Secondary Channel
	pub base:   u16,        // I/O Base
	ctrl:   u16,        // ControlBase
	bmide:  u16,        // Bus Master IDE
	pub n_ien:  u8          // nIEN (No Interrupt)
}

impl IDEChannelRegisters {
	pub const fn new(channel: ATAChannel, base: u16, ctrl: u16, bmide: u16, n_ien: u8) -> Self {
		Self { r#type: channel, base, ctrl, bmide, n_ien}
	}

    pub fn read(&mut self, reg: u8) -> u8 {
		let mut result: u8 = 0;
		if reg > 0x07 && reg < 0x0c {
			self.write(ATAReg::CONTROL, 0x80 | self.n_ien);
		}
		if reg < 0x08 {
			result = inb(self.base + reg as u16 - 0x00);
		} else if reg < 0x0c {
			result = inb(self.base + reg as u16 - 0x06);
		} else if reg < 0x0e {
			result = inb(self.ctrl + reg as u16 - 0x0a);
		} else if reg < 0x16 {
			result = inb(self.bmide + reg as u16 - 0x0e);
		}
		if reg > 0x07 && reg < 0x0c {
			self.write(ATAReg::CONTROL, self.n_ien);
		}
		return result;
	}

    pub fn read_buffer(
        &mut self,
		reg: u8,
		buffer: &mut [u32],
		quads: u32
	) {
		if reg > 0x07 && reg < 0x0c {
			self.write(
				ATAReg::CONTROL,
				0x80 | self.n_ien
			);
		}
		if reg < 0x08 {
			insl(self.base + reg as u16 - 0x00, buffer.as_mut_ptr(), quads);
		} else if reg < 0x0c {
			insl(self.base + reg as u16 - 0x06, buffer.as_mut_ptr(), quads);
		} else if reg < 0x0e {
			insl(self.ctrl + reg as u16 - 0x0a, buffer.as_mut_ptr(), quads);
		} else if reg < 0x16 {
			insl(self.bmide + reg as u16 - 0x0e, buffer.as_mut_ptr(), quads);
		}
		if reg > 0x07 && reg < 0x0c {
			self.write(ATAReg::CONTROL, self.n_ien);
		}
	}


    pub fn write(&mut self, reg: u8, data: u8) {
        if reg > 0x07 && reg < 0x0c {
			self.write(ATAReg::CONTROL, 0x80 | self.n_ien);
		}
		if reg < 0x08 {
			outb(self.base + reg as u16 - 0x00, data);
		} else if reg < 0x0c {
			outb(self.base + reg as u16 - 0x06, data);
		} else if reg < 0x0e {
			outb(self.ctrl + reg as u16 - 0x0a, data);
		} else if reg < 0x16 {
			outb(self.bmide + reg as u16 - 0x0e, data);
		}
		if reg > 0x07 && reg < 0x0c {
			self.write(ATAReg::CONTROL, self.n_ien);
		}
    }


    pub fn polling(&mut self, advanced_check: u32) -> Result<(), u8> {
		for _ in 0..4 {
			self.read(ATAReg::ALTSTATUS);
		}

		// (II) Wait for BSY to be cleared
		while (self.read(ATAReg::STATUS) & ATAStatus::BSY as u8) != 0
		{ /* Wait for BSY to be zero */ }

		if advanced_check != 0 {
			// Read Status Register
			let state: u8 = self.read(ATAReg::STATUS);

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

}
