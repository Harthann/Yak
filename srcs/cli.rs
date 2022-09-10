use core::arch::asm;

use crate::{kprint, kprintln, hexdump, screenclear};
use crate::io;
use crate::string::String;
use crate::memory::allocator;

pub static COMMANDS: [fn(&Command); 6] = [reboot, halt, hexdump_parser, clear, help, shutdown];
const KNOWN_CMD: [&str; 6]= ["reboot", "halt", "hexdump", "clear", "help", "shutdown"];

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
	kprintln!("Available commands:");
	for i in KNOWN_CMD {
		kprintln!("    {}", i);
	}
}

fn shutdown(_: &Command) {
    io::outb(0xf4, 0x10);
}

fn hextou(string: &str) -> Option<usize> {
	let slice = string.chars();
	let mut addr: usize = 0;
	let mut byte;

	if !string.starts_with("0x") {
		return None;
	}
	for i in slice.skip(2) {
		if !i.is_ascii_hexdigit() {
			return None;
		}
		if i.is_ascii_digit()	{
			byte = i as u8 - b'0';
		} else {
			byte = i as u8 - b'a' + 10;
		}
		addr = (addr << 4) | (byte & 0xf) as usize;
	}
	return Some(addr);
}

fn atou(string: &str) -> Option<usize> {
	let slice = string.chars();
	let mut num: usize = 0;

	if string.starts_with("0x") {
		return hextou(string);
	}
	for i in slice {
		if !i.is_ascii_digit() {
			return None;
		}
		num *= 10;
		num += (i as u8 - b'0') as usize;
	}
	return Some(num);
}

fn		hexdump_parser(command: &Command) {
	let cmd = &command.command;

	let mut count: i32 = 0;
	let mut args: [usize; 2] = [0, 0];

	for iter in cmd.split(&[' ', '\t', '\0'][..]) {
		if iter.len() != 0 {
			count += 1;
		}
	}

	if count != 3 {
		kprintln!("Invalid number of argument");
		return ;
	}

	count = 0;
	for iter in cmd.split(&[' ', '\t', '\0'][..]) {
		if iter.len() != 0 {
			if count > 0 {
				match atou(iter) {
					Some(x)	=> args[count as usize - 1] = x,
					_		=> {kprintln!("Invalid arg"); return;}
				}
			}
			count += 1;
		}
	}
	hexdump!(args[0] as *const u8, args[1]);
}

#[derive(Debug, Clone)]
pub struct Command {
	pub command: String
}

impl Command {
	pub const fn new() -> Command {
		Command {
			command: String::new()
		}
	}

	fn append(&mut self, x: char) -> Result<(), allocator::AllocError> {
		self.command.try_push(x)
	}

	pub fn len(&self) -> usize {
		self.command.len()
	}

	pub fn clear(&mut self) {
		self.command.clear();
	}

	pub fn is_known(&self) -> Option<usize> {
		let mut j = 0;
		while j < KNOWN_CMD.len() {
			let cmd: &str = self.command.split(" ").nth(0)?;
			if Some(cmd) == Some(KNOWN_CMD[j]) {
				return Some(j);
			}
			j += 1;
		}
		return None;
	}

	pub fn handle(&mut self, charcode: char) {
		if charcode >= ' ' && charcode <= '~' {
			if self.append(charcode).is_err() {
				kprintln!("Can't handle longer command, clearing buffer");
				kprint!("$> ");
				self.clear();
			}
		} else if charcode == '\x08' {
			self.command.pop();
		} else if charcode == '\n' {
			match self.is_known() {
				Some(x) => COMMANDS[x](&self),
				_		=> {if self.command.len() != 0 {kprintln!("Unknown command. Type `help` to list available commands")}},
			}
			self.clear();
			kprint!("$> ");
		}
	}
}
