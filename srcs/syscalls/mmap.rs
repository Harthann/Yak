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

/// hint: Adress at which the kernel will look for free space.
/// size; Size of the mapping, will be rounded up to a multiplpe of PAGE_SIZE
/// prot: Indicate protection settings to setup the LDT for the current process
/// flags: Flags of the memory zone that will be allocated
/// fd: FileDescriptor to the file that needs to be mapped
/// offset: offset on the file from which the mapping will start
pub fn sys_mmap(_hint: VirtAddr, size: usize, _prot: isize, flags: u32, _fd: i32, _offset: usize) -> Result<Arcm<MemoryZone>, ()> {
    // Parameters given to init will vary depending on flags given
    let mz = Arcm::new(MemoryZone::init(TypeZone::Anon, size, flags, false));
    let curr_proc = Process::get_running_process();
    curr_proc.add_memory_zone(mz.clone());
    Ok(mz)
}

pub fn sys_munmap(_addr: *const usize, _length: usize) -> i32 {
	0
}

