pub mod page_directory;
pub mod page_table;

use crate::kmemory;
use crate::multiboot;
use crate::multiboot::MemMapEntry;

#[allow(dead_code)]
extern "C" {
	pub static mut page_directory: PageDirectory;
}

use page_directory::PageDirectory;
use page_table::PageTable;

pub type VirtAddr = u32;
pub type PhysAddr = u32;

pub const PAGE_WRITABLE: u32 = 0b10;
pub const PAGE_USER: u32 = 0xb100;

/*
	Initiliaze the paging:
	- setup a page_table at the index 768 containing kernel code paddrs and
	page_directoy paddr
	- reset the initial page_table at index 0 and setup the page to index every
	page_tables in memory
	- initialize the bitmap of page tables
	- refresh tlb to clear the cache of the CPU
*/
pub fn init_paging() {
	unsafe {
		let pd_paddr: PhysAddr = (page_directory.get_vaddr() & 0x3ff000) as PhysAddr;
		let res = multiboot::get_last_entry();
		if res.is_ok() {
			let mmap_entry: &MemMapEntry = res.unwrap();
			/* Reserve space for kernel and SeaBIOS (GRUB) */
			kmemory::physmap_as_mut().claim_range(mmap_entry.baseaddr as PhysAddr, mmap_entry.length as usize / 4096);
		}
		kmemory::physmap_as_mut().claim_range(0x0, ((pd_paddr / 0x1000) + 1024) as usize);

		/* Init paging map */
		let kernel_pt_paddr: PhysAddr = pd_paddr + (768 + 1) * 0x1000;
		let handler_pt_paddr: PhysAddr = pd_paddr + (1023 + 1) * 0x1000;
		let init_pt_paddr: PhysAddr = pd_paddr + 0x1000;
		let mut init_page_tab: &mut PageTable = &mut *(init_pt_paddr as *mut _);
		init_page_tab.set_entry(768, kernel_pt_paddr | PAGE_WRITABLE | 1);
		init_page_tab.set_entry(1023, handler_pt_paddr | PAGE_WRITABLE | 1);
		crate::refresh_tlb!();
		let kernel_page_tab: &mut PageTable = &mut *(crate::get_vaddr!(0, 768) as *mut _);
		let mut handler_page_tab: &mut PageTable = &mut *(crate::get_vaddr!(0, 1023) as *mut _);
		kernel_page_tab.init();
		handler_page_tab.set_entry(0, init_pt_paddr | PAGE_WRITABLE | 1);
		handler_page_tab.set_entry(768, kernel_pt_paddr | PAGE_WRITABLE | 1);
		handler_page_tab.set_entry(1023, handler_pt_paddr | PAGE_WRITABLE | 1);
		page_directory.set_entry(0, 2);
		page_directory.set_entry(768, kernel_pt_paddr | PAGE_WRITABLE | 1);
		page_directory.set_entry(1023, handler_pt_paddr | PAGE_WRITABLE | 1);
		crate::refresh_tlb!();
		init_page_tab = page_directory.get_page_table(0);
		handler_page_tab = page_directory.get_page_table(1023);
		init_page_tab.clear();
		handler_page_tab.set_entry(0, 0);
		crate::refresh_tlb!();
	}
}

pub fn kalloc_pages(nb: usize, flags: u32) -> Result<VirtAddr, ()> {
	unsafe{Ok(page_directory.kget_page_frames(nb, flags)?)}
}

pub fn alloc_pages_at_addr(vaddr: VirtAddr, nb: usize, flags: u32) -> Result<VirtAddr, ()> {
	unsafe{Ok(page_directory.get_page_frames_at_addr(vaddr, nb, flags)?)}
}

pub fn kalloc_pages_at_addr(vaddr: VirtAddr, nb: usize, flags: u32) -> Result<VirtAddr, ()> {
	unsafe{Ok(page_directory.kget_page_frames_at_addr(vaddr, nb, flags)?)}
}

/* Allocate 'nb' page frames */
pub fn alloc_pages(nb: usize, flags: u32) -> Result<VirtAddr, ()> {
	unsafe{Ok(page_directory.get_page_frames(nb, flags)?)}
}

/* Allocate a page frame */
pub fn alloc_page(flags: u32) -> Result<VirtAddr, ()> {
	unsafe{Ok(page_directory.get_page_frame(flags)?)}
}

/* free multiple pages */
pub fn free_pages(vaddr: VirtAddr, nb: usize) {
	unsafe{page_directory.remove_page_frames(vaddr, nb)};
}

/* Free a page frame */
pub fn free_page(vaddr: VirtAddr) {
	unsafe{page_directory.remove_page_frame(vaddr)};
}

#[macro_export]
macro_rules! get_paddr {
	($vaddr:expr) =>
		(
			page_directory.get_page_table((($vaddr as usize) & 0xffc00000) >> 22).entries[(($vaddr as usize) & 0x3ff000) >> 12].get_paddr() + ((($vaddr as usize) & 0xfff) as crate::paging::PhysAddr)
		);
}

#[macro_export]
macro_rules! get_vaddr {
	($pd_index:expr, $pt_index:expr) =>
		(((($pd_index as usize) << 22) | (($pt_index as usize) << 12)) as crate::paging::VirtAddr);
}

#[macro_export]
macro_rules! refresh_tlb {
	() => (core::arch::asm!("mov eax, cr3",
		"mov cr3, eax"));
}

#[macro_export]
macro_rules! enable_paging {
	($page_directory:expr) => (core::arch::asm!("mov eax, {p}",
		"mov cr3, eax",
		"mov eax, cr0",
		"or eax, 0x80000001",
		"mov cr0, eax",
		p = in(reg) (&$page_directory as *const _) as usize););
}

#[macro_export]
macro_rules! disable_paging {
	() => (core::arch::asm!("mov ebx, cr0",
		"and ebx, ~(1 << 31)",
		"mov cr0, ebx"));
}
