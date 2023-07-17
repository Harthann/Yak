use core::arch::asm;

use crate::cli::LOCK_CMD;
use crate::proc::signal::SignalType;
use crate::string::{String, ToString};
use crate::syscalls::exit::sys_waitpid;
use crate::syscalls::signal::sys_kill;
use crate::syscalls::timer::sys_getppid;
use crate::vec::Vec;
use crate::vga_buffer::screenclear;
use crate::{io, kprint, kprintln};

// Commands modules
mod debugfs;
mod hexdump;
mod process;
mod time;
mod valgrind;

use debugfs::debugfs;
use hexdump::hexdump_parser;
use process::{kill, pmap, ps};
use time::{date, jiffies, uptime};
use valgrind::valgrind;

const NB_CMDS: usize = 17;
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
	valgrind,
	pmap,
	kill,
	debugfs
];

const KNOWN_CMD: [&str; NB_CMDS] = [
	"reboot", "halt", "hexdump", "keymap", "int", "clear", "help", "shutdown",
	"jiffies", "ps", "uptime", "date", "play", "valgrind", "pmap", "kill",
	"debugfs"
];

fn reboot(_: Vec<String>) {
	io::outb(0x64, 0xfe);
}
fn play(command: Vec<String>) {
	let mut sound: &str = "Unknown";

	if command.len() == 2 {
		sound = command[1].as_str();
	}
	crate::kprintln!("sound: {}", sound);
	crate::sound::play(sound);
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
	unsafe {
		crate::dprintln!("{}", crate::KTRACKER);
	}
	io::outb(0xf4, 0x10);
}

fn halt(_: Vec<String>) {
	unsafe {
		asm!("hlt");
	}
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

use crate::interrupts::int::int;

fn interrupt(command: Vec<String>) {
	let arg: usize;

	if command.len() != 2 {
		kprintln!("Invalid number of arguments.");
		kprintln!("Usage: int [nb]");
		return;
	}

	if let Some(res) = hexdump::atou(command[1].as_str()) {
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

pub fn command_entry(cmd_id: usize, ptr: *mut String, len: usize, cap: usize) {
	unsafe {
		let args: Vec<String> = Vec::from_raw_parts(ptr, len, cap);
		// notify parent that vector has been copied
		sys_kill(sys_getppid(), SignalType::SIGHUP as i32);
		COMMANDS[cmd_id](args);
	}
}

#[derive(Debug, Clone)]
pub struct Command {
	pub command: String,
	pub index:   usize
}

impl Command {
	pub const fn new() -> Command {
		Command { command: String::new(), index: 0 }
	}

	fn insert(&mut self, x: char) -> Result<(), ()> {
		if self.command.len() < MAX_CMD_LENGTH {
			self.command.insert(self.index, x);
			self.index += 1;
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
		self.index = 0;
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
			if self.command.len() != 0 && self.index != 0 {
				self.command.remove(self.index - 1);
				let tmp: &str =
					&self.command[self.index - 1..self.command.len()];
				self.index -= 1;
				crate::kprint!(
					"{delbyte}{string} {delbyte}",
					string = tmp,
					delbyte = '\x08'
				);
				crate::vga_buffer::WRITER
					.lock()
					.move_cursor(-(tmp.len() as i32));
			}
		} else if charcode >= ' ' && charcode <= '~' {
			if self.insert(charcode).is_err() {
				kprintln!("Can't handle longer command, clearing buffer");
				kprint!("$> ");
				self.clear();
			}
			let tmp: &str = &self.command[self.index - 1..self.command.len()];
			crate::kprint!("{}", tmp);
			crate::vga_buffer::WRITER
				.lock()
				.move_cursor(-(tmp.len() as i32) + 1);
		} else if charcode == '\n' {
			crate::kprint!("{}", charcode);
			match self.is_known() {
				Some(x) => {
					unsafe { LOCK_CMD = true };
					let mut background: bool = false;
					let mut split: Vec<String> = Vec::new();
					let splited = self.command.split(&[' ', '\t', '\0'][..]);
					for arg in splited {
						split.push(arg.to_string());
					}
					if split.last().unwrap() == "&" {
						split.pop();
						background = true;
					}
					let (ptr, len, cap) = split.clone().into_raw_parts();
					unsafe {
						let pid = crate::exec_fn_name!(
							split[0],
							command_entry,
							x,
							ptr,
							len,
							cap
						);
						loop {
							if LOCK_CMD == false {
								break;
							}
							crate::time::sleep(1);
						}
						if background == false {
							let mut wstatus: i32 = 0;
							sys_waitpid(pid, &mut wstatus, 0);
						}
					}
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
