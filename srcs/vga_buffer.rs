/*  Crate import */
use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;
use core::panic::PanicInfo;


/*
 *	Importing extern variable from assembly code to get cursor position
 */
extern "C" {
	static cursor: u32;
}

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
struct ColorCode(u8);
impl ColorCode {
	fn new(foreground: Color, background: Color) -> ColorCode {
		ColorCode((background as u8) << 4 | (foreground as u8))
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
	ascii_code: u8,
	color_code: ColorCode
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer {
chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT]
}

pub struct Writer {
	posx:       usize,
	posy:       usize,
	color_code: ColorCode,
	buffer:     &'static mut Buffer
}

/*
 *	Implementation of writer functions
 */
impl Writer {
	/*	Write one byte to vga buffer, update cursor position	*/
	pub fn write_byte(&mut self, byte: u8) {
		match byte {
			b'\n' => self.new_line(),
			byte => {
				if self.posx >= BUFFER_WIDTH {
					self.new_line();
				}

				self.buffer.chars[self.posy][self.posx] = ScreenChar {
					ascii_code: byte,
					color_code: self.color_code,
				};
				self.posx += 1;
			}
		}
	}

	/*	Move cursor one line lower and move all lines if needed */
	fn new_line(&mut self) {
		if self.posy != 24 {
			self.posy += 1;
		}
		else {
			for row in 1..BUFFER_HEIGHT
			{self.buffer.chars[row -1] = self.buffer.chars[row];}
			self.clear_row(BUFFER_HEIGHT -1);
		}
		self.posx = 0;
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
		for byte in s.bytes() {
			match byte {
			// printable ASCII byte or newline
				0x20..=0x7e | b'\n' => self.write_byte(byte),
			// not part of printable ASCII range
				_ => self.write_byte(0xfe),
			}
		}
		// move cursor
	}
}

/*	Tells rust how to use our writer as a format writer */
impl fmt::Write for Writer {
	fn write_str(&mut self, s: &str) -> fmt::Result {
		self.write_string(s);
		Ok(())
	}
}

/* Static writer to print on vga buffer werether we are in code
 * Using lazy static in order to tell rust that size will be known
 * at runtime. We use spin mutexes as well to protect race condition
 * and since no thread and "mutex" are developped we need spinlock
 */
lazy_static! {
	pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
		posx: unsafe{(cursor - 0xb8000) as usize % 160},
		posy: unsafe{(cursor - 0xb8000) as usize / 160},
		color_code: ColorCode::new(Color::White, Color::Black),
		buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
	});
}

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
	println!("{}", info);
	loop {}
}

pub fn _print(args: fmt::Arguments) {
	use core::fmt::Write;
	WRITER.lock().write_fmt(args).unwrap();
}

