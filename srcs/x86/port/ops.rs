pub trait PortRead {
	unsafe fn read_from_port(port: u16) -> Self;
}
pub trait PortWrite {
	unsafe fn write_to_port(port: u16, value: Self);
}

impl PortRead for u8 {
	unsafe fn read_from_port(port: u16) -> u8 {
		let mut value: u8;
		core::arch::asm!("in al, dx",
                         in("dx") port,
                         out("al") value,
                         options(nomem, nostack, preserves_flags));
		value
	}
}

impl PortRead for u16 {
	unsafe fn read_from_port(port: u16) -> u16 {
		let mut value: u16;
		core::arch::asm!("in ax, dx",
                         in("dx") port,
                         out("ax") value,
                         options(nomem, nostack, preserves_flags));
		value
	}
}

impl PortRead for u32 {
	unsafe fn read_from_port(port: u16) -> u32 {
		let mut value: u32;
		core::arch::asm!("in eax, dx",
                         in("dx") port,
                         out("eax") value,
                         options(nomem, nostack, preserves_flags));
		value
	}
}

impl PortWrite for u8 {
	unsafe fn write_to_port(port: u16, value: u8) {
		core::arch::asm!("out dx, al",
                         in("dx") port,
                         in("al") value,
                         options(nomem, nostack, preserves_flags));
	}
}

impl PortWrite for u16 {
	unsafe fn write_to_port(port: u16, value: u16) {
		core::arch::asm!("out dx, ax",
                         in("dx") port,
                         in("ax") value,
                         options(nomem, nostack, preserves_flags));
	}
}

impl PortWrite for u32 {
	unsafe fn write_to_port(port: u16, value: u32) {
		core::arch::asm!("out dx, eax",
                         in("dx") port,
                         in("eax") value,
                         options(nomem, nostack, preserves_flags));
	}
}
