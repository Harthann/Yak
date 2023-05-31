use core::fmt;
use core::fmt::Write;

const SERIAL_COM1: u16 = 0x3f8;
const SERIAL_COM2: u16 = 0x2f8;
const SERIAL_COM3: u16 = 0x3e8;
const SERIAL_COM4: u16 = 0x2e8;
const SERIAL_COM5: u16 = 0x5f8;
const SERIAL_COM6: u16 = 0x4f8;
const SERIAL_COM7: u16 = 0x5e8;
const SERIAL_COM8: u16 = 0x4e8;

/// Dummy structure to impl fmt::write
pub struct DWriter {}

impl fmt::Write for DWriter {
	/// Send to SERIAL_COM2 all bytes from s
	fn write_str(&mut self, s: &str) -> fmt::Result {
		for i in s.bytes() {
			crate::io::outb(SERIAL_COM2, i);
		}
		Ok(())
	}
}

#[macro_export]
macro_rules! dprint {
	($($arg:tt)*) => (
		$crate::debug::_print(format_args!($($arg)*))
	)
}

#[macro_export]
macro_rules! dprintln {
	() => ($crate::dprint!("\n"));
	($($arg:tt)*) => (
		$crate::dprint!("[{: >12.6}] {}\n",
                        crate::time::get_timestamp().as_f64(),
                        format_args!($($arg)*))
	)
}

/// Wrapper function to call from print macros
pub fn _print(args: fmt::Arguments) {
	let mut dwriter = DWriter {};
	dwriter.write_fmt(args).unwrap();
}
