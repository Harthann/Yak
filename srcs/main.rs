#![feature(const_mut_refs)]
#![feature(lang_items)]
#![no_std]

//use core::arch::asm;
mod vga_buffer;
mod io;
mod keyboard;

extern "C" {
	fn stack_bottom();
	fn stack_top();
	fn GDT_start();
	static GDT_ptr: u16;
}

use vga_buffer::color::Color;

#[lang = "eh_personality"] #[no_mangle] pub extern fn eh_personality() {}

#[no_mangle]
pub extern fn rust_main() -> ! {
	println!("Hello World of {}!", 42);
	change_color!(Color::Red, Color::White);
	println!("Press Ctrl-{} to navigate to the second workspace", '2');
	change_color!(Color::White, Color::Black);
/*
	let stack_size = stack_top as usize - stack_bottom as usize;
	let offset = unsafe{(stack_bottom as *const u8).offset((stack_size - 256) as isize)};
	hexdump!(offset, 256);
*/
	/* print GDT */
	hexdump!(unsafe{&*(0x800 as *mut _)}, unsafe{GDT_ptr as usize});
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

