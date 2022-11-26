#![no_std]
#![feature(lang_items)]
#![feature(alloc_error_handler)]
//#![cfg_attr(feature = "cross-compiled", no_std)]

//#[cfg(test)]
//#[macro_use]
//extern crate std;

//extern crate alloc;

use core::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;



pub struct Dummy;
unsafe impl GlobalAlloc for Dummy {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        0xdeadbeef as *mut _
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        panic!("dealloc should be never called")
    }
}

//#[cfg(not(test))]
//#[alloc_error_handler]
//fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
//    panic!("allocation error: {:?}", layout)
//}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2,2);
        assert_eq!(result, 4);
        assert(tmp != 0);
    }

    #[test]
    fn it_works_well() {
        let result = add(5,5);
        assert_eq!(result, 10);
    }
}
