#![feature(lang_items)]
#![no_std]
extern crate rlibc;

use core::panic::PanicInfo;

#[lang = "eh_personality"] #[no_mangle] pub extern fn eh_personality() {}
//#[lang = "panic_fmt"] #[no_mangle] pub extern fn panic_fmt() -> ! {loop{}}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	loop {}
}

extern "C" {
	static cursor: u32;
}

static HELLO: &[u8] = b"Hello World!";


#[no_mangle]
pub extern fn rust_main() -> ! {
	unsafe{let vga_buffer = cursor as *mut u8;

	for (i, &byte) in HELLO.iter().enumerate() {
			*vga_buffer.offset(i as isize * 2) = byte;
			*vga_buffer.offset(i as isize * 2 + 1) = 0xb;
	}}

	loop {}
}
