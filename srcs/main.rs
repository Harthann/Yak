#![feature(const_mut_refs)]
#![feature(naked_functions)]
#![feature(fmt_internals)]
#![feature(specialization)]
#![feature(nonnull_slice_from_raw_parts)]
#![feature(rustc_attrs)]
#![feature(box_syntax)]
#![feature(ptr_internals)]
#![feature(fundamental)]
#![feature(lang_items)]
#![feature(c_variadic)]
#![no_std]
#![allow(dead_code)]
#![allow(incomplete_features)]
#![no_main]

/*  Custom test framework  */
#![feature(custom_test_frameworks)]
#![test_runner(crate::test::test_runner)]
#![reexport_test_harness_main = "test_main"]

#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn eh_personality() {}

const GLOBAL_ALIGN: usize = 8;

/* Allocation tracking */
pub struct Tracker {
	allocation:			usize,
	allocated_bytes:	usize,
	freed:				usize,
	freed_bytes:		usize
}

impl Tracker {
	pub const fn new() -> Self {
		Self {
			allocation: 0,
			allocated_bytes: 0,
			freed: 0,
			freed_bytes: 0
		}
	}
}

static mut TRACKER: Tracker = Tracker::new();
static mut KTRACKER: Tracker = Tracker::new();

pub fn memory_state() {
	unsafe {
		kprintln!("\nAllocation: {} for {} bytes", KTRACKER.allocation, KTRACKER.allocated_bytes);
		kprintln!("Free:       {} for {} bytes", KTRACKER.freed, KTRACKER.freed_bytes);
	}
}

/*  Modules import  */
mod cli;
mod gdt;
mod keyboard;
mod memory;
mod multiboot;
mod vec;
mod string;
mod interrupts;
mod syscalls;
mod io;
mod vga_buffer;
mod pic;
mod proc;
mod user;
mod wrappers;
mod utils;
mod errno;

#[cfg(test)]
mod test;

/*  Modules used function and variable  */
use memory::paging::{init_paging, page_directory};
use memory::allocator::linked_list::LinkedListAllocator;
use vga_buffer::color::Color;
use cli::Command;
use pic::setup_pic8259;

#[global_allocator]
static mut ALLOCATOR: LinkedListAllocator = LinkedListAllocator::new();
static mut KALLOCATOR: LinkedListAllocator = LinkedListAllocator::new();

/*  Code from boot section  */
#[allow(dead_code)]
extern "C" {
	fn stack_bottom();
	fn stack_top();
	fn heap();
}

use crate::memory::{init_heap, init_stack, VirtAddr};
use crate::memory::paging::PAGE_WRITABLE;

use crate::interrupts::init_idt;

use proc::task::{init_tasking};

use crate::gdt::{KERNEL_BASE, gdt_desc, update_gdtr};
//use crate::memory::paging::{alloc_pages_at_addr, PAGE_USER};
pub use pic::handlers::JIFFIES;

use crate::memory::MemoryZone;

pub static mut KSTACK: MemoryZone = MemoryZone::new();
pub static mut KHEAP: MemoryZone = MemoryZone::new();

/*  Kernel initialisation   */
#[no_mangle]
pub extern "C" fn kinit() {
	unsafe{crate::cli!()};
//	multiboot::read_tags();
	/* Init paging and remove identity paging */
	init_paging();
	/* Update gdtr with higher half kernel gdt addr */
	unsafe {
		update_gdtr();
		reload_gdt!();
		init_idt();
	}

	/* HEAP KERNEL */
	let kstack_addr: VirtAddr = 0xffbfffff; /* stack kernel */
	unsafe {
		KSTACK = init_stack(kstack_addr, 2 * 0x1000, PAGE_WRITABLE, false);
		KHEAP = init_heap(heap as u32, 100 * 0x1000, PAGE_WRITABLE, true, &mut KALLOCATOR);
	}

	gdt::tss::init_tss(kstack_addr);
	reload_tss!();

	init_tasking();

	/* init tracker after init first process */
	unsafe {
		KTRACKER = Tracker::new();
		TRACKER = Tracker::new();
	}

	setup_pic8259();
	/* Setting up frequency divider to modulate IRQ0 rate, low value tends to get really slow (too much task switching */
	pic::set_pit(pic::pit::CHANNEL_0, pic::pit::ACC_LOBHIB, pic::pit::MODE_2, 0x0fff);

	/* Reserve some spaces to push things before main */
	unsafe{core::arch::asm!("mov esp, {}", in(reg) kstack_addr - 256)};
	unsafe{crate::sti!()};

	/*	Function to test and enter usermode */
//	user::test_user_page();

	#[cfg(test)]
	test_main();

	#[cfg(not(test))]
	kmain();
}

