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

use paging::get_vaddr;

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
		let mut paddr: usize = (page_directory as *mut usize) as usize - 0xc0000000;
		let mut page_dir: &mut PageDirectory = &mut *(paddr as *mut _);
		let mut init_page_tab: &mut PageTable = &mut *((paddr + 0x1000) as *mut _);
		init_page_tab.entries[1022] = (((paddr + (768 + 1) * 0x1000) | 3) as u32).into();
		let mut page_tab: &mut PageTable = &mut *(get_vaddr(0, 1022) as *mut _);
		page_tab.init(paddr + (768 + 1) * 0x1000);
		page_dir.entries[768] = (((paddr + (768 + 1) * 0x1000) | 3) as u32).into();
		page_dir.remove_page_table(0);
		page_tab = &mut *(get_vaddr(768, 1023) as *mut _);

		/* TESTS */
		page_dir = &mut *(page_directory as *mut _);
		kprintln!("PageDir entry[1]: {}", page_dir.entries[0]);
		kprintln!("PageDir entry[0]: {}", page_dir.entries[0]);
		page_tab = page_dir.new_page_table();
		kprintln!("PageDir entry[0]: {}", page_dir.entries[0]);
		kprintln!("page_dir: {:#p}", page_dir);
		kprintln!("PageDir entry[1]: {}", page_dir.entries[1]);
		page_dir.remove_page_table(0);
		kprintln!("PageDir entry[1]: {}", page_dir.entries[1]);
		kprintln!("PageDir entry[0]: {}", page_dir.entries[0]);
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
