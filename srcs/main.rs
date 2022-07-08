#![feature(const_mut_refs)]
#![feature(lang_items)]
#![no_std]

//use core::arch::asm;
mod io;
mod keyboard;
mod vga_buffer;
mod gdt;
mod cli;
mod paging;

use paging::PAGE_DIRECTORY;
use paging::PAGE_TABLE;
use paging::enable_paging;
use gdt::reload_gdt;

#[allow(dead_code)]
extern "C" {
	static gdt_desc: u16;
	fn _start();
	fn stack_bottom();
	fn stack_top();
}

use vga_buffer::color::Color;
use cli::Command;

#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn eh_personality() {}

pub fn kernel_main() -> ! {
/*
	let ptr = 0xdeadbeaf as *mut u32;
	unsafe { *ptr = 42; }
*/

	println!("Hello World of {}!", 42);

	change_color!(Color::Red, Color::White);
	println!("Press Ctrl-{} to navigate to the second workspace", '2');
	change_color!(Color::White, Color::Black);

	//println!("Stack bottom: {:x}\nStack top:{:x}\nStart: {:x}\nRust main {:x}", stack_bottom as u32, stack_top as u32, _start as u32, rust_start as u32);
	hexdump!(0x800 as *mut _, unsafe{gdt_desc as usize});
	print!("$> ");
	loop {
		if keyboard::keyboard_event() {
			let charcode = keyboard::handle_event();
			clihandle!(charcode);
		}
	}
}

#[no_mangle]
#[link_section = ".boot"]
pub extern "C" fn rust_start() {
	reload_gdt();
	unsafe{PAGE_DIRECTORY.entries[0] = (((&PAGE_TABLE as *const _) as usize) | 3) as *mut _};
	enable_paging();
	kernel_main();
}
