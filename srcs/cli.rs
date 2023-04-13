//! Command line interface (term)

use core::arch::asm;

use crate::memory::allocator;
use crate::proc::process::{Pid, Process};
use crate::string::{String, ToString};
use crate::syscalls::exit::sys_waitpid;
use crate::syscalls::signal::sys_kill;
use crate::vec::Vec;
use crate::vga_buffer::{hexdump, screenclear};
use crate::{io, kprint, kprintln};

const NB_CMDS: usize = 14;
const MAX_CMD_LENGTH: usize = 250;

pub static COMMANDS: [fn(Vec<String>); NB_CMDS] = [
	reboot,
	halt,
	hexdump_parser,
	keymap,
	interrupt,
	clear,
	help,
	shutdown,
	jiffies,
	ps,
	uptime,
	date,
	play,
	kill
];
const KNOWN_CMD: [&str; NB_CMDS] = [
	"reboot", "halt", "hexdump", "keymap", "int", "clear", "help", "shutdown",
	"jiffies", "ps", "uptime", "date", "play", "kill"
];

fn kill(command: Vec<String>) {
	let mut wstatus: i32 = 0;
	let pid: Pid;

	if command.len() != 2 {
		kprintln!("Invalid argument.");
		kprintln!("Usage: kill [pid]");
		return;
	}

	if let Some(res) = atou(command[1].as_str()) {
		pid = res as Pid;
	} else {
		kprintln!("Invalid argument.");
		kprintln!("Usage: kill [pid]");
		return;
	}

	let res: i32 = sys_kill(pid, 9); // SIGKILL
	if res != 0 {
		kprintln!("[Error]: {}", res);
		return;
	}
	sys_waitpid(pid, &mut wstatus, 0);
}

fn reboot(_: Vec<String>) {
	io::outb(0x64, 0xfe);
}

fn play(command: Vec<String>) {
	let mut sound: &str = "Unknown";

	if command.len() == 2 {
		sound = command[1].as_str();
	}
	crate::kprintln!("sound: {}", sound);
	unsafe {
		crate::sound::play(sound);
	}
}

fn jiffies(_: Vec<String>) {
	unsafe {
		crate::kprintln!("Jiffies: {}", crate::pic::JIFFIES);
	}
}

fn uptime(_: Vec<String>) {
	unsafe {
		crate::pic::pit::TIME_ELAPSED =
			crate::pic::JIFFIES as f64 * crate::pic::pit::SYSTEM_FRACTION;
		let second = (crate::pic::pit::TIME_ELAPSED / 1000.0) as u64;
		let ms =
			((crate::pic::pit::TIME_ELAPSED - second as f64) * 1000.0) as u64;
		crate::kprintln!("Time elapsed since boot: {}s {}ms", second, ms);
	}
}

fn date(_: Vec<String>) {
	crate::kprintln!("{}", crate::cmos::get_time());
}

fn halt(_: Vec<String>) {
	unsafe {
		asm!("hlt");
	}
}

fn clear(_: Vec<String>) {
	screenclear!();
}

fn help(_: Vec<String>) {
	kprintln!("Available commands:");
	for i in KNOWN_CMD {
		kprintln!("    {}", i);
	}
}

fn shutdown(_: Vec<String>) {
	io::outb(0xf4, 0x10);
}

fn ps(_: Vec<String>) {
	unsafe { Process::print_all_process() };
}

fn hextou(string: &str) -> Option<usize> {
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

fn atou(string: &str) -> Option<usize> {
	if string.starts_with("0x") {
		return hextou(string);
	}
	string.parse::<usize>().ok()
}

fn hexdump_parser(command: Vec<String>) {
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

use crate::keyboard::{KEYMAP, KEYMAP_FR, KEYMAP_US};

fn keymap(command: Vec<String>) {
	if command.len() != 2 {
		kprintln!("Invalid number of arguments.");
		kprintln!("Usage: keymap {{us, fr}}");
		return;
	}

	if command[1] == "us" {
		unsafe { KEYMAP = &KEYMAP_US };
	} else if command[1] == "fr" {
		unsafe { KEYMAP = &KEYMAP_FR };
	} else {
		kprintln!("Invalid argument.");
		kprintln!("Usage: keymap {{us, fr}}");
	}
}

extern "C" {
	pub fn int(nb: u8);
}

fn interrupt(command: Vec<String>) {
	let arg: usize;

	if command.len() != 2 {
		kprintln!("Invalid number of arguments.");
		kprintln!("Usage: int [nb]");
		return;
	}

	if let Some(res) = atou(command[1].as_str()) {
		arg = res;
	} else {
		kprintln!("Invalid number of arguments.");
		kprintln!("Usage: hexdump [addr] [size]");
		return;
	}

	if arg > 255 {
		kprintln!("Invalid argument.");
		kprintln!("Usage: int [nb]");
		return;
	}
	unsafe { int(arg as u8) };
}

#[derive(Debug, Clone)]
pub struct Command {
	pub command: String
}

impl Command {
	pub const fn new() -> Command {
		Command { command: String::new() }
	}

	fn append(&mut self, x: char) -> Result<(), ()> {
		if self.command.len() < MAX_CMD_LENGTH {
			self.command.push(x);
			return Ok(());
		} else {
			Err(())
		}
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
			if Some(cmd) == Some(&KNOWN_CMD[j].to_string()) {
				return Some(j);
			}
			j += 1;
		}
		return None;
	}

	pub fn handle(&mut self, charcode: char) {
		if charcode == '\x08' {
			if self.command.len() != 0 {
				crate::vga_buffer::WRITER.lock().write_byte(0x08);
			}
			self.command.pop();
		} else if charcode >= ' ' && charcode <= '~' {
			crate::kprint!("{}", charcode);
			if self.append(charcode).is_err() {
				kprintln!("Can't handle longer command, clearing buffer");
				kprint!("$> ");
				self.clear();
			}
		} else if charcode == '\n' {
			crate::kprint!("{}", charcode);
			match self.is_known() {
				Some(x) => {
					let mut split: Vec<String> = Vec::new();
					let splited = self.command.split(&[' ', '\t', '\0'][..]);
					for arg in splited {
						split.push(arg.to_string());
					}
					COMMANDS[x](split);
				},
				_ => {
					if self.command.len() != 0 {
						kprintln!("Unknown command. Type `help` to list available commands");
					}
				},
			}
			self.clear();
			kprint!("$> ");
		}
	}
}
