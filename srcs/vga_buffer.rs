/*  Crate import */
use core::fmt;
use core::panic::PanicInfo;
use crate::io;


#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
	Black		= 0,
	Blue		= 1,
	Green		= 2,
	Cyan		= 3,
	Red			= 4,
	Magenta		= 5,
	Brown		= 6,
	LightGray	= 7,
	DarkGray	= 8,
	LightBlue	= 9,
	LightGreen	= 10,
	LightCyan	= 11,
	LightRed	= 12,
	Pink		= 13,
	Yellow		= 14,
	White		= 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ColorCode(u8);
impl ColorCode {
	const fn new(foreground: Color, background: Color) -> ColorCode {
		ColorCode((background as u8) << 4 | (foreground as u8))
	}
}

#[derive(Debug, Clone, Copy)]
pub struct Cursor {
	x:	usize,
	y:	usize,
	color_code: ColorCode
}

impl Cursor {
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

	pub fn get_pos(&self) -> (usize, usize) {
		(self.x, self.y)
	}

	pub fn set_pos(&mut self, x: usize, y: usize) {
		self.x = x;
		self.y = y;
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
	ascii_code: u8,
	color_code: ColorCode
}

const VGABUFF_OFFSET: usize = 0xb8000;
const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer {
chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT]
}

pub struct Writer {
	cursor:		&'static mut Cursor,
	color_code: ColorCode,
	buffer:     &'static mut Buffer
}

/*
 *	Implementation of writer functions
 */
impl Writer {
	/*	Write one byte to vga buffer, update CURSOR position	*/
	pub fn write_byte(&mut self, byte: u8) {
	/*	Writing each byte to qemu serial port for external log	*/
		io::outb(0x3f8, byte);
		match byte {
			b'\n' => self.new_line(),
			byte => {
				let mut code = byte;
				let mut pos: (usize, usize) = self.cursor.get_pos();
				if byte == 0x08
				{
					if pos.0 == 0
						{return ;}
					pos.0 -= 1;
					code = 0x0
				}
				else if pos.0 >= BUFFER_WIDTH {
						self.new_line();
						pos = self.cursor.get_pos();
				}
				self.buffer.chars[pos.1][pos.0] = ScreenChar {
					ascii_code: code,
					color_code: self.color_code,
				};
				if byte != 0x08
					{pos.0 += 1;}
				self.cursor.set_pos(pos.0, pos.1);
			}
		}
	}

	/*	Move CURSOR one line lower and move all lines if needed */
	fn new_line(&mut self) {
		let pos: (usize, usize) = self.cursor.get_pos();
		let mut y = pos.1;
		if pos.1 != BUFFER_HEIGHT - 1 {
			y += 1;
		}
		else {
			for row in 1..BUFFER_HEIGHT
			{self.buffer.chars[row -1] = self.buffer.chars[row];}
			self.clear_row(BUFFER_HEIGHT -1);
		}
		self.cursor.set_pos(0, y);
	}

	/*		Simply replace all row by spaces to visualy clear it */
	pub fn clear_row(&mut self, row: usize) {
		for i in 0..BUFFER_WIDTH {
			self.buffer.chars[row][i] = ScreenChar {
				ascii_code: 0x20,
				color_code: self.color_code,
			};
		}
	}

	/*	Write string to vga using write_byte functions if printable, else print a square */
	pub fn write_string(&mut self, s: &str) {
		self.cursor.disable();
		for byte in s.bytes() {
			match byte {
			// printable ASCII byte or newline
				0x20..=0x7e | b'\n' | 0x08 => self.write_byte(byte),
			// not part of printable ASCII range
				_ => self.write_byte(0xfe),
			}
		}
		self.cursor.update();
		self.cursor.enable();
	}

	pub fn chcolor(&mut self, new_color: ColorCode) {
		self.color_code = new_color;
	}
}

/*	Tells rust how to use our writer as a format writer */
impl fmt::Write for Writer {
	fn write_str(&mut self, s: &str) -> fmt::Result {
		self.write_string(s);
		Ok(())
	}
}

pub static mut CURSOR:Cursor = Cursor{
	x: 0,
	y: 0,
	color_code: ColorCode::new(Color::White, Color::Black)
};

/* Reimplementation of rust print and println macros */
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

/* Setting our panic handler to our brand new println */
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	unsafe{CURSOR.color_code = ColorCode::new(Color::Red, Color::Black);}
	println!("{}", info);
	unsafe{CURSOR.color_code = ColorCode::new(Color::White, Color::Black);}
	loop {}
}

pub fn _print(args: fmt::Arguments) {
	use core::fmt::Write;

	let mut writer: Writer = Writer {
		cursor: unsafe{&mut CURSOR}, //Cursor{x: 0, y: 0},
		color_code: unsafe{CURSOR.color_code},
		buffer: unsafe { &mut *(VGABUFF_OFFSET as *mut Buffer) },
	};

	writer.write_fmt(args).unwrap();
}

