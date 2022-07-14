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

use paging::page_directory::PageDirectory;

use paging::init_paging;
use paging::page_directory;

#[allow(dead_code)]
extern "C" {
	static gdt_desc: u16;
	fn _start();
	fn stack_bottom();
	fn stack_top();
	fn heap();
}

use vga_buffer::color::Color;
use cli::Command;

#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn eh_personality() {}

#[no_mangle]
pub extern "C" fn kinit() {
	init_paging();
	kmemory::physmap_as_mut().claim_range(0x0, 1024);
	kmain();
}

/*  Function to put all tests and keep main clean */
fn test() {
	unsafe {
//		let ptr: kmemory::PhysAddr = kmemory::physmap_as_mut().get_page();
//		kprintln!("Get this {:#x}", ptr);
//		kprintln!("Get this {:#x}", kmemory::physmap_as_mut().get_page());
//		kmemory::physmap_as_mut().free_page(ptr);
//		kprintln!("Get this {:#x}", kmemory::physmap_as_mut().get_page());

		/* TESTS PAGES */
		let page_dir: &mut PageDirectory = &mut page_directory;
		kprintln!("PageDir entry[1]: {}", page_dir.entries[0]);
		kprintln!("PageDir entry[0]: {}", page_dir.entries[0]);
		page_dir.new_page_table();
		kprintln!("PageDir entry[0]: {}", page_dir.entries[0]);
		kprintln!("page_dir: {:#p}", page_dir);
		kprintln!("PageDir entry[1]: {}", page_dir.entries[1]);
		page_dir.remove_page_table(0);
		kprintln!("PageDir entry[1]: {}", page_dir.entries[1]);
		kprintln!("PageDir entry[0]: {}", page_dir.entries[0]);
	}
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
