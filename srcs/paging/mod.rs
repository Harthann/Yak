pub mod page_directory;
pub mod page_table;

#[allow(dead_code)]
extern "C" {
	pub static mut page_directory: PageDirectory;
}

const KERNEL_BASE: usize = 0xc0000000;

use page_directory::PageDirectory;
use page_table::PageTable;

pub type VirtAddr = u32;
pub type PhysAddr = u32;

pub fn init_paging() {
	unsafe {
		let pd_paddr: PhysAddr = (page_directory.get_vaddr() as usize - KERNEL_BASE) as PhysAddr;
		let pt_paddr: PhysAddr = pd_paddr + (768 + 1) * 0x1000;
		let page_dir: &mut PageDirectory = &mut *(pd_paddr as *mut _);
		let init_page_tab: &mut PageTable = &mut *((pd_paddr + 0x1000) as *mut _);
		init_page_tab.entries[1022] = ((pt_paddr | 3) as u32).into();
		let page_tab: &mut PageTable = &mut *(crate::get_vaddr!(0, 1022) as *mut _);
		page_tab.init(pt_paddr as usize);
		page_dir.entries[768] = ((pt_paddr | 3) as u32).into();
		page_dir.remove_page_table(0);
	}
}

#[macro_export]
macro_rules! get_paddr {
	($vaddr:expr) =>
		(
			page_directory.get_page_table(($vaddr & 0xffc00000) >> 22).entries[($vaddr & 0x3ff000) >> 12].get_paddr() + (($vaddr & 0xfff) as crate::paging::PhysAddr)
		);
}

#[macro_export]
macro_rules! get_vaddr {
	($pd_index:expr, $pt_index:expr) =>
		((($pd_index << 22) | ($pt_index << 12)) as VirtAddr);
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
