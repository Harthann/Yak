use crate::proc::process::Process;
use crate::proc::Id;

pub fn sys_getpid() -> Id {
	Process::get_running_process().pid
}

pub fn sys_getuid() -> Id {
	Process::get_running_process().owner
}

pub fn sys_getppid() -> Id {
	match &Process::get_running_process().parent {
		Some(process) => process.pid,
		None => -1
	}
}
