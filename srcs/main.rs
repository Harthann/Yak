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

use paging::init_paging;
use paging::page_directory;
use paging::page_table::PageTable;

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
//		page_directory.new_page_table();
//		page_directory.remove_page_table(0);
//		kprintln!("page_dir: {:#p}", &mut page_directory);
//		let vaddr: usize = (&mut page_directory as *mut _) as usize;
//		kprintln!("page_dir get_paddr {:#x}", get_paddr!((&mut page_directory as *mut _) as usize));
		kprintln!("page_directory: {:#x}", page_directory.get_vaddr() - 0xc0000000);
		page_directory.new_page_table();
		kprintln!("one");
		page_directory.new_page_table();
		kprintln!("two");
		page_directory.new_page_table();
		kprintln!("three");
		page_directory.new_page_table();
		kprintln!("four");
		let page_table: &mut PageTable = page_directory.get_page_table(0);
		kprintln!("beep boop {:#x}", page_directory.entries[0].get_paddr());
		kprintln!("beep boop {:#x}", page_directory.entries[1].get_paddr());
		kprintln!("beep boop {:#x}", page_directory.entries[2].get_paddr());
		kprintln!("beep boop {:#x}", page_directory.entries[3].get_paddr());
		kprintln!("{}", page_table.entries[1023].get_paddr());
		/*
		let mut i: usize = 0;

		// TODO: fix weird bug with > 1024 (stack explode ?)
		while i < 1024 {
			let res = page_directory.new_page_frame(0xfffff000);
			kprintln!("address: {:#x}", res.unwrap());
			i += 1;
		}
		*/
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
