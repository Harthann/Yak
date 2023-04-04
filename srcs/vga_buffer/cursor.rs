use crate::x86_64::io;
use crate::vga_buffer::{ColorCode, BUFFER_WIDTH};

#[derive(Debug, Clone, Copy)]
pub struct Cursor {
	x:          usize,
	y:          usize,
	color_code: ColorCode
}

impl Cursor {
	pub const fn new(x: usize, y: usize, color_code: ColorCode) -> Cursor {
		Cursor { x: x, y: y, color_code: color_code }
	}

	pub fn update(&self) {
		let pos: u16 = (self.y * BUFFER_WIDTH + self.x) as u16;

		io::outb(0x3d4, 0x0f);
		io::outb(0x3d5, (pos & 0xff) as u8);
		io::outb(0x3d4, 0x0e);
		io::outb(0x3d5, ((pos >> 8) & 0xff) as u8);
	}

	pub fn enable(&self) {
		const CURSOR_START: u8 = 14;
		const CURSOR_END: u8 = 15;

		io::outb(0x3d4, 0x0a);
		io::outb(0x3d5, (io::inb(0x3d5) & 0xc0) | CURSOR_START);
		io::outb(0x3d4, 0x0b);
		io::outb(0x3d5, (io::inb(0x3d5) & 0xe0) | CURSOR_END);
	}

	pub fn disable(&self) {
		io::outb(0x3d4, 0x0a);
		io::outb(0x3d5, 0x20);
	}

	pub fn get_color_code(&self) -> ColorCode {
		self.color_code
	}

	pub fn set_color_code(&mut self, color_code: ColorCode) {
		self.color_code = color_code;
	}

	pub fn get_pos(&self) -> (usize, usize) {
		(self.x, self.y)
	}

	pub fn set_pos(&mut self, x: usize, y: usize) {
		self.x = x;
		self.y = y;
	}
}
