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

static mut TRACKER: Tracker = Tracker {
	allocation: 0,
	allocated_bytes: 0,
	freed: 0,
	freed_bytes: 0
};

static mut KTRACKER: Tracker = Tracker {
	allocation: 0,
	allocated_bytes: 0,
	freed: 0,
	freed_bytes: 0
};

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

use proc::task::{Task, init_tasking};
use proc::process::{MASTER_PROCESS, Process};

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

	/* on the kernel stack */
	let mut main_process: Process = Process::new();
	let mut main_task: Task = Task::new();
	unsafe{MASTER_PROCESS = &mut main_process};
	unsafe{init_tasking(&mut main_process, &mut main_task)};

	setup_pic8259();
	/* Setting up frequency divider to modulate IRQ0 rate, low value tends to cause pagefault */
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


unsafe fn dumb_main(nb: usize) {
	crate::kprintln!("dumbmain{}!!!", nb);
	let mut i = 0;
	while i < 2048 {
		crate::kprintln!("dumb{} - {}", nb, crate::wrappers::cli_count);
		i += 1;
	}
	if nb > 1 {
		exec_fn!(dumb_main as u32, nb - 1);
	}
	core::arch::asm!("mov eax, 1",
					"int 0x80"); /* test syscall exit */
}

unsafe fn dumb_main2(nb: usize, nb2: u64) {
	crate::kprintln!("dumbmain{} - {:#x?}!!!", nb, nb2);
	if nb > 1 {
		exec_fn!(dumb_main2 as u32, nb - 1, nb2);
	}
	let mut i = 0;
	while i < 2048 {
		crate::kprintln!("dumb{} - {:#x?}", nb, nb2);
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
	while i < 10000 {
		crate::kprintln!("main");
		i += 1;
	}
	crate::kprintln!("MAIN to {}", i);
//	loop {}
}

pub fn test_task2() {
	unsafe {
		exec_fn!(dumb_main2 as u32, 4, 0x123456789abcdef as u64);
	}
}

use crate::syscalls::sys_waitpid;

#[no_mangle]
pub extern "C" fn kmain() -> ! {
//	test_task();

	kprintln!("Hello World of {}!", 42);

	change_color!(Color::Red, Color::White);
	let workspace_msg = string::String::from("Press Ctrl-2 to navigate to the second workspace");
	kprintln!("{}", workspace_msg);
	change_color!(Color::White, Color::Black);

	kprint!("$> ");
//	test_task2();
//	let test: i32;
	/* test syscall asm */
//	unsafe{core::arch::asm!("mov ebx, -1
//					mov eax, 7
//					int 0x80
//					mov {}, eax", out(reg) test)};
	/* test syscall rust */
//	sys_waitpid(-1, core::ptr::null_mut(), 0);
//	crate::kprintln!("result: {}", test);
	loop {}
}
