use crate::io;
use crate::vga_buffer::{ColorCode, BUFFER_WIDTH};

#[derive(Debug, Clone, Copy)]
pub struct Cursor {
	pos:        usize,
	color_code: ColorCode
}

impl Cursor {
	pub const fn new(x: usize, y: usize, color_code: ColorCode) -> Cursor {
		Cursor { pos: y * BUFFER_WIDTH + x, color_code: color_code }
	}

	pub fn update(&self) {
		io::outb(0x3d4, 0x0f);
		io::outb(0x3d5, (self.pos & 0xff) as u8);
		io::outb(0x3d4, 0x0e);
		io::outb(0x3d5, ((self.pos >> 8) & 0xff) as u8);
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

	pub fn get_pos(&self) -> usize {
		self.pos
	}

	pub fn set_pos(&mut self, x: usize) {
		self.pos = x;
	}
}
