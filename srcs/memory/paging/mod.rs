pub mod bitmap;
pub mod page_directory;
pub mod page_table;

use crate::boot::KERNEL_BASE;

use crate::memory::{PhysAddr, VirtAddr};
use crate::multiboot;

#[allow(dead_code)]
extern "C" {
	pub static mut page_directory: PageDirectory;
}

use page_directory::PageDirectory;
use page_table::PageTable;

pub const PAGE_GLOBAL: u32 = 0b100000000;
pub const PAGE_USER: u32 = 0b100;
pub const PAGE_WRITABLE: u32 = 0b10;
pub const PAGE_PRESENT: u32 = 0b1;

/// Initialize the paging:
/// + setup a page_table at the index 768 containing kernel code paddrs and
/// page_directoy paddr
/// + reset the initial page_table at index 0 and setup the page_directory
/// to index every page_tables in memory
/// + initialize the bitmap of page tables
/// + refresh tlb to clear the cache of the CPU
pub fn init_paging() {
	unsafe {
		let pd_paddr: PhysAddr =
			(page_directory.get_vaddr() & 0x3ff000) as PhysAddr;
		// Claim 1st MiB used by BIOS Real Mode
		bitmap::physmap_as_mut()
			.claim_range(0x0, (1024 * 1024) / 0x1000)
			.expect("Failed to claim BIOS Real Mode 1st MiB");
		// Claim all memory reserved by grub multiboot
		multiboot::claim_multiboot();
		// Claim page_directory
		bitmap::physmap_as_mut()
			.claim_range(0x100000, ((pd_paddr / 0x1000) + 1024) as usize)
			.expect("Failed to claim code pages");

		// Init paging map
		let kernel_pt_paddr: PhysAddr = bitmap::physmap_as_mut()
			.get_page()
			.expect("Failed to get kernel page table");
		// Use identity mapping to setup kernel page
		let init_pt_paddr: PhysAddr = pd_paddr + 0x1000;
		let init_page_tab: &mut PageTable = &mut *(init_pt_paddr as *mut _);
		init_page_tab.set_entry(
			KERNEL_BASE >> 22,
			kernel_pt_paddr | PAGE_WRITABLE | PAGE_PRESENT
		);
		refresh_tlb!();
		// Final mapping
		let kernel_page_tab: &mut PageTable =
			&mut *(get_vaddr!(0, KERNEL_BASE >> 22) as *mut _);
		kernel_page_tab.init();
		page_directory.set_entry(
			KERNEL_BASE >> 22,
			kernel_pt_paddr | PAGE_WRITABLE | PAGE_PRESENT
		);
		// Recursive mapping
		page_directory.set_entry(1023, pd_paddr | PAGE_WRITABLE | PAGE_PRESENT);
		// Remove init page
		init_page_tab.clear();
		page_directory.set_entry(0, 0);
		refresh_tlb!();
	}
}

pub fn kalloc_pages(nb: usize, flags: u32) -> Result<VirtAddr, ()> {
	unsafe { page_directory.kget_page_frames(nb, flags) }
}

pub fn alloc_pages_at_addr(
	vaddr: VirtAddr,
	nb: usize,
	flags: u32
) -> Result<VirtAddr, ()> {
	unsafe { page_directory.get_page_frames_at_addr(vaddr, nb, flags) }
}

pub fn kalloc_pages_at_addr(
	vaddr: VirtAddr,
	nb: usize,
	flags: u32
) -> Result<VirtAddr, ()> {
	unsafe { page_directory.kget_page_frames_at_addr(vaddr, nb, flags) }
}

// Allocate 'nb' page frames
pub fn alloc_pages(nb: usize, flags: u32) -> Result<VirtAddr, ()> {
	unsafe { page_directory.get_page_frames(nb, flags) }
}

// Allocate a page frame
pub fn alloc_page(flags: u32) -> Result<VirtAddr, ()> {
	unsafe { page_directory.get_page_frame(flags) }
}

// free multiple pages
pub fn free_pages(vaddr: VirtAddr, nb: usize) {
	unsafe { page_directory.remove_page_frames(vaddr, nb) };
}

// Free a page frame
pub fn free_page(vaddr: VirtAddr) {
	unsafe { page_directory.remove_page_frame(vaddr) };
}

macro_rules! get_paddr {
	($vaddr:expr) => {
		crate::memory::paging::page_directory
			.get_page_table(($vaddr as usize) >> 22)
			.entries[(($vaddr as usize) & 0x3ff000) >> 12]
			.get_paddr()
			+ ((($vaddr as usize) & 0xfff) as crate::memory::PhysAddr)
	};
}

macro_rules! get_vaddr {
	($pd_index:expr, $pt_index:expr) => {
		((($pd_index as usize) << 22) | (($pt_index as usize) << 12))
			as crate::memory::VirtAddr
	};
}

macro_rules! refresh_tlb {
	() => {
		core::arch::asm!("push eax", "mov eax, cr3", "mov cr3, eax", "pop eax")
	};
}

#[allow(unused)]
macro_rules! enable_paging {
	($page_directory:expr) => (core::arch::asm!("mov eax, {p}",
		"mov cr3, eax",
		"mov eax, cr0",
		"or eax, 0x80000001",
		"mov cr0, eax",
		p = in(reg) (&$page_directory as *const _) as usize););
}

#[allow(unused)]
macro_rules! disable_paging {
	() => {
		core::arch::asm!("mov ebx, cr0", "and ebx, ~(1 << 31)", "mov cr0, ebx")
	};
}

pub(crate) use {get_paddr, get_vaddr, refresh_tlb};
