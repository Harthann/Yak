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

use paging::PageDirectory;
use paging::PageTable;

#[allow(dead_code)]
extern "C" {
	static gdt_desc: u16;
	fn _start();
	fn stack_bottom();
	fn stack_top();
	fn page_directory();
	fn page_table();
}

use vga_buffer::color::Color;
use cli::Command;

#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn eh_personality() {}

#[no_mangle]
pub extern "C" fn kmain() -> ! {
	let page_dir: *mut PageDirectory = page_directory as *mut _;
	unsafe{
		kprintln!("PageDir entry: {}", (*page_dir).entries[0]);
		let page_tab: *mut PageTable = (*page_dir).entries[0].get_address() as *mut _;
		kprintln!("PageTable[0] entry: {}", (*page_tab).entries[0]);
		kprintln!("PageTable[767] entry: {}", (*page_tab).entries[767]);
		kprintln!("PageTable[768] entry: {}", (*page_tab).entries[768]);
		kprintln!("PageTable[234] entry: {}", (*page_tab).entries[234]);
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
	hexdump!(0x800 as *mut _, unsafe{gdt_desc as usize});
	kprint!("$> ");
	loop {
		if keyboard::keyboard_event() {
			let charcode = keyboard::handle_event();
			clihandle!(charcode);
		}
	}
}
