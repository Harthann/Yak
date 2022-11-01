use crate::proc::_exit;
use crate::proc::signal::{Signal, SignalType};
use crate::proc::process::{Process, Pid, get_running_process, get_signal_running_process};

use crate::errno::ErrNo;

const WNOHANG: u32 = 0x01;
const WUNTRACED: u32 = 0x02;

type Time = usize;

#[repr(C, packed)]
struct Timeval {
	tv_sec: Time, // Number of whole seconds of elapsed time
	tv_usec: usize // Number of microseconds of rest of elapsed time minus tv_sec
}

#[repr(C, packed)]
pub struct RUsage {
	ru_utime: Timeval, // Time spent executing user instructions
	ru_stime: Timeval, // Time spent in operating system code on behalf of processes
	ru_maxrss: usize, // The maximum resident set size used, in kilobytes
	ru_ixrss: usize, // Size of shared memory
	ru_idrss: usize, // Size of unshared memory
	ru_isrss: usize, // Size of unshared memory used fork stack space
	ru_minflt: usize, // Number of page requested
	ru_majflt: usize, // Number of page faults
	ru_nswap: usize, // Number of swap
	ru_inblock: usize, // Number of times read from disk
	ru_oublock: usize, // Number of times write to disk
	ru_msgsnd: usize, // Number of IPC messages sent
	ru_msgrcv: usize, // Number of IPC messages received
	ru_nsignals: usize, // Number of signals received
	ru_nvcsw: usize, // Number of times processes voluntarily invoked a context switch
	ru_nivcsw: usize // Number of times an involuntary context switch took place
}

pub extern "C" fn sys_wait4(pid: Pid, wstatus: *mut i32, options: u32, rusage: *mut RUsage) -> Pid {
	0
}

/* TODO: handle status (as flags) (in signal too) */
/* TODO: EINTR */
pub extern "C" fn sys_waitpid(pid: Pid, wstatus: *mut i32, options: u32) -> Pid {
	unsafe {
		/* TODO: set task status interruptible */
		loop {
			let res = get_signal_running_process(pid);
			if res.is_ok() {
				let signal: Signal = res.unwrap();
				if signal.sigtype == SignalType::SIGCHLD {
					let process_ptr = get_running_process();
					let res = (*process_ptr).search_from_pid(signal.sender);
					if res.is_ok() {
						let process: &mut Process = res.unwrap();
						process.remove();
					}
					if !wstatus.is_null() {
						*wstatus = signal.wstatus; // TODO
					}
				}
				return signal.sender;
			} else if res == Err(ErrNo::ESRCH) {
				return -(ErrNo::ECHILD as i32);
			}
			if options & WNOHANG != 0 {
				return 0;
			}
		}
	}
}

pub extern "C" fn sys_exit(status: i32) -> ! {
	unsafe {
		_exit(status);
	}
	/* Never goes there */
}

/* Macros to get status values */

#[macro_export]
macro_rules! __WEXITSTATUS {
	($status: expr) => (
		(($status & 0xff00) >> 8)
	);
}

#[macro_export]
macro_rules! __WTERMSIG {
	($status: expr) => (
		(($status) & 0x7f)
	);
}

#[macro_export]
macro_rules! __WSTOPSIG {
	($status: expr) => (
		$crate::__WEXITSTATUS!($status)
	);
}

#[macro_export]
macro_rules! __WIFEXITED {
	($status: expr) => (
		($crate::__WTERMSIG!($status) == 0)
	);
}

#[macro_export]
macro_rules! __WIFSIGNALED {
	($status: expr) => (
		((((($status & 0x7f) + 1) >> 1) > 0) & 0xff) // signed char
	);
}

#[macro_export]
macro_rules! __WIFSTOPPED {
	($status: expr) => (
		(($status & 0xff) == 0x7f)
	);
}

const __WCONTINUED: usize = 0xffff;
const __WCOREFLAG: usize = 0x80;

#[macro_export]
macro_rules! __WIFCONTINUED {
	($status: expr) => (
		($status == __W_CONTINUED)
	);
}

#[macro_export]
macro_rules! __WCOREDUMP {
	($status: expr) => (
		($status & __WCOREFLAG)
	);
}

/* Macros to set status values */

#[macro_export]
macro_rules! __W_EXITCODE {
	($ret: expr, $sig: expr) => (
		(($ret << 8) | $sig)
	);
}

#[macro_export]
macro_rules! __W_STOPCODE {
	($sig: expr) => (
		(($sig << 8) | 0x7f)
	);
}
