//! Command line interface (term)

use core::arch::asm;

use crate::proc::process::{Pid, Process};
use crate::string::{String, ToString};
use crate::syscalls::exit::sys_waitpid;
use crate::syscalls::signal::sys_kill;
use crate::vec::Vec;
use crate::vga_buffer::{hexdump, screenclear};
use crate::{io, kprint, kprintln};

const NB_CMDS: usize = 16;
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
	kill
];
const KNOWN_CMD: [&str; NB_CMDS] = [
	"reboot", "halt", "hexdump", "keymap", "int", "clear", "help", "shutdown",
	"jiffies", "ps", "uptime", "date", "play", "valgrind", "pmap", "kill"
];

pub fn command_entry(cmd_id: usize, ptr: *mut String, len: usize, cap: usize) {
	unsafe {
		let args: Vec<String> = Vec::from_raw_parts(ptr, len, cap);
		COMMANDS[cmd_id](args);
		crate::syscalls::exit::sys_exit(0);
	}
}

fn valgrind(command: Vec<String>) {
    if command.len() < 2 {
		kprintln!("Invalid argument.");
		kprintln!("Usage: valgrind [command] [command args]");
		return;
	}

    // Save current heap state
    let heap_state = unsafe {crate::KTRACKER};

    let sub_command = &command[1..command.len()];
    let cmd_id = KNOWN_CMD.iter().position(|&x| x == sub_command[0]);
    match cmd_id {
        Some(index) => COMMANDS[index](sub_command.to_vec()),
        None => kprintln!("Command: [{}] not found", sub_command[0])
    }

    let mut current_state = unsafe {crate::KTRACKER};
	current_state.allocation -= heap_state.allocation;
    current_state.allocated_bytes -= heap_state.allocated_bytes;
	current_state.freed -= heap_state.freed;
	current_state.freed_bytes -= heap_state.freed_bytes;
	crate::kprintln!("{}", current_state);
    if current_state.allocated_bytes < current_state.freed_bytes {
        crate::kprintln!(
            "Too much bytes freed: {}",
            current_state.freed_bytes - current_state.allocated_bytes
        );
    } else {
	    crate::kprintln!(
	    	"Leaks: {} bytes",
	    	current_state.allocated_bytes - current_state.freed_bytes
	    );
    }
}

fn pmap(command: Vec<String>) {
	let pid: Pid;

	if command.len() != 2 { kprintln!("Invalid argument.");
		kprintln!("Usage: pmap [pid]");
		return;
	}
	if let Some(res) = atou(command[1].as_str()) {
		pid = res as Pid;
	} else {
		kprintln!("Invalid argument.");
		kprintln!("Usage: pmap [pid]");
		return;
	}

	use crate::proc::process::MASTER_PROCESS;
	unsafe {
		crate::wrappers::_cli();
		// Send to a specific process
		let res = MASTER_PROCESS.search_from_pid(pid);
		if res.is_err() {
			crate::wrappers::_sti();
			return;
		}
		let process: &mut Process = res.unwrap();
		let mut used_size: usize = 0;
		crate::kprintln!("{}:", pid);
		crate::kprintln!("{}", process.heap);
		used_size += process.heap.size;
		crate::kprintln!("{}", process.stack);
		used_size += process.stack.size;
		crate::kprintln!("{}", process.kernel_stack);
		used_size += process.kernel_stack.size;
		for i in &process.mem_map {
			let guard = i.lock();
			crate::kprintln!("{}", *guard);
			used_size += guard.size;
		}
		crate::kprintln!(" total: {:#x}", used_size);
	}
}

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
	crate::sound::play(sound);
}

fn jiffies(_: Vec<String>) {
	crate::kprintln!("Jiffies: {}", crate::time::jiffies());
}

fn uptime(_: Vec<String>) {
	let time = crate::time::get_timestamp();
	crate::kprintln!(
		"Time elapsed since boot: {}s {}ms",
		time.second,
		time.millisecond
	);
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
	unsafe {
		crate::dprintln!("{}", crate::KTRACKER);
	}
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

use crate::interrupts::int::int;

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
					let mut split: Vec<String> = Vec::new();
					let splited = self.command.split(&[' ', '\t', '\0'][..]);
					for arg in splited {
						split.push(arg.to_string());
					}
					let (ptr, len, cap) = split.into_raw_parts();
					let pid = unsafe {
						crate::exec_fn!(command_entry, x, ptr, len, cap)
					};
					let mut status = 0;
					sys_waitpid(pid, &mut status, 0);
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
