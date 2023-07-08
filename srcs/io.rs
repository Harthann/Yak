//! Wrappers for in and out asm instructions

use core::arch::asm;


// Notes:
//
// Those are rust function instead of naked because the compiler does a great job
// and inline them inside functions. The only thing we should care about is to
// preserve registers that should be by convention (otherwise there are scratch
// registers).
//
// x86 preserved registers are: ebx, esi, edi, ebp
//

#[allow(dead_code)]
pub fn io_wait() {
	outb(0x80, 0);
}

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

pub fn outsb(port: u16, src: *const u8, count: u32) {
	unsafe {
		asm!(
			"push esi",
			"mov esi, {esi}",
			"rep outsb",
			"pop esi",
			in("ecx") count,
			esi = in(reg) src,
			in("dx") port
		);
	}
}

pub fn outsw(port: u16, src: *const u16, count: u32) {
	unsafe {
		asm!(
			"push esi",
			"mov esi, {esi}",
			"rep outsw",
			"pop esi",
			in("ecx") count,
			esi = in(reg) src,
			in("dx") port
		);
	}
}

pub fn outsl(port: u16, src: *const u32, count: u32) {
	unsafe {
		asm!(
			"push esi",
			"mov esi, {esi}",
			"rep outsd",
			"pop esi",
			in("ecx") count,
			esi = in(reg) src,
			in("dx") port
		);
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

pub fn insb(port: u16, dst: *mut u8, count: u32) {
	unsafe {
		asm!(
			"push edi",
			"mov edi, {edi}",
			"rep insb",
			"pop edi",
			in("ecx") count,
			edi = in(reg) dst,
			in("dx") port
		);
	}
}

pub fn insw(port: u16, dst: *mut u16, count: u32) {
	unsafe {
		asm!(
			"push edi",
			"mov edi, {edi}",
			"rep insw",
			"pop edi",
			in("ecx") count,
			edi = in(reg) dst,
			in("dx") port
		);
	}
}

pub fn insl(port: u16, dst: *mut u32, count: u32) {
	unsafe {
		asm!(
			"push edi",
			"mov edi, {edi}",
			"rep insd",
			"pop edi",
			in("ecx") count,
			edi = in(reg) dst,
			in("dx") port
		);
	}
}
