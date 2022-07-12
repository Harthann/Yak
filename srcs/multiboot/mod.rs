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
    htype:      u32,
    size:       u32,
    mem_lower:  u32,
    mem_upper:  u32
}

pub fn read_tags() {
	unsafe {
        let mut ptr: *const u8 = multiboot_ptr.offset(8);
        let mut tag_ptr: *const TagHeader = ptr as *const TagHeader;

		kprintln!("Multiboot ptr: {:#x}", multiboot_ptr as u32);
        while (*tag_ptr).size != 0 {
            //kprintln!("Type: {} | Size: {}", (*tag_ptr).htype, (*tag_ptr).size);
            match (*tag_ptr).htype {
                6 => kprintln!("Memory Map"),
                4 => kprintln!("Memory info:\nLower: {} | Upper: {}", (*(tag_ptr as *const MemInfo)).mem_lower, (*(tag_ptr as *const MemInfo)).mem_upper),
                _ => {},
            }


            ptr = ptr.add((*tag_ptr).size as usize);
            while *ptr == 0x0 {
                ptr = (ptr as *const u8).add(1);
            }
            tag_ptr = ptr as *const TagHeader;
        }

	}
}
