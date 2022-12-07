use crate::proc::Id;
use crate::proc::process::Process;

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
