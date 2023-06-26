use crate::proc::process::Process;
use crate::proc::Id;

pub fn sys_getpid() -> Id {
	Process::get_running_process().lock().pid
}

pub fn sys_getuid() -> Id {
	Process::get_running_process().lock().owner
}

pub fn sys_getppid() -> Id {
	let binding = Process::get_running_process();
	let process = binding.lock();
	match &process.parent {
		Some(parent) => parent.lock().pid,
		None => -1
	}
}
