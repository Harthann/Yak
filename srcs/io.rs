use core::arch::asm;

pub fn outb(port: u16, cmd: u8) {
	unsafe {
		asm!("out dx, al",
		in("dx") port,
		in("al") cmd);
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
