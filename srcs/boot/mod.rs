//! Define boot and kernel entrypoint

use core::mem::size_of;

use crate::reload_cs;

/// Base address for higher half kernel
pub const KERNEL_BASE: usize = 0xc0000000;
/// Kernel init stack size at boot
const STACK_SIZE: usize = 8192;

/// Multiboot 1.6 Header
#[repr(packed)]
struct multiboot_header {
	magic:         u32,
	architecture:  u32,
	header_length: u32,
	checksum:      u32,
	r#type:        u16,
	flags:         u16,
	size:          u32
}

/// Multiboot magic number
const MULTIBOOT_MAGIC: u32 = 0xe85250d6;
/// Arch for multiboot: protected mode
const ARCH: u32 = 0;

impl multiboot_header {
	const fn new() -> Self {
		Self {
			magic:         MULTIBOOT_MAGIC,
			architecture:  ARCH,
			header_length: size_of::<multiboot_header>() as u32,
			checksum:      (u32::MAX
				- (MULTIBOOT_MAGIC
					+ ARCH + size_of::<multiboot_header>() as u32))
				+ 1,
			r#type:        0,
			flags:         0,
			size:          8
		}
	}
}

#[no_mangle]
#[link_section = ".multiboot_header"]
/// multiboot header to define entrypoint inside linker script
static header: multiboot_header = multiboot_header::new();

#[no_mangle]
#[link_section = ".boot"]
/// Kernel entrypoint
///
/// The _start function begin from low address space and will setup:
/// - multiboot pointer
/// - basic stack
/// - pagination (identity paging and higher half kernel paging)
/// - load gdt
///
/// The function will then jump to higher half kernel after loading gdt
unsafe fn _start() {
	core::arch::asm!(
		"mov eax, offset multiboot_ptr - {0}", // Get multiboot struct from GRUB
		"mov DWORD PTR[eax], ebx",
		"mov esp, offset stack - {0} + {1}", // Stack pointer initialisation
		const KERNEL_BASE,
		const STACK_SIZE,
	);
	setup_page_directory!();
	setup_page_table!();
	enable_paging!();
	core::arch::asm!(
		"mov eax, offset gdt_desc",
		"lgdt [eax]", // load gdt
		"jmp 0x08, offset high_kernel", // long jump to higher half kernel
		options(noreturn)
	);
}

#[no_mangle]
/// Higher half kernel entrypoint
///
/// This is the first function called in higher half kernel
/// The function is called from _start entrypoint and will finish the gdt loading
/// Then it will setup stack and call the kinit function
unsafe fn high_kernel() {
	reload_cs!();
	core::arch::asm!(
		"mov esp, offset stack + {}",
		"call kinit",
		"hlt",
		const STACK_SIZE,
		options(noreturn)
	);
}

#[no_mangle]
#[link_section = ".data"]
/// Address to the multiboot structure used by GRUB
static multiboot_ptr: u32 = 0;

#[no_mangle]
#[link_section = ".bss"]
/// Kernel stack
static mut stack: [u8; STACK_SIZE] = [0; STACK_SIZE];

#[no_mangle]
#[link_section = ".padata"]
/// Kernel page directory
pub static mut page_directory: [u32; 1024] = [0x2; 1024];

#[no_mangle]
#[link_section = ".padata"]
/// Page table containing kernel code
static mut page_table: [u32; 1024] = [0; 1024];

/// Setup page tables in `page_directory`
///
/// The macro will setup a identity mapping and high `page_table` for higher
/// half kernel
macro_rules! setup_page_directory {
	() => {
		core::arch::asm!(
			"mov eax, offset page_table - {0} | 3",
			"mov ebx, offset page_directory - {0}",
			"mov DWORD PTR[ebx], eax",
			"mov DWORD PTR[ebx + 768 * 4], eax",
			const $crate::boot::KERNEL_BASE,
		);
	}
}

/// Setup kernel page table
///
/// The macro will setup all `page_table` entries to present and writable with
/// kernel code physical address
macro_rules! setup_page_table {
	() => {
		core::arch::asm!(
			"mov eax, 0x0",
			"mov ebx, 0x0",
			"2:",
			"mov ecx, ebx",
			"or ecx, 3",
			"mov DWORD PTR[page_table - {} + eax * 4], ecx",
			"add ebx, 0x1000",
			"inc eax",
			"cmp eax, 1024",
			"je 2f",
			"jmp 2b",
			"2:",
			const $crate::boot::KERNEL_BASE,
		);
	};
}

/// Enable paging by loading `page_directory` into `cr3`
macro_rules! enable_paging {
	() => {
		core::arch::asm!(
			"mov eax, offset page_directory - {}",
			"mov cr3, eax",
			"mov eax, cr0",
			"or eax, 0x80010000",
			"mov cr0, eax",
			const $crate::boot::KERNEL_BASE,
		);
	};
}

use {setup_page_directory, setup_page_table, enable_paging};
