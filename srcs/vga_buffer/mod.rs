//! Handler for vga buffer

use crate::{io, Command};
use core::fmt;
use core::fmt::Write;
use core::panic::PanicInfo;

pub mod color;
use color::{Color, ColorCode};
mod cursor;
use cursor::Cursor;

use crate::spin::Mutex;

#[derive(Debug, Clone)]
pub struct Screen {
	cursor:  Cursor,
	buffer:  Buffer,
	command: Command
}

unsafe impl Send for Screen {}

impl Screen {
	pub const fn new() -> Screen {
		Screen {
			cursor:  Cursor::new(
				0,
				0,
				ColorCode::new(Color::White, Color::Black)
			),
			buffer:  [[ScreenChar {
				ascii_code: 0,
				color_code: ColorCode::new(Color::White, Color::Black)
			}; BUFFER_WIDTH]; BUFFER_HEIGHT],
			command: Command::new()
		}
	}

	pub fn reset(&mut self) {
		for i in 0..BUFFER_HEIGHT {
			for j in 0..BUFFER_WIDTH {
				self.buffer[i][j] = ScreenChar {
					ascii_code: b' ',
					color_code: ColorCode::new(Color::White, Color::Black)
				};
			}
		}
		self.cursor.set_pos(0, 0);
		self.command.clear();
	}

	pub fn get_command(&mut self) -> &mut Command {
		&mut self.command
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
	ascii_code: u8,
	color_code: ColorCode
}

const VGABUFF_OFFSET: usize = 0xc00b8000;
const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

pub const NB_SCREEN: usize = 3;
pub static SCREENS: Mutex<[Screen; NB_SCREEN], true> =
	Mutex::new([Screen::new(), Screen::new(), Screen::new()]);

type Buffer = [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT];

#[link_section = ".vga_buffer"]
static mut VGA_BUFFER: Buffer = [[ScreenChar {
	ascii_code: 0x20,
	color_code: ColorCode::new(Color::White, Color::Black)
}; BUFFER_WIDTH]; BUFFER_HEIGHT];
pub static WRITER: Mutex<Writer, true> = Mutex::<Writer, true>::new(Writer {
	screen_index: 0,
	cursor:       Cursor::new(0, 0, ColorCode::new(Color::White, Color::Black)),
	vga_buffer:   unsafe { &mut VGA_BUFFER }
});

pub struct Writer {
	screen_index: usize,
	cursor:       Cursor,
	vga_buffer:   &'static mut Buffer
}

unsafe impl Send for Writer {}
// 	Implementation of writer functions
impl Writer {
	// Write one byte to vga buffer, update CURSOR position
	pub fn write_byte(&mut self, byte: u8) {
		// Writing each byte to qemu serial port for external log
		io::outb(0x3f8, byte);
		match byte {
			b'\n' => self.new_line(),
			byte => {
				let mut code = byte;
				let mut pos: (usize, usize) = self.cursor.get_pos();
				if byte == 0x08 {
					if pos.0 != 0 {
						pos.0 -= 1;
					} else if pos.1 != 0 {
						pos.1 -= 1;
						pos.0 = 79;
					}
					code = 0x0;
				} else if pos.0 >= BUFFER_WIDTH {
					self.new_line();
					pos = self.cursor.get_pos();
				}
				let screenchar = ScreenChar {
					ascii_code: code,
					color_code: self.cursor.get_color_code()
				};
				self.vga_buffer[pos.1][pos.0] = screenchar;
				if byte != 0x08 {
					pos.0 += 1;
				}
				self.cursor.set_pos(pos.0, pos.1);
				self.cursor.update();
			}
		}
	}

	// Move CURSOR one line lower and move all lines if needed
	fn new_line(&mut self) {
		let pos: (usize, usize) = self.cursor.get_pos();
		let mut y = pos.1;
		if pos.1 != BUFFER_HEIGHT - 1 {
			y += 1;
		} else {
			for row in 1..BUFFER_HEIGHT {
				self.vga_buffer[row - 1] = self.vga_buffer[row];
			}
			self.clear_row(BUFFER_HEIGHT - 1);
		}
		self.cursor.set_pos(0, y);
	}

	// Simply replace all row by spaces to visualy clear it
	pub fn clear_row(&mut self, row: usize) {
		for i in 0..BUFFER_WIDTH {
			let screenchar = ScreenChar {
				ascii_code: 0x20,
				color_code: ColorCode::new(Color::White, Color::Black)
			};
			self.vga_buffer[row][i] = screenchar;
		}
	}

