//  This module aim to parse mutliboot specification

use crate::{kprintln};

#[allow(dead_code)]
extern "C" {
	static multiboot_ptr:*const u8;
}

#[repr(C)]
pub struct TagHeader {
	pub htype:  u16,
	pub flags:  u16,
	pub size:   u32
}

#[repr(C)]
pub struct MemInfo {
	pub htype:	  u32,
	pub size:	   u32,
	pub mem_lower:  u32,
	pub mem_upper:  u32
}

#[repr(C)]
pub struct MemMapEntry {
	pub baseaddr:   u64,
	pub length:		u64,
	pub mtype:	  u32,
	pub reserved:   u32
}

#[repr(C)]
pub struct MemMap {
	pub htype:	  u32,
	pub size:	   u32,
	pub entry_size: u32,
	pub versions:   u32,
	pub entries:	[MemMapEntry;0]
}

#[repr(C)]
pub struct AddrTag {
	pub htype:	        u32,
	pub size:	        u32,
	pub header_addr:    u32,
	pub load_addr:      u32,
	pub load_end_addr:  u32,
	pub bss_end_addr:   u32,
}

pub fn read_tags() {
	unsafe {
		let mut ptr: *const u8 = multiboot_ptr.offset(8);
		let mut tag_ptr: *const TagHeader = ptr as *const TagHeader;

		while (*tag_ptr).size != 0 {
			match (*tag_ptr).htype {
				6 => {
					let mmap: *const MemMap = tag_ptr as *const MemMap;
					let entry_number: u32 = ((*mmap).size - 16) / (*mmap).entry_size as u32;
					let mut mmap_entry: *const MemMapEntry = (*mmap).entries.as_ptr();
					let mut i: u32 = 0;

					kprintln!("Memory Map");
					kprintln!("Number of entries: {} at {:#x}", entry_number, mmap_entry as u32);
					kprintln!("id |   Base addr   |   Length  | type | reserved");
					while i < entry_number {
						kprintln!("{:2} | {:#13x} | {:#9x} | {:4} | {:x}",
									i, (*mmap_entry).baseaddr, (*mmap_entry).length, (*mmap_entry).mtype, (*mmap_entry).reserved);
						mmap_entry = mmap_entry.add(1);
						i += 1;
					}
					break ;
				},
				2 => {
					let headers: &AddrTag = &*(tag_ptr as *const AddrTag);
					kprintln!("Header addr: {:#x}\nload_addr: {:#x}\nload_end_addr: {:#x}\nbss_end_addr: {:#x}", headers.header_addr, headers.load_addr, headers.load_end_addr, headers.bss_end_addr);
				},
				_ => {
					//kprintln!("Found tag: {}, size: {}", (*tag_ptr).htype, (*tag_ptr).size);
				 },
			}
			ptr = ptr.add((((*tag_ptr).size + 7) & !7) as usize);
			tag_ptr = ptr as *const TagHeader;
		}
	}
}

pub fn get_last_entry() -> Result<&'static MemMapEntry, ()> {
	unsafe {
		let mut mmap_entry: *const MemMapEntry;
		let mut ptr: *const u8 = multiboot_ptr.offset(8);
		let mut tag_ptr: *const TagHeader = ptr as *const TagHeader;

		while (*tag_ptr).size != 0 {
			if (*tag_ptr).htype == 6 {
				let mmap: *const MemMap = tag_ptr as *const MemMap;
				let entry_number: u32 = ((*mmap).size - 16) / (*mmap).entry_size as u32;
				mmap_entry = (*mmap).entries.as_ptr();
				let mut i: u32 = 0;

				while i < entry_number - 1 {
					mmap_entry = mmap_entry.add(1);
					i += 1;
				}
				return Ok(&*mmap_entry);
			}
			ptr = ptr.add((((*tag_ptr).size + 7) & !7) as usize);
			tag_ptr = ptr as *const TagHeader;
		}
		Err(())
	}
}
