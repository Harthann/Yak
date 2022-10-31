use crate::{print_fn, exec_fn};
use crate::proc::process::get_nb_process;
use crate::syscalls::exit::sys_waitpid;

use crate::{__WIFEXITED, __WEXITSTATUS};

pub fn simple_exec() -> usize {
	2
}

#[test_case]
fn new_process() {
	print_fn!();
	unsafe {
		assert_eq!(get_nb_process(), 1);
		let pid = exec_fn!(simple_exec as u32);
		assert_eq!(get_nb_process(), 2);
		let mut wstatus: i32 = 0;
		let ret = sys_waitpid(pid, &mut wstatus, 0);
		assert_eq!(ret, pid);
		assert_eq!(__WIFEXITED!(wstatus), true);
		assert_eq!(__WEXITSTATUS!(wstatus), 2);
	}
}
