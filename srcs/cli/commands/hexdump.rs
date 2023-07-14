use crate::kprintln;
use crate::alloc::vec::Vec;
use crate::alloc::string::String;
use crate::vga_buffer::hexdump;

pub fn hextou(string: &str) -> Option<usize> {
	let slice = string.chars();
	let mut addr: usize = 0;

	if !string.starts_with("0x") {
		return None;
	}
	for i in slice.skip(2) {
		if !i.is_ascii_hexdigit() {
			return None;
		}
		let byte = i.to_digit(16).unwrap();
		addr = (addr << 4) | (byte & 0xf) as usize;
	}
	return Some(addr);
}

pub fn atou(string: &str) -> Option<usize> {
	if string.starts_with("0x") {
		return hextou(string);
	}
	string.parse::<usize>().ok()
}

pub fn hexdump_parser(command: Vec<String>) {
	let mut args: [usize; 2] = [0, 0];

	if command.len() != 3 {
		kprintln!("Invalid number of arguments.");
		kprintln!("Usage: hexdump [addr] [size]");
		return;
	}

	if let Some(res) = atou(command[1].as_str()) {
		args[0] = res;
	} else {
		kprintln!("Invalid number of arguments.");
		kprintln!("Usage: hexdump [addr] [size]");
		return;
	}

	if let Some(res) = atou(command[2].as_str()) {
		args[1] = res;
	} else {
		kprintln!("Invalid number of arguments.");
		kprintln!("Usage: hexdump [addr] [size]");
		return;
	}

	hexdump(args[0] as *const u8, args[1]);
}
