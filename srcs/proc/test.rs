use crate::proc::process::Process;
use crate::syscalls::exit::{sys_exit, sys_waitpid};
use crate::syscalls::signal::{sys_kill, sys_signal};
use crate::syscalls::timer::{sys_getpid, sys_getppid};
use crate::{exec_fn, print_fn};

pub fn simple_exec() -> usize {
	2
}

#[test_case]
fn test_exec_fn_no_args() {
	print_fn!();
	unsafe {
		assert_eq!(Process::get_nb_process(), 1);
		let pid = exec_fn!(simple_exec);
		assert_eq!(Process::get_nb_process(), 2);
		let mut wstatus: i32 = 0;
		let ret = sys_waitpid(pid, &mut wstatus, 0);
		assert_eq!(ret, pid);
		assert_eq!(__WIFEXITED!(wstatus), true);
		assert_eq!(__WEXITSTATUS!(wstatus), 2);
	}
}

pub fn add(nb1: i32, nb2: i32) -> i32 {
	nb1 + nb2
}

#[test_case]
fn test_exec_fn_simple_args() {
	print_fn!();
	unsafe {
		assert_eq!(Process::get_nb_process(), 1);
		let pid = exec_fn!(add, 5, 4);
		assert_eq!(Process::get_nb_process(), 2);
		let mut wstatus: i32 = 0;
		let ret = sys_waitpid(pid, &mut wstatus, 0);
		assert_eq!(ret, pid);
		assert_eq!(__WIFEXITED!(wstatus), true);
		assert_eq!(__WEXITSTATUS!(wstatus), 9);
	}
}

pub fn add_diff_type(nb1: i32, nb2: u32, string: &str) -> i32 {
	string.len() as i32 + nb1 + nb2 as i32
}

#[test_case]
fn test_exec_fn_diff_args() {
	print_fn!();
	unsafe {
		assert_eq!(Process::get_nb_process(), 1);
		let pid = exec_fn!(add_diff_type, 8, 9, "salut");
		assert_eq!(Process::get_nb_process(), 2);
		let mut wstatus: i32 = 0;
		let ret = sys_waitpid(pid, &mut wstatus, 0);
		assert_eq!(ret, pid);
		assert_eq!(__WIFEXITED!(wstatus), true);
		assert_eq!(__WEXITSTATUS!(wstatus), 22);
	}
}

#[test_case]
fn test_simple_multiple_process() {
	print_fn!();
	unsafe {
		let mut pids: [i32; 3] = [0; 3];
		assert_eq!(Process::get_nb_process(), 1);
		pids[0] = exec_fn!(simple_exec);
		assert_eq!(Process::get_nb_process(), 2);
		pids[1] = exec_fn!(add, 1, 2);
		assert_eq!(Process::get_nb_process(), 3);
		pids[2] = exec_fn!(add, 1, 2);
		assert_eq!(Process::get_nb_process(), 4);
		let mut i = 0;
		while i < 3 {
			sys_waitpid(pids[i], core::ptr::null_mut(), 0);
			i += 1;
			assert_eq!(Process::get_nb_process(), 4 - i);
		}
	}
}

fn create_subprocess(nb: usize) {
	if nb > 0 {
		unsafe { exec_fn!(create_subprocess, nb - 1) };
		sys_waitpid(-1, core::ptr::null_mut(), 0);
	}
}

#[test_case]
fn test_subprocess() {
	print_fn!();
	unsafe {
		assert_eq!(Process::get_nb_process(), 1);
		let pid = exec_fn!(create_subprocess, 1);
		let res = sys_waitpid(pid, core::ptr::null_mut(), 0);
		assert_eq!(res, pid);
		assert_eq!(Process::get_nb_process(), 1);
	}
}

fn create_multiple_subprocess(nb: usize) {
	for i in 1..nb - 1 {
		unsafe { exec_fn!(create_multiple_subprocess, i) };
	}
	for _i in 1..nb - 1 {
		let res = sys_waitpid(-1, core::ptr::null_mut(), 0);
		assert!(res > 0);
	}
}

