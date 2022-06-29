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

//macro_rules! test {
//	($fg:expr, $bg:expr, $string:expr, ($arg:tt)*) =>	(
//	change_color!($fg, $bg);
//	println!($string, format_args!($($arg)*));
//	change_color!(Color::White, Color::Black);
//	);
//}

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
	unsafe{PAGE_DIRECTORY.entries[0] = (((&PAGE_TABLE as *const _) as usize) | 3) as *mut _};
	enable_paging();

/*
	let ptr = 0xdeadbeaf as *mut u32;
	unsafe { *ptr = 42; }
*/

	println!("Hello World of {}!", 42);
//	test!(Color::Yellow, Color::Red, "This is a test {}", 42);

	change_color!(Color::Red, Color::White);
	println!("Press Ctrl-{} to navigate to the second workspace", '2');
	change_color!(Color::White, Color::Black);

/*
	println!("Stack bottom: {:x}\nStack top:{:x}\nStart: {:x}\nRust main {:x}", stack_bottom as u32, stack_top as u32, _start as u32, rust_main as u32);

	gdt::print_gdt();
	/* print GDT */
*/
	hexdump!(0x800 as *mut _, unsafe{gdt_desc as usize});
	print!("$> ");
	loop {
		if keyboard::keyboard_event() {
			let charcode = keyboard::handle_event();
			clihandle!(charcode);
		}
	}
}
