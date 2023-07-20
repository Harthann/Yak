use crate::memory::paging;
use crate::memory::paging::page_table::PageTable;
use crate::memory::paging::{page_directory, PAGE_PRESENT, PAGE_WRITABLE};

#[repr(C)]
#[derive(Debug)]
pub struct mmap_arg {
	addr:   usize,
	length: usize,
	prot:   u32,
	flags:  u32,
	fd:     i32,
	offset: usize
}

/// Translate VirtAddr from userspace to kernelspace
/// This can be optimize a lot.
pub fn translate_vaddr(addr: VirtAddr) -> VirtAddr {
	unsafe {
		let binding = Process::get_running_process();
		let curr_process = binding.lock();

		if curr_process.owner == 0 {
			return addr;
		}

		let pd_index = addr as usize >> 22;
		let pt_index = (addr as usize & 0x3ff000) >> 12;
		let pt_paddr = (*curr_process.pd).get_entry(pd_index).get_paddr();
		for i in &curr_process.page_tables {
			if get_paddr!(i.get_vaddr()) == pt_paddr {
				let page_paddr = i.entries[pt_index].get_paddr();
				for j in 0..1024 {
					if page_directory.get_entry(j).get_present() == 1 {
						for k in 0..1024 {
							let pt = page_directory.get_page_table(j);
							if pt.entries[k].get_paddr() == page_paddr {
								crate::dprintln!(
									"Found: {:#x} {} {}",
									get_vaddr!(j, k),
									j,
									k
								);
								return get_vaddr!(j, k);
							}
						}
					}
				}
				return 0x0;
			}
		}
		0x0
	}
}

pub fn mmap(addr: *const mmap_arg) -> *const u8 {
	//! WARNING: Currently flags sended to MemoryZone are ignored and used to give PageDirectory
	//! the page flags. The flag PAGE_USER may overlap with one of mmap flags

	let translated_addr = translate_vaddr(addr as VirtAddr);
	let translated_addr = (translated_addr as usize) + (addr as usize & 0xfff);

	let ptr = unsafe { &*(translated_addr as *const mmap_arg) };
	let mz = sys_mmap(
		ptr.addr as VirtAddr,
		ptr.length,
		ptr.prot,
		ptr.flags,
		ptr.fd,
		ptr.offset
	);
	let size = ptr.length + (4096 - (ptr.length % 4096));
	let binding = Process::get_running_process();
	let mut curr_process = binding.lock();

	let user_pd = unsafe { &mut *curr_process.pd };

	if mz.is_err() {
		return 0xff as *const u8;
	}
	let mz = mz.unwrap();
	let offset = mz.lock().offset;
	if user_pd.get_entry(991).get_present() == 1 {
		let pt_paddr = user_pd.get_entry(991).get_paddr();
		for i in &mut curr_process.page_tables {
			if pt_paddr == unsafe { get_paddr!(i.get_vaddr()) } {
				let pt_index = i.new_frames(
					unsafe { get_paddr!(offset) },
					(size / 4096) as u32,
					ptr.flags | paging::PAGE_USER
				);
				return match pt_index {
					Ok(index) => get_vaddr!(991, index as usize) as *const u8,
					Err(_) => 0xff as *const u8
				};
			}
		}
		// try map in pt
		// if fail try next
		todo!()
	} else {
		let pt: &'static mut PageTable = PageTable::new();
		unsafe {
			let pt_index = pt
				.new_frames(
					get_paddr!(offset),
					(size / 4096) as u32,
					ptr.flags | paging::PAGE_USER
				)
				.expect("Mmap failed to create first PageTable");
			user_pd.set_entry(
				991,
				get_paddr!(pt as *const _)
					| PAGE_WRITABLE | PAGE_PRESENT
					| paging::PAGE_USER
			);
			curr_process.page_tables.push(pt);
			get_vaddr!(991, pt_index) as *const u8
		}
	}
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

	let translated_addr = translate_vaddr(addr as VirtAddr);
	let translated_addr = (translated_addr as usize) + (addr as usize & 0xfff);

	// Bind and lock the current process
	let binding = Process::get_running_process();
	let mut curr_process = binding.lock();

	// Look if addr given is in range of one MemoryZone
	// Current lookup does not handle if the given zone doesn't exist
	let mut index = 0;
	for i in &curr_process.mem_map {
		let (offset, size) = i.lock().area();
		if offset >= translated_addr as VirtAddr
			&& translated_addr < offset as usize + size
		{
			break;
		}
		index += 1;
	}
	// Get back the concerned MemoryZone
	let mut split_list = curr_process.mem_map.split_off(index);
	let mz = split_list.pop_front();

	// Bind and lock the concerned MemoryZone
	// This cause panic if we try to unmap non mapped MemoryZone
	let mzbinding = mz.unwrap();
	let mut guard = mzbinding.lock();

	// If given size is lower than the total size, we need to shrink the MemoryZone
	if size < guard.size {
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
