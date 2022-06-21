#![feature(const_mut_refs)]
#![feature(lang_items)]
#![no_std]

//use core::arch::asm;
mod io;
mod keyboard;
mod vga_buffer;
mod cli;


extern "C" {
	static gdt_desc: u16;
}

use vga_buffer::color::Color;
use cli::Command;

#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn eh_personality() {}

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
	println!("Hello World of {}!", 42);
	change_color!(Color::Red, Color::White);
	println!("Press Ctrl-{} to navigate to the second workspace", '2');
	change_color!(Color::White, Color::Black);

	/* print GDT */
	hexdump!(0x800 as *mut _, unsafe{gdt_desc as usize});
	print!("$> ");
	loop {
		if keyboard::keyboard_event() {
			let charcode = keyboard::handle_event();
			clihandle!(charcode);
		}
	}
}
