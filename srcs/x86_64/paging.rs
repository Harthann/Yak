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
