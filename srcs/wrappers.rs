//! Wrappers for cli, sti and hlt asm instructions

#[no_mangle]
pub static mut cli_count: usize = 0;

#[naked]
#[no_mangle]
pub extern "C" fn _cli() {
	unsafe {
		core::arch::asm!("
		add dword ptr[cli_count], 1
		cmp dword ptr[cli_count], 1
		jne 1f
		cli
		1:
		ret",
		options(noreturn));
	}
}

#[naked]
#[no_mangle]
pub extern "C" fn _sti() {
	unsafe {
		core::arch::asm!("
		sub dword ptr[cli_count], 1
		cmp dword ptr[cli_count], 0
		jne 2f
		sti
		2:
		ret",
		options(noreturn));
	}
}

#[naked]
#[no_mangle]
pub extern "C" fn _rst() {
	unsafe {
		core::arch::asm!("
		mov dword ptr[cli_count], 0
		ret",
		options(noreturn));
	}
}

macro_rules! cli {
	() => {
		core::arch::asm!("cli")
	}
}

macro_rules! sti {
	() => {
		core::arch::asm!("sti")
	}
}

macro_rules! hlt {
	() => {
		core::arch::asm!("hlt")
	}
}

pub (crate) use {cli, sti, hlt};
