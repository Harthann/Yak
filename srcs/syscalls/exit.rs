use crate::proc::process::{Pid, Process};
use crate::proc::signal::{Signal, SignalType};
use crate::proc::task::{Task, TaskStatus};

use crate::wrappers::{_cli, _sti, cli, cli_count, hlt, sti};

use crate::errno::ErrNo;
use crate::KSTACK_ADDR;

const WNOHANG: u32 = 0x01;
const WUNTRACED: u32 = 0x02;

type Time = usize;

#[repr(C, packed)]
struct Timeval {
	tv_sec:  Time,  // Number of whole seconds of elapsed time
	tv_usec: usize  /* Number of microseconds of rest of elapsed time minus tv_sec */
}

#[repr(C, packed)]
pub struct RUsage {
	ru_utime:    Timeval, // Time spent executing user instructions
	ru_stime:    Timeval, /* Time spent in operating system code on behalf of processes */
	ru_maxrss:   usize,   // The maximum resident set size used, in kilobytes
	ru_ixrss:    usize,   // Size of shared memory
	ru_idrss:    usize,   // Size of unshared memory
	ru_isrss:    usize,   // Size of unshared memory used fork stack space
	ru_minflt:   usize,   // Number of page requested
	ru_majflt:   usize,   // Number of page faults
	ru_nswap:    usize,   // Number of swap
	ru_inblock:  usize,   // Number of times read from disk
	ru_oublock:  usize,   // Number of times write to disk
	ru_msgsnd:   usize,   // Number of IPC messages sent
	ru_msgrcv:   usize,   // Number of IPC messages received
	ru_nsignals: usize,   // Number of signals received
	ru_nvcsw:    usize, /* Number of times processes voluntarily invoked a context switch */
	ru_nivcsw:   usize /* Number of times an involuntary context switch took place */
}

extern "C" {
	pub fn next_task();
}

pub fn sys_wait4(
	_pid: Pid,
	_wstatus: *mut i32,
	_options: u32,
	_rusage: *mut RUsage
) -> Pid {
	0
}

// TODO: EINTR
// + Make a func to get wstatus adress in userspace to kernel addr
pub fn sys_waitpid(pid: Pid, wstatus: *mut i32, options: u32) -> Pid {
	unsafe {
		_cli();
		loop {
			let res =
				Process::get_signal_running_process(pid, SignalType::SIGCHLD);
			if res.is_ok() {
				let signal: Signal = res.unwrap();
				crate::kprintln!("signal: {:?}", signal);
				let parent_process: &mut Process =
					Process::get_running_process();
				let res = parent_process.search_from_pid(signal.sender);
				if res.is_ok() {
					let process: &mut Process = res.unwrap();
					process.remove();
				}
				if !wstatus.is_null() {
					*wstatus = signal.wstatus; // TODO
				}
				crate::kprintln!("signal.sender: {}", signal.sender);
				_sti();
				return signal.sender;
			} else if res == Err(ErrNo::ESRCH) {
				_sti();
				return -(ErrNo::ECHILD as i32);
			}
			if options & WNOHANG != 0 {
				_sti();
				return 0;
			} else {
				crate::kprintln!("pause");
				let task: &mut Task = Task::get_running_task();
				task.state = TaskStatus::Interruptible;
				let save = cli_count;
				cli_count = 0;
				sti!();
				hlt!(); // wait for scheduler
				cli!(); // unblocked here
				cli_count = save;
			}
		}
	}
}

pub fn sys_exit(status: i32) -> ! {
	unsafe {
		core::arch::asm!(
		"mov eax, {status}",
		"mov esp, {kstack}",
		"push eax",
		"call _exit",
		status = in(reg) status,
		kstack = const KSTACK_ADDR);
		// Never goes there
		loop {}
	}
}

// Macros to get status values

macro_rules! __WEXITSTATUS {
	($status: expr) => {
		(($status & 0xff00) >> 8)
	};
}

macro_rules! __WTERMSIG {
	($status: expr) => {
		(($status) & 0x7f)
	};
}

macro_rules! __WSTOPSIG {
	($status: expr) => {
		$crate::syscalls::exit::__WEXITSTATUS!($status)
	};
}

macro_rules! __WIFEXITED {
	($status: expr) => {
		($crate::syscalls::exit::__WTERMSIG!($status) == 0)
	};
}

macro_rules! __WIFSIGNALED {
	($status: expr) => {
		(((($status & 0x7f) + 1) >> 1) > 0)
	};
}

#[allow(unused)]
macro_rules! __WIFSTOPPED {
	($status: expr) => {
		(($status & 0xff) == 0x7f)
	};
}

const __WCONTINUED: usize = 0xffff;
const __WCOREFLAG: usize = 0x80;

macro_rules! __WIFCONTINUED {
	($status: expr) => {
		($status == __W_CONTINUED)
	};
}

macro_rules! __WCOREDUMP {
	($status: expr) => {
		($status & __WCOREFLAG)
	};
}

// Macros to set status values

macro_rules! __W_EXITCODE {
	($ret: expr, $sig: expr) => {
		(($ret << 8) | $sig)
	};
}

macro_rules! __W_STOPCODE {
	($sig: expr) => {
		(($sig << 8) | 0x7f)
	};
}

#[allow(unused)]
pub(crate) use {
	__WCOREDUMP,
	__WEXITSTATUS,
	__WIFCONTINUED,
	__WIFEXITED,
	__WIFSIGNALED,
	__WIFSTOPPED,
	__WSTOPSIG,
	__WTERMSIG,
	__W_EXITCODE,
	__W_STOPCODE
};
