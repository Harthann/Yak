use crate::proc::Id;
use crate::proc::process::{Process, MASTER_PROCESS};

#[derive(Copy, Clone, PartialEq)]
pub enum SignalType {
	SIGHUP		= 1,
	SIGINT		= 2,
	SIGQUIT		= 3,
	SIGILL		= 4,
	SIGTRAP		= 5,
	SIGABRT		= 6,
	SIGBUS		= 7,
	SIGFPE		= 8,
	SIGKILL		= 9,
	SIGUSR1		= 10,
	SIGSEGV		= 11,
	SIGUSR2		= 12,
	SIGPIPE		= 13,
	SIGALRM		= 14,
	SIGTERM		= 15,
	SIGSTKFLT	= 16,
	SIGCHLD		= 17,
	SIGCONT		= 18,
	SIGTSTOP	= 19,
	SIGTSTP		= 20,
	SIGTTIN		= 21,
	SIGTTOU		= 22,
	SIGURG		= 23,
	SIGXCPU		= 24,
	SIGXFSZ		= 25,
	SIGVTALRM	= 26,
	SIGPROF		= 27,
	SIGWINCH	= 28,
	SIGIO		= 29,
	SIGPWR		= 30,
	SIGSYS		= 31,
	SIGRTMIN	= 32
}

#[derive(Copy, Clone)]
pub struct Signal {
	pub sender: Id,
	pub sigtype: SignalType
}

impl Signal {
	pub const fn new(pid: Id, sigtype: SignalType) -> Self {
		Self {
			sender: pid,
			sigtype: sigtype
		}
	}

	pub fn send_to_pid(pid: Id, sender_pid: Id, sigtype: SignalType) {
		unsafe {
			let res = MASTER_PROCESS.search_from_pid(pid);
			if !res.is_ok() {
				todo!();
			}
			let process: &mut Process = res.unwrap();
			let signal = Self::new(sender_pid, sigtype);
			process.signals.push(signal);
		}
	}

	pub fn send_to_process(process: &mut Process, pid: Id, sigtype: SignalType) {
		let signal = Self::new(pid, sigtype);
		process.signals.push(signal);
	}
}
