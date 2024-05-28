use crate::alloc::string::String;
use crate::alloc::vec::Vec;

use crate::println;

pub fn jiffies(_: Vec<String>) {
	println!("Jiffies: {}", crate::time::jiffies());
}

pub fn uptime(_: Vec<String>) {
	let time = crate::time::get_timestamp();
	println!(
		"Time elapsed since boot: {}s {}ms",
		time.second, time.millisecond
	);
}

pub fn date(_: Vec<String>) {
	println!("{}", crate::cmos::get_time());
}
