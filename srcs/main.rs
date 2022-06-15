#![feature(lang_items)]
#![no_std]

use core::arch::asm;
mod vga_buffer;

#[lang = "eh_personality"] #[no_mangle] pub extern fn eh_personality() {}

#[no_mangle]
pub extern fn rust_main() -> ! {
	println!("Hello World of {}!", 42);
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
	assert!(x	== 6);


	loop {}
}
