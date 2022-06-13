#![feature(lang_items)]
#![no_std]
extern crate rlibc;

use core::panic::PanicInfo;
mod vga_buffer;

#[lang = "eh_personality"] #[no_mangle] pub extern fn eh_personality() {}
//#[lang = "panic_fmt"] #[no_mangle] pub extern fn panic_fmt() -> ! {loop{}}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	loop {}
}


#[no_mangle]
pub extern fn rust_main() -> ! {
    vga_buffer::print_something();

	loop {}
}
