use core::sync::atomic::{AtomicUsize, Ordering};
#[no_mangle]
static JIFFIES: AtomicUsize = AtomicUsize::new(0);

// Number of ms between each irq0
// This value should be written only onth at the boot
#[no_mangle]
pub static mut SYSTEM_FRACTION: f64 = 1.0;

pub struct Time {
	pub second:      usize,
	pub millisecond: usize
}

impl Time {
	pub fn as_f64(&self) -> f64 {
		(self.second as f64 * 1000.0) + (self.millisecond as f64 / 1000.0)
	}
}

/// Construct a Time structure using the JIFFIES and SYSTEM_FRACTION to calculate time elapsed
/// since boot
#[inline(always)]
pub fn get_timestamp() -> Time {
	// SYSTEM_FRACTION shouldn't be change after boot and should so be safe
	unsafe {
		let total_ms =
			(JIFFIES.load(Ordering::Relaxed) as f64 * SYSTEM_FRACTION) as usize;
		Time {
			second:      total_ms / 1000,
			millisecond: total_ms - (total_ms / 1000)
		}
	}
}

/// Increment by one the JIFFIES counter
#[inline(always)]
#[no_mangle]
pub fn jiffies_inc() {
	JIFFIES.fetch_add(1, Ordering::Relaxed);
}

/// Return the value stored in the JIFFIES static variable
#[inline(always)]
pub fn jiffies() -> usize {
	JIFFIES.load(Ordering::Relaxed)
}

/// Sleep until x millisecond have passed
pub fn sleep(ms: usize) {
	if ms > 1000 {
		// Due to interrupt frequency and time to return to this job
		// this sleep is only perform for sleep higher than a second
		sleep_ms(ms);
	} else {
		// loop over io_wait to delay
		raw_delay_ms(ms);
	}
}

/// Wait x millisecond looping over io_wait
/// This is quite imprecise but do the job
fn raw_delay_ms(ms: usize) {
	for _ in 0..(ms * 1000) {
		microsleep();
	}
}

#[inline]
fn sleep_ms(ms: usize) {
	unsafe {
		let saved_time =
			(JIFFIES.load(Ordering::Relaxed) as f64 * SYSTEM_FRACTION) as usize;
		while saved_time + ms
			> (JIFFIES.load(Ordering::Relaxed) as f64 * SYSTEM_FRACTION)
				as usize
		{
			crate::wrappers::sti!();
			crate::wrappers::hlt!();
			crate::wrappers::cli!();
		}
	}
}

/// Unaccurate sleep for 1 microsecond
/// io_wait should take 1~4 microsecond as stated in osdev
/// But using io_wait as a microsecond seems to be too fast
/// Slowing it down 40 times looks to do the job
/// Test where performed by listening to the mario music implemented
/// This is surely imprecise but gives a raw idea if you're timing are too fast or slow
#[inline]
pub fn microsleep() {
	for _ in 0..40 {
		// io wait ~1-4 microseconds
		crate::io::io_wait();
	}
}


macro_rules! isleap {
    ($arg: tt) => {
        (($arg % 4) == 0 && ($arg % 100) != 0) || (($arg % 400) == 0)
    }
}
//#define isleap(y) ((((y) % 4) == 0 && ((y) % 100) != 0) || ((y) % 400) == 0)
const WDAYS: [&str; 7] = [
	"Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"
];
const MONTHS: [&str; 12] = [
	"Jan", "Feb", "Mar", "Apr", "May", "Jun",
	"Jul", "Aug", "Sep", "Oct", "Nov", "Dec"
];
const MONTHCNT: [u8; 12] = [
	31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31
];

// This implementation seems incorrect
pub fn ctime(mut timestamp: u32) -> crate::string::String {
	let ss = timestamp % 60;
	timestamp /= 60;	/* minutes */
	let mm = timestamp % 60;
	timestamp /= 60;	/* hours */
	let hh = timestamp % 24;
	timestamp /= 24;	/* days */
	let wday = (4 + timestamp) % 7;	/* weekday, 'twas thursday when time started */

    let mut year = 1970;
    while timestamp >= 365 {
        timestamp = timestamp - match isleap!(year) {
            true => 366,
            false => 365
        };
		//timestamp -= ? 366: 365;
        year = year + 1;
    }

	timestamp = timestamp + 1;	/* days are 1-based */

    let mut month = 0;
    while timestamp > MONTHCNT[month] as u32 {
		timestamp = timestamp - MONTHCNT[month] as u32;
        month = month + 1;
    }

	if month > 2 && isleap!(year) {
		timestamp = timestamp - 1;
    }
    let date = crate::alloc::format!("{} {}{:3} {:02}:{:02}:{:02} {}",
             WDAYS[wday as usize], MONTHS[month], timestamp, hh, mm, ss, year);
    /*
	snprintf(buf, sizeof buf, "%s %s%3d %02d:%02d:%02d %d\n",
	    ((wday  < 0 || wday  >=  7)? "???": wdays[wday]),
	    ((month < 0 || month >= 12)? "???": months[month]),
	    (int)timestamp, hh, mm, ss, year);
        */
    return date;
}
