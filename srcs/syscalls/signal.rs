use crate::proc::process::{Pid, Process, MASTER_PROCESS};
use crate::proc::signal::{
	get_signal_type,
	SigHandlerFn,
	Signal,
	SignalHandler,
	SignalType
};
use crate::proc::task::Task;
use crate::wrappers::{_cli, _sti};

use crate::syscalls::exit::__W_STOPCODE;

use crate::vec::Vec;

pub fn sys_signal(signal: i32, handler: SigHandlerFn) -> SigHandlerFn {
	// TODO: check signal validity
	// TODO: Use map/hashmap instead
	let handlers: &mut Vec<SignalHandler> =
		&mut Process::get_running_process().signal_handlers;
	for i in 0..handlers.len() {
		if handlers[i].signal == signal {
			handlers.remove(i);
			break;
		}
	}
	handlers.push(SignalHandler { signal: signal, handler: handler });
	handler
}

/// Returns 0 on success, otherwise, returns a negative value corresponding to errno
pub fn sys_kill(pid: Pid, signal: i32) -> i32 {
	if pid > 0 {
		// Send to a specific process
		unsafe {
			let res = MASTER_PROCESS.search_from_pid(pid);
			if res.is_err() {
				return -(res.err().unwrap() as i32);
			}
			let process: &mut Process = res.unwrap();
			if signal == 0 {
				return 0; // kill check for pid presence if signal is 0
			}
			let sender_pid = Process::get_running_process().pid;
			let res = get_signal_type(signal);
			if res.is_err() {
				return -(res.err().unwrap() as i32);
			}
			let signal_type = res.unwrap();
			if signal_type == SignalType::SIGKILL {
				_cli();
				Task::remove_task_from_process(process);
				process.zombify(__W_STOPCODE!(signal_type as i32));
				_sti();
				return 0;
			} else {
				let res = Signal::send_to_pid(pid, sender_pid, signal_type, 0);
				if res.is_err() {
					return -(res.err().unwrap() as i32);
				}
				return 0;
			}
		}
	} else if pid == 0 {
		// Send to every process in process group
		todo!();
	} else if pid == -1 {
		// Send to every process that has permission
		todo!();
	} else {
		// pid > -1: Send to every process in process group where is -pid
		todo!();
	}
}
