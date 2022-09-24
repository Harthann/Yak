#[macro_export]
macro_rules! cli {
	() => {
		unsafe{ core::arch::asm!("cli") };
	}
}

#[macro_export]
macro_rules! sti {
	() => {
		unsafe{ core::arch::asm!("sti") };
	}
}

#[macro_export]
macro_rules! hlt {
	() => {
		unsafe{ core::arch::asm!("hlt") };
	}
}
