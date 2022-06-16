#![feature(lang_items)]
#![no_std]

use core::arch::asm;
mod vga_buffer;
mod io;
mod keyboard;

#[lang = "eh_personality"] #[no_mangle] pub extern fn eh_personality() {}

#[no_mangle]
pub extern fn rust_main() -> ! {
	println!("Hello World of {}!", 42);
	vga_buffer::change_color(vga_buffer::Color::Red, vga_buffer::Color::White);
	println!("Hello World of {}!", 42);
	vga_buffer::change_color(vga_buffer::Color::White, vga_buffer::Color::Black);
/*
	let mut x: u32 = 4;
	unsafe {
		asm!(
				"mov {tmp}, {x}",
				"shl {tmp}, 1",
				"shl {x}, 2",
				"add {x}, {tmp}",
				x = inout(reg) x,
				tmp = out(reg) _,
			);
	}
	assert!(x == 6);
*/

	loop {
		if keyboard::keyboard_event() {
			keyboard::handle_event();
		}
	}
}

