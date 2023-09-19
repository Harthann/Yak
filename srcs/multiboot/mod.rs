//!  This module aim to parse mutliboot specification

use crate::memory::paging::bitmap;
use crate::memory::PhysAddr;
use crate::{kprint, kprintln};

use crate::boot::multiboot_ptr;

enum TagType {
	End            = 0,
	CmdLine        = 1,
	BootLoaderName = 2,
	Module         = 3,
	BasicMemInfo   = 4,
	BootDev        = 5,
	Mmap           = 6,
	Vbe            = 7,
	FrameBuffer    = 8,
	ElfSections    = 9,
	Apm            = 10,
	Efi32          = 11,
	Efi64          = 12,
	Smbios         = 13,
	AcpiOld        = 14,
	AcpiNew        = 15,
	Network        = 16,
	EfiMmap        = 17,
	EfiBs          = 18,
	Efi32Ih        = 19,
	Efi64Ih        = 20,
	LoadBaseAddr   = 21
}

#[repr(C)]
struct TagHeader {
	htype: u16,
	flags: u16,
	size:  u32
}

#[repr(C)]
struct MemInfo {
	htype:     u32,
	size:      u32,
	mem_lower: u32,
	mem_upper: u32
}

#[repr(C)]
struct BootDev {
	htype:         u32,
	size:          u32,
	biosdev:       u32,
	partition:     u32,
	sub_partition: u32
}

#[repr(C)]
struct MemMapEntry {
	baseaddr: u64,
	length:   u64,
	mtype:    u32,
	reserved: u32
}

#[repr(C)]
struct MemMap {
	htype:      u32,
	size:       u32,
	entry_size: u32,
	versions:   u32,
	entries:    [MemMapEntry; 0]
}

#[repr(C)]
struct FrameBufferInfo {
	htype:              u32,
	size:               u32,
	framebuffer_addr:   u64,
	framebuffer_pitch:  u32,
	framebuffer_width:  u32,
	framebuffer_height: u32,
	framebuffer_bpp:    u8,
	framebuffer_type:   u8,
	reserved:           u8
}

#[repr(C)]
struct ElfSymbols {
	htype:    u32,
	size:     u32,
	num:      u16,
	entsize:  u16,
	shndx:    u16,
	reserved: u16
}

#[repr(C)]
struct ApmTable {
	htype:       u32,
	size:        u32,
	version:     u16,
	cseg:        u16,
	offset:      u32,
	cseg_16:     u16,
	dseg:        u16,
	flags:       u16,
	cseg_len:    u16,
	cseg_16_len: u16,
	dseg_len:    u16
}

#[repr(C)]
struct LoadBasePhys {
	htype:          u32,
	size:           u32,
	load_base_addr: u32
}

pub unsafe fn claim_multiboot() {
	let mut ptr: *const u8 = (multiboot_ptr as *const u8).offset(8);
	let mut tag_ptr: *const TagHeader = ptr as *const TagHeader;

	while (*tag_ptr).size != 0 {
		match (*tag_ptr).htype {
			6 => {
				let mmap: *const MemMap = tag_ptr as *const MemMap;
				let entry_number: u32 =
					((*mmap).size - 16) / (*mmap).entry_size;
				let mut mmap_entry: *const MemMapEntry =
					(*mmap).entries.as_ptr();
				let mut i: u32 = 0;

				while i < entry_number {
					if (*mmap_entry).mtype == 2 {
						crate::dprintln!(
							"{} {}",
							(*mmap_entry).baseaddr as PhysAddr / 4096,
							(*mmap_entry).length as usize / 4096
						);
						// Force because it could be already mapped by BIOS
						bitmap::physmap_as_mut().force_claim_range(
							(*mmap_entry).baseaddr as PhysAddr,
							(*mmap_entry).length as usize / 4096
						);
					}
					mmap_entry = mmap_entry.add(1);
					i += 1;
				}
				break;
			},
			_ => {}
		}
		ptr = ptr.add((((*tag_ptr).size + 7) & !7) as usize);
		tag_ptr = ptr as *const TagHeader;
	}
}

