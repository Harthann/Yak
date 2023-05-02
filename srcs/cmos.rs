use core::fmt;

pub struct Time {
	pub seconds: u8,
	pub minutes: u8,
	pub hours:   u8,
	pub weekday: u8,
	pub day:     u8,
	pub month:   u8,
	pub year:    u8,
	pub century: u8
}

impl fmt::Display for Time {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(
			f,
			"{:02}/{:02}/{}{} {:02}:{:02}:{:02}",
			self.day,
			self.month,
			self.century,
			self.year,
			self.hours,
			self.minutes,
			self.seconds
		)
	}
}

const CMOS_CMD: u16 = 0x70;
const CMOS_DATA: u16 = 0x71;

#[allow(non_snake_case)]
fn get_RTC_register(reg: u8) -> u8 {
	crate::io::outb(CMOS_CMD, reg);
	crate::io::inb(CMOS_DATA)
}

macro_rules! from_bcd {
	($val:expr) => {
		(($val / 16) * 10 + ($val & 0xf))
	};
}

/// Ask cmos for actual date and time
/// Current implementation isn't either optimal or completely precise
/// 1) Change can occur while reading cmos register
/// 2) Assume that we always need to convert from bcd value
pub fn get_time() -> Time {
	Time {
		seconds: from_bcd!(get_RTC_register(0x00)),
		minutes: from_bcd!(get_RTC_register(0x02)),
		hours:   from_bcd!(get_RTC_register(0x04)),
		weekday: from_bcd!(get_RTC_register(0x06)),
		day:     from_bcd!(get_RTC_register(0x07)),
		month:   from_bcd!(get_RTC_register(0x08)),
		year:    from_bcd!(get_RTC_register(0x09)),
		century: from_bcd!(get_RTC_register(0x32))
	}
}
