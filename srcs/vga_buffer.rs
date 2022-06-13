use core::fmt;


#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
	Black = 0,
	Blue = 1,
	Green = 2,
	Cyan = 3,
	Red = 4,
	Magenta = 5,
	Brown = 6,
	LightGray = 7,
	DarkGray = 8,
	LightBlue = 9,
	LightGreen = 10,
	LightCyan = 11,
	LightRed = 12,
	Pink = 13,
	Yellow = 14,
	White = 15,
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
	posx: usize,
	posy: usize,
	color_code: ColorCode,
	buffer: &'static mut Buffer
}


impl Writer {
	pub fn write_byte(&mut self, byte: u8) {
		match byte {
			b'\n' => self.new_line(),
			byte => {
				if self.posx >= BUFFER_WIDTH {
					self.new_line();
				}

				let row = self.posy;
				let col = self.posx;

				let color_code = self.color_code;
				self.buffer.chars[row][col] = ScreenChar {
					ascii_code: byte,
					color_code,
				};
				self.posx += 1;
			}
		}
	}

	fn new_line(&mut self) {
		if self.posy != 25 {
			self.posy += 1;
		}
		else {
			for row in 1..BUFFER_HEIGHT {
				for col in 0..BUFFER_WIDTH {
					let character = self.buffer.chars[row][col];
					self.buffer.chars[row - 1][col] = character;
				}
			}
			self.clear_row(BUFFER_HEIGHT -1);
		}
		self.posx = 0;
	}

	fn clear_row(&mut self, row: usize) {
		for i in 0.. BUFFER_WIDTH {
			self.buffer.chars[row][i] = ScreenChar {
				ascii_code: 0x20,
				color_code: self.color_code,
			};
		}
	}

	pub fn write_string(&mut self, s: &str) {
		for byte in s.bytes() {
			match byte {
				// printable ASCII byte or newline
				0x20..=0x7e | b'\n' => self.write_byte(byte),
				// not part of printable ASCII range
				_ => self.write_byte(0xfe),
			}

		}
	}
}

impl fmt::Write for Writer {
	fn write_str(&mut self, s: &str) -> fmt::Result {
		self.write_string(s);
		Ok(())
	}
}

extern "C" {
	static cursor: u32;
}

pub fn print_something() {
	use core::fmt::Write;
	let mut writer = Writer {
		posx: unsafe{(cursor - 0xb8000) as usize % 160},
		posy: unsafe{(cursor - 0xb8000) as usize / 160},
		color_code: ColorCode::new(Color::White, Color::Black),
		buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
	};

	writer.write_string("Hello\n");
	writer.write_string("World!\n");
	write!(writer, "The numbers are {} and {}\n", 42, 1.0/3.0).unwrap();
	for _i in 1..21 {
		writer.write_string("World!\n");
	}
}


