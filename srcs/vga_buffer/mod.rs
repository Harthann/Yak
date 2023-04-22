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

impl Default for Screen {
	fn default() -> Self {
		Screen {
			cursor:  Cursor::new(
				0,
				0,
				ColorCode::default()
			),
			buffer:  [ScreenChar {
				ascii_code: 0,
				color_code: ColorCode::default()
			}; BUFFER_WIDTH * BUFFER_HEIGHT],
			command: Command::new()
		}
	}
}

impl Screen {
	pub const fn new() -> Screen {
		Screen {
			cursor:  Cursor::new(
				0,
				0,
				ColorCode::new_default()
			),
			buffer:  [ScreenChar {
				ascii_code: 0,
				color_code: ColorCode::new_default()
			}; BUFFER_WIDTH * BUFFER_HEIGHT],
			command: Command::new()
		}
	}

	pub fn reset(&mut self) {
        todo!()
		//for i in 0..BUFFER_HEIGHT {
		//	for j in 0..BUFFER_WIDTH {
		//		self.buffer[i][j] = ScreenChar {
		//			ascii_code: b' ',
		//			color_code: ColorCode::default()
		//		};
		//	}
		//}
		//self.cursor.set_pos(0, 0);
		//self.command.clear();
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
impl Default for ScreenChar {
    fn default() -> Self {
        Self { ascii_code: b' ', color_code: ColorCode::default() }
    }
}

const VGABUFF_OFFSET: usize = 0xc00b8000;
const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;
const VGA_LEN: usize = BUFFER_WIDTH * BUFFER_HEIGHT;

type Buffer = [ScreenChar; BUFFER_WIDTH * BUFFER_HEIGHT];

#[link_section = ".vga_buffer"]
static mut VGA_BUFFER: Buffer = [ScreenChar {
	ascii_code: 0x20,
	color_code: ColorCode::new(Color::White, Color::Black)
}; BUFFER_WIDTH * BUFFER_HEIGHT];
pub static WRITER: Mutex<Writer, true> = Mutex::<Writer, true>::new(Writer {
	screen_index: 0,
	cursor:       Cursor::new(0, 0, ColorCode::new_default()),
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
			b'\n' => self.scroll_up(),
			byte => {
                let screenchar = ScreenChar {
					ascii_code: byte,
					color_code: self.cursor.get_color_code()
				};
				
                if self.cursor.get_pos() >= VGA_LEN {
                    self.scroll_up();
                }

                self.vga_buffer[self.cursor.get_pos()] = screenchar;
                self.move_cursor(1);
			}
		}
	}

    pub fn del_byte(&mut self) {
        let color = self.cursor.get_color_code();
		self.move_cursor(-1);
        self.vga_buffer[self.cursor.get_pos()] = ScreenChar {
            ascii_code: b' ',
            color_code: color
        };
    }

    pub fn move_cursor(&mut self, x: i32) {
        let mut pos: i32 = self.cursor.get_pos() as i32;
        // It is safe to convert pos to i32 since it will be at most 2000
        // let mut real_pos: i32 = (pos.1 * BUFFER_WIDTH + pos.0) as i32;
        pos += x;
        pos = pos.max(0);
        pos = pos.min(VGA_LEN as i32);
        // Convertion to usize is safe since pos is bound between 0 and VGA_LEN
        self.cursor.set_pos(pos as usize);
        self.cursor.update();
    }

	/// Move CURSOR one line lower and move all lines if needed
	fn scroll_up(&mut self) {
        let mut movement: i32 = (self.cursor.get_pos() % BUFFER_WIDTH) as i32;
		if self.cursor.get_pos() < VGA_LEN - BUFFER_WIDTH {
			movement = BUFFER_WIDTH as i32 - movement;
		} else {
            if self.cursor.get_pos() / BUFFER_WIDTH == 25 {
                movement = BUFFER_WIDTH as i32;
            }
            self.vga_buffer.copy_within(BUFFER_WIDTH..VGA_LEN, 0);
            self.vga_buffer[VGA_LEN-BUFFER_WIDTH..VGA_LEN].fill(ScreenChar::default());
            movement = -movement;
		}
        self.move_cursor(movement);
	}

	/// Simply replace all row by spaces to visualy clear it
	pub fn clear_row(&mut self, row: usize) {
        self.vga_buffer[row * BUFFER_WIDTH..(row + 1) * BUFFER_WIDTH]
            .fill(ScreenChar::default());
	}

	pub fn clear(&mut self) {
        self.vga_buffer.fill(ScreenChar::default());
	}

	// Write string to vga using write_byte functions if printable, else print a square
	pub fn write_string(&mut self, s: &str) {
		self.cursor.disable();
		for byte in s.bytes() {
			match byte {
				// printable ASCII byte or newline
				0x20..=0x7e | b'\n' => self.write_byte(byte),
                0x08 => self.del_byte(),
				// not part of printable ASCII range
				_ => self.write_byte(0xfe)
			}
		}
		self.cursor.update();
		self.cursor.enable();
	}

	pub fn save(&self, screen: &mut Screen) {
		screen.buffer.copy_from_slice(self.vga_buffer);
		screen.cursor = self.cursor;
	}

	pub fn render(&mut self, screen: &mut Screen) {
		self.cursor.disable();

		self.vga_buffer.copy_from_slice(screen.buffer.as_mut());
		self.cursor = screen.cursor;

		self.cursor.update();
		self.cursor.enable();
		if self.cursor.get_pos() == 0 {
			self.write_string("$> ");
		}
	}

	pub fn get_screen(&mut self) -> usize {
        todo!();
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
		$crate::kprint!("{}\n", format_args!($($arg)*))
	)
}

// Setting our panic handler to our brand new kprintln
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    unsafe { crate::dprintln!("{}", info); }
	WRITER
		.lock()
		.chcolor(ColorCode::new(Color::Red, Color::Black));
	kprintln!("{}", info);
	WRITER
		.lock()
		.chcolor(ColorCode::default());
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
			.chcolor(ColorCode::default())
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

macro_rules! screenclear {
	() => {
		$crate::vga_buffer::WRITER.lock().clear()
	};
}

pub(crate) use {change_color, screenclear};
