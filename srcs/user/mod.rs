//! Testing user space code

use crate::memory::paging as mem;
use crate::memory::paging::{PAGE_USER, PAGE_WRITABLE};

extern "C" {
	fn jump_usermode();
	fn userfunc();
	fn userfunc_end();
}

pub fn test_user_page() {
	let userpage =
		mem::alloc_pages_at_addr(0x400000, 1, PAGE_WRITABLE | PAGE_USER)
			.expect("");
	let funclen = userfunc_end as usize - userfunc as usize;

	unsafe {
		core::ptr::copy_nonoverlapping(
			userfunc as *const u8,
			userpage as *mut u8,
			funclen
		);
	}
	mem::print_pdentry(1);
	unsafe {
		jump_usermode();
	}
}