	pub fn clear(&mut self) {
		todo!();
	}

	// Write string to vga using write_byte functions if printable, else print a square
	pub fn write_string(&mut self, s: &str) {
		self.cursor.disable();
		for byte in s.bytes() {
			match byte {
				// printable ASCII byte or newline
				0x20..=0x7e | b'\n' => self.write_byte(byte),
				// not part of printable ASCII range
				_ => self.write_byte(0xfe)
			}
		}
		self.cursor.update();
		self.cursor.enable();
	}

	pub fn change_screen(&mut self, nb: usize) {
		let mut screen_guard = SCREENS.lock();
		// Should copy vga to current buffer index
		self.cursor.disable();

		screen_guard[self.screen_index]
			.buffer
			.copy_from_slice(self.vga_buffer.as_mut());
		screen_guard[self.screen_index].cursor = self.cursor;

		self.screen_index = nb;
		self.vga_buffer
			.copy_from_slice(screen_guard[self.screen_index].buffer.as_mut());
		self.cursor = screen_guard[self.screen_index].cursor;

		self.cursor.update();
		self.cursor.enable();
		if self.cursor.get_pos() == (0, 0) {
			self.write_string("$> ");
		}
	}

	pub fn get_screen(&mut self) -> usize {
		self.screen_index
	}

	pub fn chcolor(&mut self, new_color: ColorCode) {
		self.cursor.set_color_code(new_color);
	}
}

// Tells rust how to use our writer as a format writer
impl fmt::Write for Writer {
	fn write_str(&mut self, s: &str) -> fmt::Result {
		self.write_string(s);
		Ok(())
	}
}

// Reimplementation of rust print and println macros
#[macro_export]
macro_rules! kprint {
	($($arg:tt)*) => (
		$crate::vga_buffer::_print(format_args!($($arg)*))
	)
}

#[macro_export]
macro_rules! kprintln {
	() => ($crate::kprint!("\n"));
	($($arg:tt)*) => (
		$crate::kprint!("{}\n", format_args!($($arg)*));
	)
}

// Setting our panic handler to our brand new kprintln
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	WRITER
		.lock()
		.chcolor(ColorCode::new(Color::Red, Color::Black));
	kprintln!("{}", info);
	WRITER
		.lock()
		.chcolor(ColorCode::new(Color::White, Color::Black));
	loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	unsafe {
		WRITER
			.lock()
			.chcolor(ColorCode::new(Color::Red, Color::Black))
	};
	kprintln!("[failed]");
	kprintln!("{}", info);
	unsafe {
		WRITER
			.lock()
			.chcolor(ColorCode::new(Color::White, Color::Black))
	};
	io::outb(0xf4, 0x11);
	loop {}
}

pub fn _print(args: fmt::Arguments) {
	WRITER.lock().write_fmt(args).unwrap();
}

pub fn hexdump(ptr: *const u8, size: usize) {
	let mut i: usize = 0;

	while i < size {
		kprint!("{:08x}: ", unsafe { ptr.offset(i as isize) as usize });
		let nb = if size - i > 16 { 16 } else { size - i };
		for j in 0..nb {
			let byte: u8 = unsafe { *(ptr.offset((i + j) as isize)) as u8 };
			kprint!("{:02x}", byte);
			if j % 2 == 1 {
				kprint!(" ");
			}
		}
		for j in 0..16 - nb {
			if j % 2 == 0 {
				kprint!(" ");
			}
			kprint!("  ");
		}
		for j in 0..nb {
			let byte: u8 = unsafe { *(ptr.offset((i + j) as isize)) as u8 };
			if byte >= 0x20 && byte < 0x7f {
				// printable
				kprint!("{}", byte as char);
			} else {
				kprint!(".");
			}
		}
		kprint!("\n");
		i += 16;
	}
}

macro_rules! change_color {
	($fg:expr, $bg:expr) => {
		$crate::vga_buffer::WRITER
			.lock()
			.chcolor($crate::vga_buffer::color::ColorCode::new($fg, $bg))
	};
}

macro_rules! clihandle {
	($arg:expr) => {
		unsafe {
			let screen_number = crate::vga_buffer::WRITER.lock().get_screen();
			$crate::vga_buffer::SCREENS.lock()[screen_number]
				.get_command()
				.handle($arg);
		}
	};
}

macro_rules! screenclear {
	() => {
		$crate::vga_buffer::WRITER.lock().clear()
	};
}

pub(crate) use {change_color, clihandle, screenclear};
