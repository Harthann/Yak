pub mod page_directory;
pub mod page_table;

const KERNEL_BASE: usize = 0xc0000000;

type phys_addr = u32;
type virt_addr = u32;

#[macro_export]
macro_rules! enable_paging {
	($page_directory:expr) => (unsafe{core::arch::asm!("mov eax, {p}",
		"mov cr3, eax",
		"mov eax, cr0",
		"or eax, 0x80000001",
		"mov cr0, eax",
		p = in(reg) (&$page_directory as *const _) as usize)};);
}
