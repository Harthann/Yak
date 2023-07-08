#![feature(const_mut_refs)]
#![feature(naked_functions)]
#![feature(const_size_of_val)]
#![feature(lang_items)]
#![feature(c_variadic)]
#![feature(asm_const)]
#![feature(alloc_error_handler)]
#![feature(unsize)]
#![feature(coerce_unsized)]
#![feature(vec_into_raw_parts)]
#![no_std]
#![allow(dead_code)]
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
#[derive(Copy, Clone)]
pub struct Tracker {
	allocation:      usize,
	allocated_bytes: usize,
	freed:           usize,
	freed_bytes:     usize
}

use core::fmt;
impl fmt::Display for Tracker {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(
			f,
			"Alloc: {} for {} bytes\nFreed: {} for {} bytes",
			self.allocation, self.allocated_bytes, self.freed, self.freed_bytes
		)
	}
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

static mut KTRACKER: Tracker = Tracker::new();

pub fn memory_state() {
	unsafe {
		kprintln!("{}", KTRACKER);
	}
}

// Modules import
mod boot;
mod cli;
mod gdt;
mod keyboard;
mod kmain;
#[macro_use]
mod memory;
mod interrupts;
mod multiboot;
#[macro_use]
mod syscalls;
mod io;
mod pci;
mod pic;
mod proc;
mod time;
mod user;
mod vga_buffer;
#[macro_use]
mod wrappers;
mod cmos;
mod errno;
mod sound;
mod spin;
mod utils;
#[macro_use]
mod debug;
mod fs;

extern crate alloc;
extern crate sys_macros;
// mod alloc;

use alloc::{string, vec};

#[cfg(test)]
mod test;

// Modules used function and variable

use cli::Command;
use memory::allocator::linked_list::LinkedListAllocator;
use memory::paging::{init_paging, page_directory};
use pci::ide::IDE;
use pic::setup_pic8259;

#[global_allocator]
static mut KALLOCATOR: LinkedListAllocator = LinkedListAllocator::new();

#[alloc_error_handler]
pub fn rust_oom(layout: core::alloc::Layout) -> ! {
	panic!("Failed to allocate memory: {}", layout.size())
}

use crate::memory::VirtAddr;

use crate::interrupts::init_idt;

use proc::task::Task;

use crate::gdt::{gdt_desc, GDTR};
// use crate::memory::paging::{alloc_pages_at_addr, PAGE_USER};

const KSTACK_ADDR: VirtAddr = 0xffbfffff;
const STACK_ADDR: VirtAddr = 0xff0fffff;
const STACK_SIZE: u32 = 0x1000 * 4;

// Kernel initialisation
#[no_mangle]
pub extern "C" fn kinit() {
	crate::wrappers::_cli();

	multiboot::read_tags();
	// Init paging and remove identity paging
	init_paging();

	// Update gdtr with higher half kernel gdt addr
	unsafe {
		GDTR::update();
		reload_gdt!();
		init_idt();
	}

	Task::init_multitasking(STACK_ADDR);

	gdt::tss::init_tss(KSTACK_ADDR + 1);
	reload_tss!();

	// init tracker after init first process
	unsafe {
		KTRACKER = Tracker::new();
	}

	unsafe {
		IDE::initialize(0x1F0, 0x3F6, 0x170, 0x376, 0x000)
			.expect("Error while reading disks")
	};

	setup_pic8259();

	// Setting up frequency divider to modulate IRQ0 rate, low value tends to get really slow (too much task switching
	// This setup should be done using frequency, but for readability and ease of use, this is done
	// with time between each interrupt in ms.
	pic::set_irq0_in_ms(1.0);

	// Reserve some spaces to push things before main
	unsafe { core::arch::asm!("mov esp, {}", in(reg) STACK_ADDR + 1 - 32) };
	crate::wrappers::_sti();

	#[cfg(test)]
	test_main();

	#[cfg(not(test))]
	kmain::kmain();
}
