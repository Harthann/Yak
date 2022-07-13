//  This module aim to parse mutliboot specification

use crate::{kprintln, hexdump};

#[allow(dead_code)]
extern "C" {
	static multiboot_ptr:*const u8;
}

#[repr(C)]
pub struct TagHeader {
	htype:  u32,
	size:   u32
}

#[repr(C)]
pub struct MemInfo {
	htype:	  u32,
	size:	   u32,
	mem_lower:  u32,
	mem_upper:  u32
}

#[repr(C)]
pub struct MemMapEntry {
	baseaddr:   u64,
	length:	    u64,
	mtype:	  u32,
	reserved:   u32
}

#[repr(C)]
pub struct MemMap {
	htype:	  u32,
	size:	   u32,
	entry_size: u32,
	versions:   u32,
	entries:	*const MemMapEntry

}

pub fn read_tags() {
	unsafe {
		let mut ptr: *const u8 = multiboot_ptr.offset(8);
		let mut tag_ptr: *const TagHeader = ptr as *const TagHeader;

		kprintln!("Multiboot ptr: {:#x}", multiboot_ptr as u32);
		while (*tag_ptr).size != 0 {
			match (*tag_ptr).htype {
				6 => {
                    let mmap: *const MemMap = tag_ptr as *const MemMap;
                    hexdump!(mmap as *const u8, (*mmap).size as usize);
                    let entry_number: u32 =  ((*mmap).size - 16) / (*mmap).entry_size as u32;
                    let mut mmap: *const MemMapEntry = mmap.add(16) as *const MemMapEntry;
                    let mut i: u32 = 0;

                    kprintln!("Memory Map");
                    kprintln!("Number of entries: {} at {:#x}", entry_number, mmap as u32);
                    kprintln!("id |   Base addr   |   Length  | type | reserved");
                    while i < entry_number {
                        kprintln!("{:2} | {:#x} | {:#x} | {:4} | {:x}",
                                    i, (*mmap).baseaddr, (*mmap).length, (*mmap).mtype, (*mmap).reserved);
                        mmap = mmap.add(1);
                        i += 1;
                    }
                    


                },
				_ => {},
			}


			ptr = ptr.add((((*tag_ptr).size + 7) & !7) as usize);
//			while *ptr == 0x0 {
//				ptr = (ptr as *const u8).add(1);
//			}
			tag_ptr = ptr as *const TagHeader;
		}

	}
}
