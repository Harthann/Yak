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
#![feature(asm_const)]
#![no_std]
#![allow(dead_code)]
#![allow(incomplete_features)]
#![no_main]
// Custom test framework
#![feature(custom_test_frameworks)]
#![test_runner(crate::test::test_runner)]
#![reexport_test_harness_main = "test_main"]

//! 32bits - i386 Rust Kernel from Scratch 🦀

#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn eh_personality() {}

const GLOBAL_ALIGN: usize = 8;

// Allocation tracking
pub struct Tracker {
	allocation:      usize,
	allocated_bytes: usize,
	freed:           usize,
	freed_bytes:     usize
}

impl Tracker {
	pub const fn new() -> Self {
		Self {
			allocation:      0,
			allocated_bytes: 0,
			freed:           0,
			freed_bytes:     0
		}
	}
}

static mut TRACKER: Tracker = Tracker::new();
static mut KTRACKER: Tracker = Tracker::new();

pub fn memory_state() {
	unsafe {
		kprintln!(
			"\nAllocation: {} for {} bytes",
			KTRACKER.allocation,
			KTRACKER.allocated_bytes
		);
		kprintln!(
			"Free:       {} for {} bytes",
			KTRACKER.freed,
			KTRACKER.freed_bytes
		);
	}
}

// Modules import
mod cli;
mod gdt;
mod keyboard;
mod main;
#[macro_use]
mod memory;
mod interrupts;
mod multiboot;
mod string;
mod vec;
#[macro_use]
mod syscalls;
mod io;
mod pic;
mod proc;
mod user;
mod vga_buffer;
#[macro_use]
mod wrappers;
mod errno;
mod spin;
mod utils;

#[cfg(test)]
mod test;

// Modules used function and variable
use cli::Command;
use memory::allocator::linked_list::LinkedListAllocator;
use memory::paging::{init_paging, page_directory};
use pic::setup_pic8259;

#[global_allocator]
static mut ALLOCATOR: LinkedListAllocator = LinkedListAllocator::new();
static mut KALLOCATOR: LinkedListAllocator = LinkedListAllocator::new();

// Code from boot section
#[allow(dead_code)]
extern "C" {
	fn stack_bottom();
	fn stack_top();
	fn heap();
}

use crate::memory::VirtAddr;

use crate::interrupts::init_idt;

use proc::task::Task;

use crate::gdt::{gdt_desc, update_gdtr, KERNEL_BASE};
// use crate::memory::paging::{alloc_pages_at_addr, PAGE_USER};
use main::kmain;
pub use pic::handlers::JIFFIES;

const KSTACK_ADDR: VirtAddr = 0xffbfffff;
const STACK_ADDR: VirtAddr = 0xffafffff;

// Kernel initialisation
#[no_mangle]
pub extern "C" fn kinit() {
	crate::wrappers::_cli();

	// multiboot::read_tags();
	// Init paging and remove identity paging
	init_paging();

	// Update gdtr with higher half kernel gdt addr
	unsafe {
		update_gdtr();
		reload_gdt!();
		init_idt();
	}

	Task::init_multitasking(STACK_ADDR, heap as u32);

	gdt::tss::init_tss(KSTACK_ADDR);
	reload_tss!();

	// init tracker after init first process
	unsafe {
		KTRACKER = Tracker::new();
		TRACKER = Tracker::new();
	}

	setup_pic8259();
	// Setting up frequency divider to modulate IRQ0 rate, low value tends to get really slow (too much task switching
	pic::set_pit(
		pic::pit::CHANNEL_0,
		pic::pit::ACC_LOBHIB,
		pic::pit::MODE_2,
		0x0fff
	);

	// Reserve some spaces to push things before main
	unsafe { core::arch::asm!("mov esp, {}", in(reg) STACK_ADDR - 256) };
	crate::wrappers::_sti();

	// Function to test and enter usermode
	// user::test_user_page();

	#[cfg(test)]
	test_main();

	#[cfg(not(test))]
	kmain();
}