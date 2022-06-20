use core::arch::asm;

pub fn outb(port: u16, cmd: u8) {
	unsafe {
		asm!("out dx, al",
		in("dx") port,
		in("al") cmd);
	}
}

pub fn outw(port: u16, cmd: u16) {
	unsafe {
		asm!("out dx, ax",
		in("dx") port,
		in("ax") cmd);
	}
}

pub fn outl(port: u16, cmd: u32) {
	unsafe {
		asm!("out dx, eax",
		in("dx") port,
		in("eax") cmd);
	}
}

pub fn inb(port: u16) -> u8 {
	let mut input_byte: u8;
	unsafe {
		asm!("in al, dx",
		in("dx") port,
		out("al") input_byte);
	}
	input_byte
}

pub fn inw(port: u16) -> u16 {
	let mut input_byte: u16;
	unsafe {
		asm!("in ax, dx",
		in("dx") port,
		out("ax") input_byte);
	}
	input_byte
}

pub fn inl(port: u16) -> u32 {
	let mut input_byte: u32;
	unsafe {
		asm!("in eax, dx",
		in("dx") port,
		out("eax") input_byte);
	}
	input_byte
}