pub fn read_tags() {
	unsafe {
		let mut ptr: *const u8 = (multiboot_ptr as *const u8).offset(8);
		let mut tag_ptr: *const TagHeader = ptr as *const TagHeader;

		while (*tag_ptr).size != 0 {
			match (*tag_ptr).htype {
				x if x as u32 == TagType::End as u32 => {
					break;
				},
				x if x as u32 == TagType::CmdLine as u32 => {
					kprint!("Command line = ");
					let cstr: &[u8] = core::slice::from_raw_parts(
						(tag_ptr as *const u8).offset(8),
						(*tag_ptr).size as usize - 8 - 1 /* remove size and '\0' */
					);
					match core::str::from_utf8(cstr) {
						Ok(string) => kprintln!("{}", string),
						Err(error) => kprintln!("[Error]: {}", error)
					}
				},
				x if x as u32 == TagType::BootLoaderName as u32 => {
					kprint!("Boot loader name = ");
					let cstr: &[u8] = core::slice::from_raw_parts(
						(tag_ptr as *const u8).offset(8),
						(*tag_ptr).size as usize - 8 - 1 /* remove size and '\0' */
					);
					match core::str::from_utf8(cstr) {
						Ok(string) => kprintln!("{}", string),
						Err(error) => kprintln!("[Error]: {}", error)
					}
				},
				x if x as u32 == TagType::BasicMemInfo as u32 => {
					let elem: &MemInfo = &*(tag_ptr as *const _);
					kprintln!(
						"mem_lower = {}KB, mem_upper: {}KB",
						elem.mem_lower,
						elem.mem_upper
					);
				},
				x if x as u32 == TagType::BootDev as u32 => {
					kprint!("Boot device ");
					let elem: &BootDev = &*(tag_ptr as *const _);
					kprintln!(
						"{:#x}, {}, {}",
						elem.biosdev,
						elem.partition,
						elem.sub_partition
					);
				},
				x if x as u32 == TagType::Mmap as u32 => {
					kprintln!("Memory map");
					let mmap: *const MemMap = tag_ptr as *const MemMap;
					let entry_number: u32 =
						((*mmap).size - 16) / (*mmap).entry_size;
					let mut mmap_entry: *const MemMapEntry =
						(*mmap).entries.as_ptr();
					let mut i: u32 = 0;

					kprintln!(
						"Number of entries: {} at {:#x}",
						entry_number,
						mmap_entry as u32
					);
					kprintln!(
						"id |   Base addr   |   Length  | type | reserved"
					);
					while i < entry_number {
						kprintln!(
							"{:2} | {:#13x} | {:#9x} | {:4} | {:x}",
							i,
							(*mmap_entry).baseaddr,
							(*mmap_entry).length,
							(*mmap_entry).mtype,
							(*mmap_entry).reserved
						);
						mmap_entry = mmap_entry.add(1);
						i += 1;
					}
				},
				x if x as u32 == TagType::FrameBuffer as u32 => {
					kprintln!("Framebuffer info");
					let elem: &FrameBufferInfo = &*(tag_ptr as *const _);
					kprintln!("  addr: {:#x}", elem.framebuffer_addr);
					kprintln!("  pitch: {}", elem.framebuffer_pitch);
					kprintln!("  width: {}", elem.framebuffer_width);
					kprintln!("  height: {}", elem.framebuffer_height);
					kprintln!("  bpp: {}", elem.framebuffer_bpp);
					kprintln!("  type: {}", elem.framebuffer_type);
					kprintln!("  reserved: {}", elem.reserved);
					// TODO: Add color_info ?
				},
				x if x as u32 == TagType::ElfSections as u32 => {
					kprintln!("-- skipped -- ELF Symbols");
					/*
					kprintln!("=> ELF Symbols");
					let elem: &ElfSymbols = &*(tag_ptr as *const _);
					kprintln!("    size: {}", elem.size);
					kprintln!("    num: {}", elem.num);
					kprintln!("    entsize: {}", elem.entsize);
					kprintln!("    shndx: {}", elem.shndx);
					kprintln!("    reserved: {}", elem.reserved);
					*/
				},
				x if x as u32 == TagType::Apm as u32 => {
					kprintln!("-- skipped -- APM table");
					/*
					kprintln!("=> APM table");
					let elem: &ApmTable = &*(tag_ptr as *const _);
					kprintln!("    version: {}", elem.version);
					kprintln!("    cseg: {}", elem.cseg);
					kprintln!("    offset: {:#x}", elem.offset);
					kprintln!("    cseg_16: {}", elem.cseg_16);
					kprintln!("    dseg: {}", elem.dseg);
					kprintln!("    flags: {}", elem.flags);
					kprintln!("    cseg_len: {}", elem.cseg_len);
					kprintln!("    cseg_16_len: {}", elem.cseg_16_len);
					kprintln!("    dseg_len: {}", elem.dseg_len);
					*/
				},
				x if x as u32 == TagType::AcpiOld as u32 => {
					kprintln!("-- skipped -- ACPI old RSDP");
				},
				x if x as u32 == TagType::LoadBaseAddr as u32 => {
					kprintln!("-- skipped -- Image load physical address");
					/*
					kprintln!("=> Image load base physical address");
					let elem: &LoadBasePhys = &*(tag_ptr as *const _);
					kprintln!("    load_base_addr: {:#x}", elem.load_base_addr);
					*/
				},
				_ => {
					kprintln!(
						"Found tag: {}, size: {}",
						(*tag_ptr).htype,
						(*tag_ptr).size
					);
				}
			}
			ptr = ptr.add((((*tag_ptr).size + 7) & !7) as usize);
			tag_ptr = ptr as *const TagHeader;
		}
	}
}
