pub fn mmap(
	_addr: *const usize,
	_length: usize,
	_prot: i32,
	_flags: i32,
	_fd: i32,
	_offset: usize
) -> *const u8 {
	core::ptr::null()
}

use crate::proc::process::Process;
use crate::utils::arcm::Arcm;
use crate::memory::{MemoryZone, TypeZone, VirtAddr};
pub fn sys_mmap(_hint: VirtAddr, size: usize, _prot: isize, flags: u32, _fd: i32, _offset: usize) -> Result<Arcm<MemoryZone>, ()> {
    let mz = Arcm::new(MemoryZone::init(TypeZone::Anon, size, flags, false));
    let curr_proc = Process::get_running_process();
    curr_proc.add_memory_zone(mz.clone());
    Ok(mz)
}

pub fn sys_munmap(_addr: *const usize, _length: usize) -> i32 {
	0
}

