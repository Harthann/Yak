use crate::proc::process::{Pid, get_running_process};
use crate::proc::signal::{Signal, SignalType, get_signal_type};

pub extern "C" fn sys_kill(pid: Pid, signal: i32) -> i32 {
	if pid > 0 { /* Send to a specific process */
		unsafe {
			let sender_pid = (*get_running_process()).pid;
			let res = get_signal_type(signal);
			if !res.is_ok() {
				return res.unwrap() as i32
			}
			let res = Signal::send_to_pid(pid, sender_pid, res.unwrap(), 0);
			if !res.is_ok() {
				return -(res.unwrap() as i32)
			}
			return res.unwrap() as i32;
		}
	} else if pid == 0 { /* Send to every process in process group */
		todo!();
	} else if pid == -1 { /* Send to every process that has permission */
		todo!();
	} else { /* pid > -1: Send to every process in process group where is -pid */
		todo!();
	}
}
