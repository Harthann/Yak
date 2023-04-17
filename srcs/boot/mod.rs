use core::mem::size_of;

pub const KERNEL_BASE: usize = 0xc0000000;

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

const MULTIBOOT_MAGIC: u32 = 0xe85250d6;
const ARCH: u32 = 0; // protected mode

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
static header: multiboot_header = multiboot_header::new();

#[naked]
#[no_mangle]
#[link_section = ".boot"]
pub unsafe extern "C" fn _start() {
	core::arch::asm!(
		"mov eax, offset multiboot_ptr - {0}",
		"mov DWORD PTR[eax], ebx",
		"mov esp, offset stack - {0} + {1}",
		// setup page_directory entries
		"mov eax, offset page_table - {0} | 3",
		"mov ebx, offset page_directory - {0}",
		"mov DWORD PTR[ebx], eax",
		"mov DWORD PTR[ebx + 768 * 4], eax",
		// setup page_table entries
		"mov eax, 0x0", 
		"mov ebx, 0x0",
		"2:",
		"mov ecx, ebx",
		"or ecx, 3",
		"mov DWORD PTR[page_table - {0} + eax * 4], ecx",
		"add ebx, 0x1000",
		"inc eax",
		"cmp eax, 1024",
		"je 2f",
		"jmp 2b",
		"2:",
		// enable paging
		"mov eax, offset page_directory - {0}",
		"mov cr3, eax",
		"mov eax, cr0",
		"or eax, 0x80010001",
		"mov cr0, eax",
		// load gdt
		"mov eax, offset gdt_desc",
		"lgdt [eax]",
		"jmp lol",
		const KERNEL_BASE,
		const STACK_SIZE,
		options(noreturn)
	);
}

#[no_mangle]
#[naked]
#[link_section = ".boot"]
unsafe fn lol() {
	core::arch::asm!("ljmp $0x08, $high_kernel", options(noreturn, att_syntax));
}

#[no_mangle]
#[naked]
unsafe fn high_kernel() {
	core::arch::asm!(
		// reload cs
		"mov ax, 0x10",
		"mov ds, ax",
		"mov es, ax",
		"mov fs, ax",
		"mov gs, ax",
		"mov ss, ax",
		"mov esp, offset stack + {0}",
		"call kinit",
		"hlt",
		const STACK_SIZE,
		options(noreturn)
	);
}

#[no_mangle]
#[link_section = ".data"]
static multiboot_ptr: u32 = 0;

const STACK_SIZE: usize = 8192;

#[no_mangle]
#[link_section = ".bss"]
static mut stack: [u8; STACK_SIZE] = [0; STACK_SIZE];

#[no_mangle]
#[link_section = ".padata"]
pub static mut page_directory: [u32; 1024] = [0x2; 1024];

#[no_mangle]
#[link_section = ".padata"]
static mut page_table: [u32; 1024] = [0; 1024];
