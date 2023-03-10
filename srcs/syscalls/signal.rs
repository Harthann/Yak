use crate::proc::process::{get_running_process, Pid};
use crate::proc::signal::{get_signal_type, Signal};

pub extern "C" fn sys_kill(pid: Pid, signal: i32) -> i32 {
	if pid > 0 {
		unsafe {
			let sender_pid = (*get_running_process()).pid;
			let res = get_signal_type(signal);
			if !res.is_ok() {
				todo!();
			}
			Signal::send_to_pid(pid, sender_pid, res.unwrap(), 0);
		}
	} else if pid == 0 {
		todo!();
	} else if pid == -1 {
		todo!();
	} else {
		todo!();
	}
	0
}
