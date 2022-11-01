use crate::{print_fn, exec_fn};
use crate::proc::process::get_nb_process;
use crate::syscalls::exit::sys_waitpid;

use crate::{__WIFEXITED, __WEXITSTATUS};

pub fn simple_exec() -> usize {
	2
}

#[test_case]
fn test_exec_fn_no_args() {
	print_fn!();
	unsafe {
		assert_eq!(get_nb_process(), 1);
		let pid = exec_fn!(simple_exec);
		assert_eq!(get_nb_process(), 2);
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
		assert_eq!(get_nb_process(), 1);
		let pid = exec_fn!(add, 5, 4);
		assert_eq!(get_nb_process(), 2);
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
		assert_eq!(get_nb_process(), 1);
		let pid = exec_fn!(add_diff_type, 8, 9, "salut");
		assert_eq!(get_nb_process(), 2);
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
		assert_eq!(get_nb_process(), 1);
		pids[0] = exec_fn!(simple_exec);
		assert_eq!(get_nb_process(), 2);
		pids[1] = exec_fn!(add, 1, 2);
		assert_eq!(get_nb_process(), 3);
		pids[2] = exec_fn!(add, 1, 2);
		assert_eq!(get_nb_process(), 4);
		let mut i = 0;
		while i < 3 {
			sys_waitpid(pids[i], core::ptr::null_mut(), 0);
			i += 1;
			assert_eq!(get_nb_process(), 4 - i);
		}
	}
}

fn create_subprocess(nb: usize) {
	if nb > 0 {
		unsafe{exec_fn!(create_subprocess, nb - 1)};
		sys_waitpid(-1, core::ptr::null_mut(), 0);
	}
}

#[test_case]
fn test_subprocess() {
	print_fn!();
	unsafe {
		assert_eq!(get_nb_process(), 1);
		let pid = exec_fn!(create_subprocess, 1);
		let res = sys_waitpid(pid, core::ptr::null_mut(), 0);
		assert_eq!(res, pid);
		assert_eq!(get_nb_process(), 1);
	}
}

fn create_multiple_subprocess(nb: usize) {
	for i in 1..nb - 1 {
		unsafe{exec_fn!(create_multiple_subprocess, i)};
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
		assert_eq!(get_nb_process(), 1);
		let pid = exec_fn!(create_multiple_subprocess, 4);
		let res = sys_waitpid(pid, core::ptr::null_mut(), 0);
		assert_eq!(res, pid);
		assert_eq!(get_nb_process(), 1);
	}
}
