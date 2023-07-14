use crate::alloc::vec::Vec;
use crate::alloc::string::String;

pub fn jiffies(_: Vec<String>) {
	crate::kprintln!("Jiffies: {}", crate::time::jiffies());
}

pub fn uptime(_: Vec<String>) {
	let time = crate::time::get_timestamp();
	crate::kprintln!(
		"Time elapsed since boot: {}s {}ms",
		time.second,
		time.millisecond
	);
}

pub fn date(_: Vec<String>) {
	crate::kprintln!("{}", crate::cmos::get_time());
}