#[test_case]
fn test_multiple_subprocess() {
	print_fn!();
	unsafe {
		assert_eq!(Process::get_nb_process(), 1);
		let pid = exec_fn!(create_multiple_subprocess, 4);
		let res = sys_waitpid(pid, core::ptr::null_mut(), 0);
		assert_eq!(res, pid);
		assert_eq!(Process::get_nb_process(), 1);
	}
}

fn to_kill() {
	loop {}
}

#[test_case]
fn test_sigkill() {
	print_fn!();
	unsafe {
		assert_eq!(Process::get_nb_process(), 1);
		let pid = exec_fn!(sub_fn);
		let mut wstatus: i32 = 0;
		assert_eq!(Process::get_nb_process(), 2);
		let res: i32 = sys_kill(666, 0); // Check for pid presence
		assert_ne!(res, 0); // TODO: Check for errcode
		let res: i32 = sys_kill(pid, 0); // Check for pid presence
		assert_eq!(res, 0);
		let res: i32 = sys_kill(pid, 9); // SIGKILL
		assert_eq!(res, 0);
		sys_waitpid(pid, &mut wstatus, 0);
		assert_eq!(__WIFSIGNALED!(wstatus), true);
		assert_eq!(__WEXITSTATUS!(wstatus), 9);
		assert_eq!(Process::get_nb_process(), 1);
	}
}

fn handler(_sig_no: i32) {
	sys_exit(42);
}

fn sub_fn() {
	sys_signal(8, handler);
	loop {}
}

#[test_case]
fn test_simple_signal() {
	print_fn!();
	unsafe {
		assert_eq!(Process::get_nb_process(), 1);
		let pid = exec_fn!(sub_fn);
		let mut wstatus: i32 = 0;
		assert_eq!(Process::get_nb_process(), 2);
		let res: i32 = sys_kill(pid, 8);
		assert_eq!(res, 0);
		sys_waitpid(pid, &mut wstatus, 0);
		assert_eq!(__WIFEXITED!(wstatus), true);
		assert_eq!(__WEXITSTATUS!(wstatus), 42);
		assert_eq!(Process::get_nb_process(), 1);
	}
}

fn handler2(sig_no: i32) {
	assert_eq!(sig_no, 8);
}

fn sub_fn2() {
	sys_signal(8, handler2);
	loop {}
}

unsafe fn sub_test() -> i32 {
	let pid = exec_fn!(sub_fn2);
	let mut wstatus: i32 = 0;
	assert_eq!(Process::get_nb_process(), 3);
	sys_kill(pid, 8);
	sys_kill(pid, 8);
	sys_kill(pid, 8);
	let res: i32 = sys_kill(pid, 9);
	assert_eq!(res, 0);
	sys_waitpid(pid, &mut wstatus, 0);
	42
}

#[test_case]
fn test_signal_subprocess() {
	print_fn!();
	unsafe {
		assert_eq!(Process::get_nb_process(), 1);
		let pid = exec_fn!(sub_test);
		let mut wstatus: i32 = 0;
		sys_waitpid(pid, &mut wstatus, 0);
		assert_eq!(__WIFEXITED!(wstatus), true);
		assert_eq!(__WEXITSTATUS!(wstatus), 42);
		assert_eq!(Process::get_nb_process(), 1);
	}
}

fn subsub_test_pid(ppid: i32) -> i32 {
	let pid: i32 = sys_getpid();
	assert_eq!(sys_getppid(), ppid);
	assert_eq!(pid, ppid + 1);
	pid
}

unsafe fn sub_test_pid() {
	let child_pid: i32 = exec_fn!(subsub_test_pid, sys_getpid());
	let mut wstatus: i32 = 0;
	sys_waitpid(child_pid, &mut wstatus, 0);
	assert_eq!(__WIFEXITED!(wstatus), true);
	assert_eq!(__WEXITSTATUS!(wstatus), child_pid);
}

#[test_case]
fn test_pid() {
	print_fn!();
	unsafe {
		assert_eq!(sys_getpid(), 0);
		assert_eq!(sys_getppid(), -1);
		exec_fn!(sub_test_pid);
		sys_waitpid(-1, core::ptr::null_mut(), 0);
	}
}
