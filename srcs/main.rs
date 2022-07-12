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
		let mut page_tab: &mut PageTable = &mut *(page_table as *mut _);
		kprintln!("entry[0]: {:#x}", page_tab.entries[0].get_paddr());
		kprintln!("entry[1023]: {:#x}", page_tab.entries[1023].get_paddr());
		page_tab.init();
		kprintln!("page_tab -> :{:#x}", page_dir.entries[0].get_paddr() as usize);
		kprintln!("entry[1023]: {:#x}", page_tab.entries[1023].get_paddr());

		page_tab = &mut *(0xc03ff000 as *mut _);

		let mut i: usize = 0;

		while i < 1024 {
			kprintln!("entry[{}]: {:#x}", i, page_tab.entries[i].get_paddr());
			i += 1;
		}
//		kprintln!("PageDir entry[0]: {}", page_dir.entries[0]);
		kprintln!("PageDir entry[1]: {}", page_dir.entries[1]);
		page_tab = page_dir.new_page_table();
		kprintln!("PageDir entry[1]: {}", page_dir.entries[1]);
		asm!("hlt");
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
