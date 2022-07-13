#![feature(const_mut_refs)]
#![feature(lang_items)]
#![no_std]
#![allow(dead_code)]

mod io;
mod keyboard;
mod vga_buffer;
mod gdt;
mod cli;
mod paging;
mod interrupts;
mod kmemory;

use core::arch::asm;
use paging::page_directory::PageDirectory;
use paging::page_table::PageTable;
use kmemory::get_map_mut;

#[allow(dead_code)]
extern "C" {
	static gdt_desc: u16;
	fn _start();
	fn stack_bottom();
	fn stack_top();
	fn page_directory();
	fn page_table();
	fn heap();
}

use vga_buffer::color::Color;
use cli::Command;

#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn eh_personality() {}

#[no_mangle]
pub extern "C" fn kinit() {
	kmemory::get_map_mut().claim_range(0x0, 1024);
	kmain();
}

/*  Function to put all tests and keep main clean */
fn test() {
//	unsafe {
//		let ptr: kmemory::PhysAddr = kmemory::get_map_mut().get_page();
//		kprintln!("Get this {:#x}", ptr);
//		kprintln!("Get this {:#x}", kmemory::get_map_mut().get_page());
//		kmemory::get_map_mut().free_page(ptr);
//		kprintln!("Get this {:#x}", kmemory::get_map_mut().get_page());
//	}
}

#[no_mangle]
pub extern "C" fn kmain() -> ! {
	kprintln!("Hello World of {}!", 42);
	change_color!(Color::Red, Color::White);
	kprintln!("Press Ctrl-{} to navigate to the second workspace", '2');
	change_color!(Color::White, Color::Black);

	test();

	kprint!("$> ");
	loop {
		if keyboard::keyboard_event() {
			let charcode = keyboard::handle_event();
			clihandle!(charcode);
		}
	}
}
