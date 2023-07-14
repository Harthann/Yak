use crate::alloc::vec::Vec;
use crate::alloc::string::String;

use crate::kprintln;

pub fn valgrind(command: Vec<String>) {
	if command.len() < 2 {
		kprintln!("Invalid argument.");
		kprintln!("Usage: valgrind [command] [command args]");
		return;
	}

	// Save current heap state
	let heap_state = unsafe { crate::KTRACKER };

	let sub_command = &command[1..command.len()];
	let cmd_id = super::KNOWN_CMD.iter().position(|&x| x == sub_command[0]);
	match cmd_id {
		Some(index) => super::COMMANDS[index](sub_command.to_vec()),
		None => kprintln!("Command: [{}] not found", sub_command[0])
	}

	let mut current_state = unsafe { crate::KTRACKER };
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
