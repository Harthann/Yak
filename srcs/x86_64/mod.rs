pub mod port;
pub mod io;
pub mod paging;
pub mod interrupt;

#[allow(dead_code)]
pub fn io_wait() {
    unsafe {
	    	core::arch::asm!("out dx, eax",
		        in("dx") 0x80,
		        in("eax") 0,
                options(nomem, nostack, preserves_flags));
	    }
}
