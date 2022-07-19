pub mod page_directory;
pub mod page_table;

use crate::kmemory;
use crate::multiboot;
use crate::multiboot::MemMapEntry;

#[allow(dead_code)]
extern "C" {
	pub static mut page_directory: PageDirectory;
}

const KERNEL_BASE: usize = 0xc0000000;

use page_directory::PageDirectory;
use page_table::PageTable;

pub type VirtAddr = u32;
pub type PhysAddr = u32;

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
		let pd_paddr: PhysAddr = page_directory.get_vaddr() - KERNEL_BASE as PhysAddr;
		let mmap_entry:&'static MemMapEntry = multiboot::get_last_entry();
		/* Reserve space for kernel and SeaBIOS (GRUB) */
		kmemory::physmap_as_mut().claim_range(mmap_entry.baseaddr as PhysAddr, mmap_entry.length as usize / 4096);
		kmemory::physmap_as_mut().claim_range(0x0, ((pd_paddr / 0x1000) + 1024) as usize);

		/* Init paging map */
		let pt_paddr: PhysAddr = pd_paddr + (768 + 1) * 0x1000;
		let ipt_paddr: PhysAddr = pd_paddr + 0x1000;
		let init_page_tab: &mut PageTable = &mut *(ipt_paddr as *mut _);
		init_page_tab.set_entry(768, pt_paddr | 3);
		crate::refresh_tlb!();
		let page_tab: &mut PageTable = page_directory.get_page_table(768);
		page_tab.init();
		page_directory.set_entry(768, pt_paddr | 3);
		init_page_tab.clear();
		init_page_tab.set_entry(0, (pd_paddr + 0x1000) | 3);
		init_page_tab.set_entry(768, pt_paddr | 3);
		crate::refresh_tlb!();
	}
}

/* Allocate 'nb' page frames */
pub fn alloc_pages(nb: usize) -> Result<VirtAddr, ()> {
	unsafe{Ok(page_directory.get_page_frames(nb)?)}
}

/* TODO: free nb page frames */

/* Allocate a page frame */
pub fn alloc_page() -> Result<VirtAddr, ()> {
	unsafe{Ok(page_directory.get_page_frame()?)}
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
