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
#![no_std]
#![allow(dead_code)]
#![allow(incomplete_features)]
#![no_main]

/*  Custom test framwork    */
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

use crate::gdt::{KERNEL_BASE, gdt_desc, update_gdtr};
//use crate::memory::paging::{alloc_pages_at_addr, PAGE_USER};
pub use pic::handlers::JIFFIES;


/*  Kernel initialisation   */
#[no_mangle]
pub extern "C" fn kinit() {
	cli!();
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
	init_stack(kstack_addr, 2 * 0x1000, PAGE_WRITABLE, false);
	unsafe {init_heap(heap as u32, 100 * 0x1000, PAGE_WRITABLE, true, &mut KALLOCATOR)};

	gdt::tss::init_tss(kstack_addr);
	reload_tss!();

	let mut main_task: Task = Task::new();
	unsafe{init_tasking(&mut main_task)};

	setup_pic8259();
	/* Setting up frequency divider to modulate IRQ0 rate, low value tends to cause pagefault */
	pic::set_pit(pic::pit::CHANNEL_0, pic::pit::ACC_LOBHIB, pic::pit::MODE_2, 0xffff);

	/* Reserve some spaces to push things before main */
	unsafe{core::arch::asm!("mov esp, {}", in(reg) kstack_addr - 256)};
	sti!();

	/*	Function to test and enter usermode */
//	user::test_user_page();

	#[cfg(test)]
	test_main();

	#[cfg(not(test))]
	kmain();
}

use proc::{Task, init_tasking};

fn dumb_main() {
	crate::kprintln!("dumbmain1!!!");
	let mut i = 0;
	while i < 2048 {
		crate::kprintln!("dumb1");
		i += 1;
	}
}

fn dumb_main2() {
	crate::kprintln!("dumbmain2!!!");
	let mut i = 0;
	while i < 2048 {
		crate::kprintln!("dumb2");
		i += 1;
	}
}

fn dumb_main3() {
	crate::kprintln!("dumbmain3!!!");
	let mut i = 0;
	while i < 2048 {
		crate::kprintln!("dumb3");
		i += 1;
	}
}

use crate::memory::paging::alloc_page;
use crate::proc::exec_fn;

pub fn test_task() {
	let esp: u32;
	let res = alloc_page(PAGE_WRITABLE);
	if res.is_ok() {
		esp = res.unwrap();
	} else {
		todo!();
	}
	exec_fn(esp, dumb_main as u32, 0x1000);

	let esp: u32;
	let res = alloc_page(PAGE_WRITABLE);
	if res.is_ok() {
		esp = res.unwrap();
	} else {
		todo!();
	}
	exec_fn(esp, dumb_main2 as u32, 0x1000);

	let esp: u32;
	let res = alloc_page(PAGE_WRITABLE);
	if res.is_ok() {
		esp = res.unwrap();
	} else {
		todo!();
	}
	exec_fn(esp, dumb_main3 as u32, 0x1000);

	let mut i = 0;
	while i < 10000 {
		crate::kprintln!("main");
		i += 1;
	}
	crate::kprintln!("MAIN to {}", i);
//	loop {}
}

fn dumb_main4() {
	crate::kprintln!("dumbmain4!!!");
	let mut i = 0;
	while i < 2048 {
		crate::kprintln!("dumb4");
		i += 1;
	}
}

pub fn test_task2() {
	let esp: u32;
	let res = alloc_page(PAGE_WRITABLE);
	if res.is_ok() {
		esp = res.unwrap();
	} else {
		todo!();
	}
	exec_fn(esp, dumb_main4 as u32, 0x1000);
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
	test_task2();
	loop {
		unsafe{core::arch::asm!("hlt")};
	}
}
