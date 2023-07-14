use crate::alloc::vec::Vec;
use crate::alloc::string::String;
use crate::kprintln;

use crate::proc::process::{Pid, Process};
use crate::cli::commands::hexdump::atou;
use crate::syscalls::signal::sys_kill;

pub fn pmap(command: Vec<String>) {
	let pid: Pid;

	if command.len() != 2 {
		kprintln!("Invalid argument.");
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

	// Send to a specific process
	let binding = match Process::search_from_pid(pid) {
		Ok(x) => x,
		Err(_) => return
	};
	let process = binding.lock();
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

pub fn kill(command: Vec<String>) {
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
}

pub fn ps(_: Vec<String>) {
	unsafe { Process::print_all_process() };
}


