use crate::memory::paging;

pub fn mmap(
	addr: *const usize,
	length: usize,
	prot: u32,
	flags: u32,
	fd: i32,
	offset: usize
) -> *const u8 {
	//! WARNING: Currently flags sended to MemoryZone are ignored and used to give PageDirectory
	//! the page flags. The flag PAGE_USER may overlap with one of mmap flags
	let mz = sys_mmap(
		addr as VirtAddr,
		length,
		prot,
		flags | paging::PAGE_USER,
		fd,
		offset
	);
	let offset = match mz {
		Ok(x) => x.lock().offset,
		Err(_) => 0xff
	};
	offset as *const u8
}

use crate::memory::{MemoryZone, TypeZone, VirtAddr};
use crate::proc::process::Process;
use crate::utils::arcm::Arcm;

/// hint: Adress at which the kernel will look for free space.
/// size; Size of the mapping, will be rounded up to a multiplpe of PAGE_SIZE
/// prot: Indicate protection settings to setup the LDT for the current process
/// flags: Flags of the memory zone that will be allocated
/// fd: FileDescriptor to the file that needs to be mapped
/// offset: offset on the file from which the mapping will start
pub fn sys_mmap(
	_hint: VirtAddr,
	size: usize,
	_prot: u32,
	flags: u32,
	_fd: i32,
	_offset: usize
) -> Result<Arcm<MemoryZone>, ()> {
	// Parameters given to init will vary depending on flags given
	let mz = Arcm::new(MemoryZone::init(TypeZone::Anon, size, flags, false));
	let binding = Process::get_running_process();
	binding.lock().add_memory_zone(mz.clone());
	Ok(mz)
}

/// Current implementation have a lot of unverified cases.
///
/// Current optimal use is to give munmap the real addr and length of a MemoryZone
/// else this is undefined behaviour.
pub fn sys_munmap(addr: *const usize, length: usize) -> i32 {
	let size = length + (4096 - (length % 4096));

	// Bind and lock the current process
	let binding = Process::get_running_process();
	let mut curr_process = binding.lock();

	// Look if addr given is in range of one MemoryZone
	// Current lookup does not handle if the given zone doesn't exist
	let mut index = 0;
	for i in &curr_process.mem_map {
		let (offset, size) = i.lock().area();
		if offset >= addr as VirtAddr
			&& (addr as usize) < offset as usize + size
		{
			break;
		}
		index += 1;
	}
	// Get back the concerned MemoryZone
	let mut split_list = curr_process.mem_map.split_off(index);
	let mz = split_list.pop_front();

	// Bind and lock the concerned MemoryZone
	let mzbinding = mz.unwrap();
	let mut guard = mzbinding.lock();

	// If given size is lower than the total size, we need to shrink the MemoryZone
	if size < guard.size {
		crate::dprintln!("Remapping zone");
		let (offset, size) = guard.area();
		// Expect may need to be replaced with proper error handling later
		// The remap function isn't yet implemented, this will result in a panic
		guard
			.remap(offset, size - length, crate::memory::MAP_FIXED)
			.expect("Remap failed");
		// Drop guard in order to get back the ownership over mzbinding
		drop(guard);
		// Add back the new MemoryZone to our list
		split_list.push_front(mzbinding);
	}

	curr_process.mem_map.append(&mut split_list);
	0
}
