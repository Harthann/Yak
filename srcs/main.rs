#![feature(const_mut_refs)]
#![feature(lang_items)]
#![no_std]

//use core::arch::asm;
mod io;
mod keyboard;
mod vga_buffer;
mod cli;

use core::arch::asm;
use core::slice;

extern "C" {
	fn stack_bottom();
	fn stack_top();
	fn gdt_start();
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
	hexdump!(unsafe{(0x800 as *mut _)}, unsafe{gdt_desc as usize});
	let mut command: Command = Command::new();
	print!("$> ");
	loop {
		if keyboard::keyboard_event() {
			let charcode = keyboard::handle_event();
			clihandle!(charcode);
		}
	}
}
