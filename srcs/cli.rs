use core::arch::asm;
use crate::{print, println, hexdump, screenclear};
use crate::io;
use core::mem::size_of;

pub static COMMANDS: [fn(&Command); 5] = [reboot, halt, hexdump_parser, clear, help];
const KNOWN_CMD: [&str; 5]= ["reboot", "halt", "hexdump", "clear", "help"];

fn reboot(_: &Command) {
	io::outb(0x64, 0xfe);
}

fn halt(_: &Command) {
	unsafe {
		asm!("hlt");
	}
}

fn clear(_: &Command) {
	screenclear!();
}

fn help(_: &Command) {
	println!("Available commands:");
	for i in KNOWN_CMD {
		println!("    {}", i);
	}
}

fn hextou(slice: &[char]) -> Option<usize> {
	let mut addr: usize = 0;
	let mut byte;

	if slice.len() < 2 || slice.len() > (2 + size_of::<usize>() * 2 ) || (slice[0] != '0' || slice[1] != 'x') {
		return None;
	}
	for i in slice.iter().skip(2) {
		if !i.is_ascii_hexdigit() {
			return None;
		}
		if i.is_ascii_digit()	{
			byte = *i as u8 - b'0';
		} else {
			byte = *i as u8 - b'a' + 10;
		}
		addr = (addr << 4) | (byte & 0xf) as usize;
	}
	return Some(addr);
}

fn atou(slice: &[char]) -> Option<usize> {
	let mut num: usize = 0;

	if slice[0] == '-' {
		return None;
	}
	for i in slice {
		if !i.is_ascii_digit() {
			return None;
		}
		num *= 10;
		num += (*i as u8 - b'0') as usize;
	}
	return Some(num);
}

fn hexdump_parser(command: &Command) {
	let cmd = command.command;
	let iter = cmd.split(|a| *a == ' ' || *a == '\t' || *a == '\0');

	let mut count: i32 = 0;
	let mut addr: usize = 0;
	let mut size: usize = 0;

	for i in iter.clone() {
		if i.len()	!= 0 {
			count += 1
		}
	}
	if count != 3 {
		println!("Invalid number of argument");
		return ;
	}
	for (index, item) in iter.enumerate() {
		if index == 1  {
			match hextou(item) {
			Some(x) => addr = x,
			_		=> {println!("Invalid arg"); return;},
			}
		} else if index == 2 {
			match atou(item) {
			Some(x) => size = x,
			_		=> {println!("Invalid arg"); return;},
			}
		}
	}
	hexdump!(addr as *const u8, size);
}

#[derive(Debug, Clone, Copy)]
pub struct Command {
	pub command: [char; 76],
	pub length: usize,
}

impl Command {
	pub const fn new() -> Command {
		Command {
			command: ['\0'; 76],
			length: 0
		}
	}

	fn append(&mut self, x: char) -> Result<(), ()> {
		if self.length < 76 {
			self.command[self.length] = x;
			self.length += 1;
			return Ok(());
		}
		return Err(());
	}

	pub fn pop_back(&mut self) {
		if self.length != 0 {
			self.length -= 1;
			self.command[self.length] = '\0';
		}
	}

	pub fn clear(&mut self) {
		while  {
			if self.length != 0 {
				self.length -= 1;
			}
			self.command[self.length] = '\0';
			self.length != 0
		} {}
	}

	pub fn is_known(&self) -> Option<usize> {
		let mut j = 0;
		while j < KNOWN_CMD.len() {
			let len = KNOWN_CMD[j].chars().count();
			if (self.command[len] == '\0' || self.command[len] == ' ')
				&& KNOWN_CMD[j].chars().zip(self.command.iter()).position(|(a, b)| a != *b) == None
			{
				return Some(j);
			}
			j += 1;
		}
		return None;
	}

	pub fn handle(&mut self, charcode: char) {
		if charcode >= ' ' && charcode <= '~' {
			if self.append(charcode).is_err() {
				println!("Can't handle longer command, clearing buffer");
				print!("$> ");
				self.clear();
			}
		} else if charcode == '\x08' {
			self.pop_back();
		} else if charcode == '\n' {
			match self.is_known() {
				Some(x) => COMMANDS[x](&self),
				_		=> {if self.length != 0 {println!("Unknown command. Type `help` to list available commands")}},
			}
			self.clear();
			print!("$> ");
		}
	}
}
