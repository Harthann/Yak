#![feature(lang_items)]
#![no_std]
extern crate rlibc;

use core::panic::PanicInfo;
mod vga_buffer;

#[lang = "eh_personality"] #[no_mangle] pub extern fn eh_personality() {}

#[no_mangle]
pub extern fn rust_main() -> ! {
	println!("Hello World of {}!", 42);

	loop {}
}
