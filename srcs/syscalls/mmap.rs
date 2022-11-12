pub fn sys_mmap(addr: *const usize, length: usize, prot: i32, flags: i32, fd: i32, offset: usize) -> *const u8 {
	core::ptr::null()
}

pub fn sys_munmap(addr: *const usize, length: usize) -> i32 {
	0
}
