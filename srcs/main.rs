use crate::proc::process::Pid;
use crate::string;
use crate::vga_buffer::color::Color;
use crate::{exec_fn, kprint, kprintln, change_color};

unsafe fn dumb_main(nb: usize) {
	crate::kprintln!("dumbmain{}!!!", nb);
	let mut pid: Pid = -1;
	if nb > 1 {
		pid = exec_fn!(dumb_main as u32, nb - 1);
	}
	let mut i = 0;
	while i < 2048 {
//		crate::kprintln!("dumb{}", nb);
		i += 1;
	}
	if nb > 1 {
		let mut status: i32 = 0;
		let test: i32 = sys_waitpid(pid, &mut status, 0);
		crate::kprintln!("exited process pid: {} - status: {}", test, status);
	}
	core::arch::asm!("mov ebx, 8
					mov eax, 1",
					"int 0x80"); /* test syscall exit */
}

unsafe fn dumb_main2(nb: usize, nb2: u64) {
	crate::kprintln!("not_dumbmain{} - {:#x?}!!!", nb, nb2);
	if nb > 1 {
		exec_fn!(dumb_main2 as u32, nb - 1, nb2);
	}
	let mut i = 0;
	while i < 2048 {
		crate::kprintln!("not_dumb{} - {:#x?}", nb, nb2);
		i += 1;
	}
	loop {}
}

pub fn test_task() {
	unsafe {
		exec_fn!(dumb_main as u32, 3);
		exec_fn!(dumb_main as u32, 2);
		exec_fn!(dumb_main as u32, 1);
	}

	let mut i = 0;
	while i < 3 {
		let mut status: i32 = 0;
		let test: i32 = sys_waitpid(-1, &mut status, 0);
		crate::kprintln!("exited process pid: {} - status: {}", test, status);
		i += 1;
	}
	/* TEST NOWHANG */
	let mut status: i32 = 0;
	let test: i32 = sys_waitpid(-1, &mut status, 0x01);
	crate::kprintln!("exited process pid: {} - status: {}", test, status);
//	loop {}
}

pub fn test_task2() {
	unsafe {
		exec_fn!(dumb_main2 as u32, 4, 0x123456789abcdef as u64);
	}
}

use crate::syscalls::exit::sys_waitpid;

#[no_mangle]
pub extern "C" fn kmain() -> ! {

	kprintln!("Hello World of {}!", 42);

	change_color!(Color::Red, Color::White);
	let workspace_msg = string::String::from("Press Ctrl-2 to navigate to the second workspace");
	kprintln!("{}", workspace_msg);
	change_color!(Color::White, Color::Black);

	kprint!("$> ");
	loop {
    }
}
