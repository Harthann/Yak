#![feature(lang_items)]
#![no_std]

use core::panic::PanicInfo;

#[lang = "eh_personality"] #[no_mangle] pub extern fn eh_personality() {}
//#[lang = "panic_fmt"] #[no_mangle] pub extern fn panic_fmt() -> ! {loop{}}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

extern "C" {
    fn _print();
}

#[no_mangle]
pub extern fn rust_main() {
    let x = ["hello", "World", "!"];
    let y =x;
//    unsafe {_print();}
}


