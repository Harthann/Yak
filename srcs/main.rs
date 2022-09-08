#![feature(const_mut_refs)]
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
mod io;
mod vga_buffer;

#[cfg(test)]
mod test;

/*  Modules used function and variable  */
use memory::paging::{init_paging, page_directory};
use memory::allocator::linked_list::LinkedListAllocator;
use vga_buffer::color::Color;
use cli::Command;

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

/*  Kernel initialisation   */
#[no_mangle]
pub extern "C" fn kinit() {
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
	unsafe {init_heap(heap as u32, 100 * 4096, PAGE_WRITABLE, true, &mut KALLOCATOR)};
	let kstack_addr: VirtAddr = 0xffbfffff; /* stack kernel */
	init_stack(kstack_addr, 8192, PAGE_WRITABLE, false);
	/* Reserve some spaces to push things before main */
	unsafe{core::arch::asm!("mov esp, eax", in("eax") kstack_addr - 256)};

	#[cfg(test)]
	test_main();

	#[cfg(not(test))]
	kmain();
}

#[no_mangle]
pub extern "C" fn kmain() -> ! {
	kprintln!("Hello World of {}!", 42);

	change_color!(Color::Red, Color::White);
	let workspace_msg = string::String::from("Press Ctrl-2 to navigate to the second workspace");
	kprintln!("{}", workspace_msg);
	change_color!(Color::White, Color::Black);

	kprint!("$> ");
	loop {
		if keyboard::keyboard_event() {
			let charcode = keyboard::handle_event();
			clihandle!(charcode);
		}
	}
}

/*  Function to put all tests and keep main clean */
#[cfg(not(test))]
fn test() {
	string::test();
}
