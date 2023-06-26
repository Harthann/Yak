use crate::errno::ErrNo;
use crate::proc::process::Process;
use crate::proc::Id;

pub type SigHandlerFn = fn(i32);

#[derive(Clone)]
pub struct SignalHandler {
	pub signal:  i32,
	pub handler: SigHandlerFn
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum SignalType {
	SIGHUP    = 1,
	SIGINT    = 2,
	SIGQUIT   = 3,
	SIGILL    = 4,
	SIGTRAP   = 5,
	SIGABRT   = 6,
	SIGBUS    = 7,
	SIGFPE    = 8,
	SIGKILL   = 9,
	SIGUSR1   = 10,
	SIGSEGV   = 11,
	SIGUSR2   = 12,
	SIGPIPE   = 13,
	SIGALRM   = 14,
	SIGTERM   = 15,
	SIGSTKFLT = 16,
	SIGCHLD   = 17,
	SIGCONT   = 18,
	SIGTSTOP  = 19,
	SIGTSTP   = 20,
	SIGTTIN   = 21,
	SIGTTOU   = 22,
	SIGURG    = 23,
	SIGXCPU   = 24,
	SIGXFSZ   = 25,
	SIGVTALRM = 26,
	SIGPROF   = 27,
	SIGWINCH  = 28,
	SIGIO     = 29,
	SIGPWR    = 30,
	SIGSYS    = 31,
	SIGRTMIN  = 32
}

pub fn get_signal_type(nb: i32) -> Result<SignalType, ErrNo> {
	match nb {
		_ if nb == SignalType::SIGHUP as i32 => Ok(SignalType::SIGHUP),
		_ if nb == SignalType::SIGINT as i32 => Ok(SignalType::SIGINT),
		_ if nb == SignalType::SIGQUIT as i32 => Ok(SignalType::SIGQUIT),
		_ if nb == SignalType::SIGILL as i32 => Ok(SignalType::SIGILL),
		_ if nb == SignalType::SIGTRAP as i32 => Ok(SignalType::SIGTRAP),
		_ if nb == SignalType::SIGABRT as i32 => Ok(SignalType::SIGABRT),
		_ if nb == SignalType::SIGBUS as i32 => Ok(SignalType::SIGBUS),
		_ if nb == SignalType::SIGFPE as i32 => Ok(SignalType::SIGFPE),
		_ if nb == SignalType::SIGKILL as i32 => Ok(SignalType::SIGKILL),
		_ if nb == SignalType::SIGUSR1 as i32 => Ok(SignalType::SIGUSR1),
		_ if nb == SignalType::SIGSEGV as i32 => Ok(SignalType::SIGSEGV),
		_ if nb == SignalType::SIGUSR2 as i32 => Ok(SignalType::SIGUSR2),
		_ if nb == SignalType::SIGPIPE as i32 => Ok(SignalType::SIGPIPE),
		_ if nb == SignalType::SIGALRM as i32 => Ok(SignalType::SIGALRM),
		_ if nb == SignalType::SIGTERM as i32 => Ok(SignalType::SIGTERM),
		_ if nb == SignalType::SIGSTKFLT as i32 => Ok(SignalType::SIGSTKFLT),
		_ if nb == SignalType::SIGCHLD as i32 => Ok(SignalType::SIGCHLD),
		_ if nb == SignalType::SIGCONT as i32 => Ok(SignalType::SIGCONT),
		_ if nb == SignalType::SIGTSTOP as i32 => Ok(SignalType::SIGTSTOP),
		_ if nb == SignalType::SIGTSTP as i32 => Ok(SignalType::SIGTSTP),
		_ if nb == SignalType::SIGTTIN as i32 => Ok(SignalType::SIGTTIN),
		_ if nb == SignalType::SIGTTOU as i32 => Ok(SignalType::SIGTTOU),
		_ if nb == SignalType::SIGURG as i32 => Ok(SignalType::SIGURG),
		_ if nb == SignalType::SIGXCPU as i32 => Ok(SignalType::SIGXCPU),
		_ if nb == SignalType::SIGXFSZ as i32 => Ok(SignalType::SIGXFSZ),
		_ if nb == SignalType::SIGVTALRM as i32 => Ok(SignalType::SIGVTALRM),
		_ if nb == SignalType::SIGPROF as i32 => Ok(SignalType::SIGPROF),
		_ if nb == SignalType::SIGWINCH as i32 => Ok(SignalType::SIGWINCH),
		_ if nb == SignalType::SIGIO as i32 => Ok(SignalType::SIGIO),
		_ if nb == SignalType::SIGPWR as i32 => Ok(SignalType::SIGPWR),
		_ if nb == SignalType::SIGSYS as i32 => Ok(SignalType::SIGSYS),
		_ if nb == SignalType::SIGRTMIN as i32 => Ok(SignalType::SIGRTMIN),
		_ => Err(ErrNo::EINVAL)
	}
}

#[derive(Copy, Clone, PartialEq)]
pub struct Signal {
	pub sender:  Id,
	pub sigtype: SignalType,
	pub wstatus: i32
}

impl Signal {
	pub const fn new(pid: Id, sigtype: SignalType, wstatus: i32) -> Self {
		Self { sender: pid, sigtype: sigtype, wstatus: wstatus }
	}

	pub fn send_to_pid(
		pid: Id,
		sender_pid: Id,
		sigtype: SignalType,
		wstatus: i32
	) -> Result<Id, ErrNo> {
		let binding = Process::search_from_pid(pid)?;
		let mut process = binding.lock();
		Self::send_to_process(&mut *process, sender_pid, sigtype, wstatus);
		Ok(pid)
	}

	pub fn send_to_process(
		process: &mut Process,
		pid: Id,
		sigtype: SignalType,
		wstatus: i32
	) {
		let signal = Self::new(pid, sigtype, wstatus);
		process.signals.push(signal);
	}
}
