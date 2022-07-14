pub mod page_directory;
pub mod page_table;

#[allow(dead_code)]
extern "C" {
	pub static mut page_directory: PageDirectory;
}

const KERNEL_BASE: usize = 0xc0000000;

use page_directory::PageDirectory;
use page_table::PageTable;

type VirtAddr = u32;
type PhysAddr = u32;

pub fn init_paging() {
	unsafe {
		let paddr: usize = page_directory.get_vaddr() as usize - KERNEL_BASE;
		let page_dir: &mut PageDirectory = &mut *(paddr as *mut _);
		let init_page_tab: &mut PageTable = &mut *((paddr + 0x1000) as *mut _);
		init_page_tab.entries[1022] = (((paddr + (768 + 1) * 0x1000) | 3) as u32).into();
		let page_tab: &mut PageTable = &mut *(crate::get_vaddr!(0, 1022) as *mut _);
		page_tab.init(paddr + (768 + 1) * 0x1000);
		page_dir.entries[768] = (((paddr + (768 + 1) * 0x1000) | 3) as u32).into();
		page_dir.remove_page_table(0);
	}
}

/*
#[macro_export]
macro_rules! get_paddr {
	($vaddr:expr) =>
		(PAGE_DIRECTORY.entries[$vaddr & 0xfff00000]);
}
*/

#[macro_export]
macro_rules! get_vaddr {
	($pd_index:expr, $pt_index:expr) =>
		(($pd_index << 22 | $pt_index << 12) as VirtAddr);
}

#[macro_export]
macro_rules! enable_paging {
	($page_directory:expr) => (unsafe{core::arch::asm!("mov eax, {p}",
		"mov cr3, eax",
		"mov eax, cr0",
		"or eax, 0x80000001",
		"mov cr0, eax",
		p = in(reg) (&$page_directory as *const _) as usize)};);
}
