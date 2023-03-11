pub fn sys_mmap(
	_addr: *const usize,
	_length: usize,
	_prot: i32,
	_flags: i32,
	_fd: i32,
	_offset: usize
) -> *const u8 {
	core::ptr::null()
}

pub fn sys_munmap(_addr: *const usize, _length: usize) -> i32 {
	0
}
