pub static mut cli_count: usize = 0;

#[no_mangle]
pub extern "C" fn _cli() {
	unsafe {
		cli_count += 1;
		if cli_count == 1 {
			crate::cli!();
		}
	}
}

#[no_mangle]
pub extern "C" fn _sti() {
	unsafe {
		cli_count -= 1;
		if cli_count == 0 {
			crate::sti!();
		}
	}
}

#[macro_export]
macro_rules! cli {
	() => {
		unsafe{core::arch::asm!("cli")}
	}
}

#[macro_export]
macro_rules! sti {
	() => {
		unsafe{core::arch::asm!("sti")}
	}
}

#[macro_export]
macro_rules! hlt {
	() => {
		unsafe{core::arch::asm!("hlt")}
	}
}
