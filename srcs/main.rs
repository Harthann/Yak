#![feature(const_mut_refs)]
#![feature(lang_items)]
#![no_std]

//use core::arch::asm;
mod io;
mod keyboard;
mod vga_buffer;
mod cli;

mod gdt;

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

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
	println!("Hello World of {}!", 42);
	change_color!(Color::Red, Color::White);
	println!("Press Ctrl-{} to navigate to the second workspace", '2');
	change_color!(Color::White, Color::Black);

	unsafe { println!("Stack bottom: {:x}\nStack top:{:x}\nStart: {:x}\nRust main {:x}", stack_bottom as u32, stack_top as u32, _start as u32, rust_main as u32)};
	unsafe { gdt::set_segment(gdt::KERNEL_CODE, _start as u32, 0x00ffff3f, 0x4e, 0x77)};
	unsafe { gdt::set_segment(gdt::KERNEL_STACK, stack_bottom as u32, stack_top as u32 - stack_bottom as u32, 0x0c, 0x96) };
	gdt::print_gdt();

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
