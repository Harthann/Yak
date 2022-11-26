#![feature(lang_items)]
#![no_std]
extern crate adder;

#[global_allocator]
static ALLOCATOR: adder::Dummy = adder::Dummy;

extern "Rust" {
    // These are the magic symbols to call the global allocator.  rustc generates
    // them to call `__rg_alloc` etc. if there is a `#[global_allocator]` attribute
    // (the code expanding that attribute macro generates those functions), or to call
    // the default implementations in libstd (`__rdl_alloc` etc. in `library/std/src/alloc.rs`)
    // otherwise.
    // The rustc fork of LLVM 14 and earlier also special-cases these function names to be able to optimize them
    // like `malloc`, `realloc`, and `free`, respectively.
//    #[rustc_allocator]
 //   #[rustc_nounwind]
    fn __rust_alloc(size: usize, align: usize) -> *mut u8;
  //  #[rustc_deallocator]
//    #[rustc_nounwind]
    fn __rust_dealloc(ptr: *mut u8, size: usize, align: usize);
//    #[rustc_reallocator]
//    #[rustc_nounwind]
    fn __rust_realloc(ptr: *mut u8, old_size: usize, align: usize, new_size: usize) -> *mut u8;
//    #[rustc_allocator_zeroed]
//    #[rustc_nounwind]
    fn __rust_alloc_zeroed(size: usize, align: usize) -> *mut u8;
}
#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn eh_personality() {}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    loop {}
}

fn main() {
    let x = adder::add(2,3);
    unsafe {
        let tmp: *mut u8 = __rust_alloc(8, 8);
        assert_eq!(tmp, 0xdeadbeef as *mut _);
        //println!("{:?}", tmp);
    }
}
