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
#![feature(core_intrinsics)]
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

// Modules import
mod cli;
mod gdt;
mod interrupts;
mod io;
mod keyboard;
mod main;
mod memory;
mod multiboot;
mod pic;
mod proc;
mod spin;
mod string;
mod syscalls;
mod user;
mod utils;
mod vec;
mod vga_buffer;
mod wrappers;

#[cfg(test)]
mod test;

// Modules used function and variable
use crate::gdt::{gdt_desc, update_gdtr, KERNEL_BASE};
use crate::interrupts::init_idt;
use crate::memory::paging::PAGE_WRITABLE;
use crate::memory::{init_heap, init_stack, MemoryZone, VirtAddr};
use cli::Command;
use main::kmain;
use memory::allocator::linked_list::LinkedListAllocator;
use memory::paging::{init_paging, page_directory};
pub use pic::handlers::JIFFIES;
use pic::setup_pic8259;

// Code from boot section
#[allow(dead_code)]
extern "C" {
	fn stack_bottom();
	fn stack_top();
	fn heap();
}

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

#[global_allocator]
static mut ALLOCATOR: LinkedListAllocator = LinkedListAllocator::new();
static mut KALLOCATOR: LinkedListAllocator = LinkedListAllocator::new();

static mut TRACKER: Tracker = Tracker::new();
static mut KTRACKER: Tracker = Tracker::new();

pub static mut KSTACK: MemoryZone = MemoryZone::new();
pub static mut KHEAP: MemoryZone = MemoryZone::new();

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

// Kernel initialisation
#[no_mangle]
pub extern "C" fn kinit() {
	crate::wrappers::_cli();

	// Init paging and remove identity paging
	init_paging();

	// Update gdtr with higher half kernel gdt addr
	unsafe {
		update_gdtr();
		reload_gdt!();
		init_idt();
	}

	// HEAP KERNEL
	let kstack_addr: VirtAddr = 0xffbfffff; // stack kernel
	unsafe {
		KSTACK = init_stack(kstack_addr, 2 * 0x1000, PAGE_WRITABLE, false);
		KHEAP = init_heap(
			heap as u32,
			100 * 0x1000,
			PAGE_WRITABLE,
			true,
			&mut KALLOCATOR
		);
	}

	gdt::tss::init_tss(kstack_addr);
	reload_tss!();

	#[cfg(feature = "multitasking")]
	init_tasking();

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
		0x00ff
	);
	pic::set_irq0_in_ms(1.0);

	// Reserve some spaces to push things before main
	unsafe { core::arch::asm!("mov esp, {}", in(reg) kstack_addr - 256) };
	crate::wrappers::_sti();

	// Function to test and enter usermode

	#[cfg(test)]
	test_main();

	#[cfg(not(test))]
	kmain();
}
