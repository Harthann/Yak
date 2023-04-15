//! Wrappers for cli, sti and hlt asm instructions

#[no_mangle]
pub static mut cli_count: usize = 0;

#[inline(always)]
pub fn _cli() {
	unsafe {
		core::arch::asm!(
			"cmp dword ptr[cli_count], 0",
			"jne 1f",
			"cli",
			"1:",
			"add dword ptr[cli_count], 1",
		);
	}
}

#[inline(always)]
pub fn _sti() {
	unsafe {
		core::arch::asm!(
			"sub dword ptr[cli_count], 1",
			"cmp dword ptr[cli_count], 0",
			"jne 2f",
			"sti",
			"2:",
		);
	}
}

#[inline(always)]
pub fn _rst() {
	unsafe {
		core::arch::asm!("mov dword ptr[cli_count], 0");
	}
}

macro_rules! cli {
	() => {
		core::arch::asm!("cli")
	};
}

macro_rules! sti {
	() => {
		core::arch::asm!("sti")
	};
}

macro_rules! hlt {
	() => {
		core::arch::asm!("hlt")
	};
}

pub(crate) use {cli, hlt, sti};
