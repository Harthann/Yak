use crate::proc::process::Process;
use crate::proc::Id;

pub fn sys_getpid() -> Id {
	Process::get_running_process().pid
}

pub fn sys_getuid() -> Id {
	Process::get_running_process().owner
}

pub fn sys_getppid() -> Id {
	unsafe {
		let ptr: *mut Process = Process::get_running_process().parent;
		if ptr.is_null() {
			-1
		} else {
			(*ptr).pid
		}
	}
}
