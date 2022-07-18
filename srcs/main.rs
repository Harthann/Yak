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
use paging::alloc_page;
use paging::free_page;
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
//		kprintln!("page[1]: {}", page_directory.entries[0]);
		let mut res = alloc_page();
		if !res.is_ok() {
			kprintln!("ko");
			core::arch::asm!("hlt");
		}
		let mut virt_addr: u32 = res.unwrap();
		kprintln!("virt_addr: {:#x}", virt_addr);
		kprintln!("paddr: {:#x}", get_paddr!(virt_addr as usize));
		let mut nb: *mut usize = &mut *(virt_addr as *mut usize);
		kprintln!("init value of nb: {:#x}", *nb);
		*nb = 8;
		kprintln!("next value of nb: {:#x}", *nb);
		let mut i = 0;
		while i < 0x100000 {
			res = alloc_page();
			if !res.is_ok() {
				kprintln!("ko");
				core::arch::asm!("hlt");
			}
			virt_addr = res.unwrap();
			kprintln!("{}: {:#010x}", i, virt_addr);
			if i % 2 == 0 {
				free_page(virt_addr);
			}
			i += 1;
		}
//		page_directory.remove_page_frame(virt_addr);
//		*nb = 0x1000;
//		kprintln!("next value of nb: {:#x}", *nb);
//		kprintln!("paddr: {:#x}", get_paddr!(virt_addr as usize));
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
