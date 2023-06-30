use core::arch::global_asm;

use crate::print_fn;
use crate::proc::process::Process;
use crate::syscalls::exit::{sys_waitpid, __WEXITSTATUS};
use crate::syscalls::signal::sys_kill;

global_asm!(
	r#"
.globl userfunc_1
.globl end_userfunc_1
userfunc_1:
	mov ebx, 42
	mov eax, 1
	int 0x80
end_userfunc_1:
"#
);

extern "C" {
	fn userfunc_1();
	fn end_userfunc_1();
}

#[test_case]
fn simple_test_userspace() {
	print_fn!();
	unsafe {
		let mut status: i32 = 0;
		let pid = crate::exec_fn_userspace!(
			userfunc_1,
			end_userfunc_1 as usize - userfunc_1 as usize
		);
		let ret = crate::syscalls::exit::sys_waitpid(pid, &mut status, 0);
		assert_eq!(ret, pid);
		assert_eq!(__WIFEXITED!(status), true);
		assert_eq!(__WEXITSTATUS!(status), 42);
	}
}

global_asm!(
	r#"
.globl userfunc_2
.globl end_userfunc_2
userfunc_2:
	jmp userfunc_2
end_userfunc_2:
"#
);

extern "C" {
	fn userfunc_2();
	fn end_userfunc_2();
}

#[test_case]
fn test_kill_userspace() {
	print_fn!();
	unsafe {
		let mut status: i32 = 0;
		let pid = crate::exec_fn_userspace!(
			userfunc_2,
			end_userfunc_2 as usize - userfunc_2 as usize
		);
		assert_eq!(Process::get_nb_process(), 2);
		let res: i32 = sys_kill(pid, 9);
		assert_eq!(res, 0);
		let ret = sys_waitpid(pid, &mut status, 0);
		assert_eq!(ret, pid);
		assert_eq!(__WIFSIGNALED!(status), true);
		assert_eq!(__WEXITSTATUS!(status), 9);
		assert_eq!(Process::get_nb_process(), 1);
	}
}

global_asm!(
	r#"
.globl userfunc_3
.globl end_userfunc_3
userfunc_3:
	mov eax, 64
	int 0x80
	mov ebx, eax
	mov eax, 1
	int 0x80
end_userfunc_3:
"#
);

extern "C" {
	fn userfunc_3();
	fn end_userfunc_3();
}

#[test_case]
fn test_getppid_userspace() {
	print_fn!();
	unsafe {
		let mut status: i32 = 0;
		let pid = crate::exec_fn_userspace!(
			userfunc_3,
			end_userfunc_3 as usize - userfunc_3 as usize
		);
		let ret = crate::syscalls::exit::sys_waitpid(pid, &mut status, 0);
		assert_eq!(ret, pid);
		assert_eq!(__WIFEXITED!(status), true);
		assert_eq!(__WEXITSTATUS!(status), 0);
	}
}

global_asm!(
	r#"
.globl userfunc_4
.globl end_userfunc_4
userfunc_4:
	mov eax, 2
	int 0x80
	mov ebx, 42
	mov eax, 1
	int 0x80
end_userfunc_4:
"#
);

extern "C" {
	fn userfunc_4();
	fn end_userfunc_4();
}

#[test_case]
fn test_fork_userspace() {
	print_fn!();
	unsafe {
		let mut status: i32 = 0;
		let pid = crate::exec_fn_userspace!(
			userfunc_4,
			end_userfunc_4 as usize - userfunc_4 as usize
		);
		let ret = crate::syscalls::exit::sys_waitpid(pid, &mut status, 0);
		assert_eq!(ret, pid);
		assert_eq!(__WIFEXITED!(status), true);
		assert_eq!(__WEXITSTATUS!(status), 42);
		let ret = crate::syscalls::exit::sys_waitpid(pid + 1, &mut status, 0);
		assert_eq!(ret, pid + 1);
		assert_eq!(__WIFEXITED!(status), true);
		assert_eq!(__WEXITSTATUS!(status), 42);
	}
}

global_asm!(
	r#"
.globl userfunc_5
.globl end_userfunc_5
userfunc_5:
	mov eax, 2 // fork
	int 0x80
	cmp eax, 0
	jne .wait_child_5

	mov ebx, 42
	mov eax, 1
	int 0x80

	.wait_child_5:
	mov edx, 0
	mov ecx, 0
	mov ebx, eax
	mov eax, 7 // waitpid
	int 0x80
	mov ebx, eax // exit
	mov eax, 1
	int 0x80
end_userfunc_5:
"#
);

extern "C" {
	fn userfunc_5();
	fn end_userfunc_5();
}

#[test_case]
fn test_fork2_userspace() {
	print_fn!();
	unsafe {
		let mut status: i32 = 0;
		let pid = crate::exec_fn_userspace!(
			userfunc_5,
			end_userfunc_5 as usize - userfunc_5 as usize
		);
		let ret = crate::syscalls::exit::sys_waitpid(pid, &mut status, 0);
		assert_eq!(ret, pid);
		assert_eq!(__WIFEXITED!(status), true);
		assert_eq!(__WEXITSTATUS!(status), pid + 1);
	}
}
