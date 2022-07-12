#![feature(const_mut_refs)]
#![feature(lang_items)]
#![no_std]

mod io;
mod keyboard;
mod vga_buffer;
mod gdt;
mod cli;
mod paging;
mod memory;
mod interrupts;

use core::arch::asm;
use paging::page_directory::PageDirectory;
use paging::page_table::PageTable;

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
pub extern "C" fn kmain() -> ! {
	unsafe {
		let mut page_dir: &mut PageDirectory = &mut *(page_directory as *mut _);
		kprintln!("PageDir entry[0]: {}", page_dir.entries[0]);
		kprintln!("PageDir entry[768]: {}", page_dir.entries[768]);
		kprintln!("PageDir entry[1]: {}", page_dir.entries[1]);
		let page_tab: *mut PageTable = page_dir.entries[0].get_address() as *mut _;
		kprintln!("PageTable entry[0]: {}", (*page_tab).entries[0]);
		kprintln!("PageTable entry[1]: {}", (*page_tab).entries[1]);
		kprintln!("PageTable entry[2]: {}", (*page_tab).entries[2]);
		(*page_tab).entries[769].free_entry();
		kprintln!("New page frame: {:#x}", page_dir.new_page_frame(0x5000000).unwrap());
	}
/*
	let ptr = 0xdeadbeaf as *mut u32;
	unsafe { *ptr = 42; }
*/
	kprintln!("Hello World of {}!", 42);

	change_color!(Color::Red, Color::White);
	kprintln!("Press Ctrl-{} to navigate to the second workspace", '2');
	change_color!(Color::White, Color::Black);

	//kprintln!("Stack bottom: {:x}\nStack top:{:x}\nStart: {:x}\nRust main {:x}", stack_bottom as u32, stack_top as u32, _start as u32, rust_start as u32);
//	hexdump!(0x800 as *mut _, unsafe{gdt_desc as usize});
	kprint!("$> ");
	loop {
		if keyboard::keyboard_event() {
			let charcode = keyboard::handle_event();
			clihandle!(charcode);
		}
	}
}