use crate::proc::process::Pid;

use crate::syscalls::exit::sys_waitpid;
use crate::syscalls::signal::{sys_signal, sys_kill};

#[no_mangle]
extern "C" fn handler(nb: i32) {
	kprintln!("in handler");
	unsafe {core::arch::asm!("mov ebx, 8
								mov eax, 1
								int 0x80");}
}

unsafe fn dumb_main(nb: usize) {
	kprintln!("dumbmain{}!!!", nb);
	let mut pid: Pid = -1;
	if nb > 1 {
		pid = exec_fn!(dumb_main, nb - 1);
	}
	let mut i = 0;
	while i < 2048 {
//		crate::kprintln!("dumb{}", nb);
		i += 1;
	}
	if nb > 1 {
		let test: i32 = sys_kill(pid, 2);
		if test < 0 {
			kprintln!("kill: {}: no such pid: {}", test, pid);
		}
		let mut wstatus: i32 = 0;
		let test: i32 = sys_waitpid(pid, &mut wstatus, 0);
		if __WIFEXITED!(wstatus) {
			kprintln!("exited process pid: {} - exit: {}", test, __WEXITSTATUS!(wstatus));
		} else {
			kprintln!("exited process pid: {} - signal: {}", test, __WSTOPSIG!(wstatus));
		}
	} else {
		sys_signal(2, handler);
		loop {}
	}
	if nb == 3 {
		loop {}
	}
	// TODO: fix syscalls
//	core::arch::asm!("mov ebx, 8
//					mov eax, 1",
//					"int 0x80"); /* test syscall exit */
}

pub fn test_task() {
	let mut pids: [i32; 3] = [0; 3];
	unsafe {
		pids[0] = exec_fn!(dumb_main, 1);
		pids[1] = exec_fn!(dumb_main, 2);
		pids[2] = exec_fn!(dumb_main, 3);
	}

	let mut i = 0;
	while i < 2 {
		let mut wstatus: i32 = 0;
		crate::kprintln!("wait {}", pids[i]);
		let test: i32 = sys_waitpid(pids[i], &mut wstatus, 0);
		crate::kprintln!("end wait: {}", pids[i]);
		if __WIFEXITED!(wstatus) {
			kprintln!("exited process pid: {} - exit: {}", test, __WEXITSTATUS!(wstatus));
		} else {
			kprintln!("exited process pid: {} - signal: {}", test, __WSTOPSIG!(wstatus));
		}
		i += 1;
	}
	/* TEST NO PID */
	let mut wstatus: i32 = 0;
	let test: i32 = sys_waitpid(123, &mut wstatus, 0x01);
	if test < 0 {
		kprintln!("no such pid - waitpid: {}", test);
	}
	/* TEST NO PROCESS */
	let test: i32 = sys_kill(123, 17);
	kprintln!("kill no process pid: {}", test);
	/* TEST NO SIGNAL TYPE */
	let test: i32 = sys_kill(pids[2], 2000);
	kprintln!("kill no signal type: {}", test);
	/* TEST KILL */
	let test: i32 = sys_kill(pids[2], 17);
	kprintln!("killed {} with {}: {}", 1, 17, test);
	/* TEST KILL SIGILL*/
	let test: i32 = sys_kill(pids[2], 9);
	kprintln!("killed {} with {}: {}", 1, 9, test);
	let mut wstatus: i32 = 0;
	let test: i32 = sys_waitpid(pids[2], &mut wstatus, 0x01);
	if __WIFEXITED!(wstatus) {
		kprintln!("exited process pid: {} - exit: {}", test, __WEXITSTATUS!(wstatus));
	} else {
		kprintln!("exited process pid: {} - signal: {}", test, __WSTOPSIG!(wstatus));
	}
	let test: i32 = sys_waitpid(-1, &mut wstatus, 0);
	if __WIFEXITED!(wstatus) {
		kprintln!("exited process pid: {} - exit: {}", test, __WEXITSTATUS!(wstatus));
	} else {
		kprintln!("exited process pid: {} - signal: {}", test, __WSTOPSIG!(wstatus));
	}
//	loop {}
}

#[no_mangle]
pub extern "C" fn kmain() -> ! {
	test_task();

	kprintln!("Hello World of {}!", 42);

	change_color!(Color::Red, Color::White);
	let workspace_msg = string::String::from("Press Ctrl-2 to navigate to the second workspace");
	kprintln!("{}", workspace_msg);
	change_color!(Color::White, Color::Black);

	kprint!("$> ");
//	test_task2();
	loop {}
}
