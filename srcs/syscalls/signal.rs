use crate::proc::process::{Pid, Process};
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
	_cli();
	{
		let binding = Process::get_running_process();
		let mut process = binding.lock();
		let handlers: &mut Vec<SignalHandler> = &mut process.signal_handlers;
		for i in 0..handlers.len() {
			if handlers[i].signal == signal {
				handlers.remove(i);
				break;
			}
		}
		handlers.push(SignalHandler { signal: signal, handler: handler });
	}
	_sti();
	handler
}

/// Returns 0 on success, otherwise, returns a negative value corresponding to errno
pub fn sys_kill(pid: Pid, signal: i32) -> i32 {
	_cli();
	if pid > 0 {
		// Send to a specific process
		unsafe {
			let res = Process::search_from_pid(pid);
			if res.is_err() {
				_sti();
				return -(res.err().unwrap() as i32);
			}
			if signal == 0 {
				_sti();
				return 0; // kill check for pid presence if signal is 0
			}
			let sender_pid = Process::get_running_process().lock().pid;
			let res = get_signal_type(signal);
			if res.is_err() {
				_sti();
				return -(res.err().unwrap() as i32);
			}
			let signal_type = res.unwrap();
			if signal_type == SignalType::SIGKILL {
				Process::zombify(pid, __W_STOPCODE!(signal_type as i32));
				Task::remove_task_from_process(pid);
				_sti();
				return 0;
			} else {
				let res = Signal::send_to_pid(pid, sender_pid, signal_type, 0);
				if res.is_err() {
					_sti();
					return -(res.err().unwrap() as i32);
				}
				_sti();
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
