use crate::syscalls::syscall_handler;

const GDT_OFFSET_KERNEL_CODE: u16 = 0x08;
const IDT_SIZE: usize = 48;
const IDT_MAX_DESCRIPTORS: usize = 256;

const EXCEPTION_SIZE: usize = 32;
const STR_EXCEPTION: [&'static str; EXCEPTION_SIZE] = [
	"Divide-by-zero",
	"Debug",
	"Non-maskable Interrupt",
	"Breakpoint",
	"Overflow",
	"Bound Range Exceeded",
	"Invalid Opcode",
	"Device Not Available",
	"Double Fault",
	"Coprocessor Segment Overrun",
	"Invalid TSS",
	"Segment Not Present",
	"Stack-Segment Fault",
	"General Protection Fault",
	"Page Fault",
	"Reserved",
	"x87 Floating-Point Exception",
	"Alignment Check",
	"Machine Check",
	"SIMD Floating-Point Exception",
	"Virtualization Exception",
	"Control Protection Exception",
	"Reserved",
	"Reserved",
	"Reserved",
	"Reserved",
	"Reserved",
	"Reserved",
	"Hypervisor Injection Exception",
	"VMM Communication Exception",
	"Security Exception",
	"Reserved"
];

extern "C" {
	static mut isr_stub_table: [u32; IDT_SIZE];
	static isr_stub_syscall: u32;
}

static mut IDT: IDT = IDT {
	idt_entries: [InterruptDescriptor {
					offset_0: 0,
					selector: 0,
					zero: 0,
					type_attr: 0,
					offset_1: 0
				}; IDT_MAX_DESCRIPTORS],
	idtr: IDTR {
		size: 0,
		offset: 0
	}
};


#[derive(Debug, Copy, Clone)]
#[repr(C, packed)]
pub struct Registers {
	ds:			u32,
	edi:		u32,
	esi:		u32,
	ebp:		u32,
	esp:		u32,
	ebx:		u32,
	edx:		u32,
	ecx:		u32,
	eax:		u32,
	int_no:		u32,
	err_code:	u32,
	eip:		u32,
	cs:			u32,
	eflags:		u32,
	useresp:	u32,
	ss:			u32
}

use crate::pic::{
PIC1_IRQ_OFFSET,
PIC2_IRQ_OFFSET
};

/* [https://wiki.osdev.org/Interrupts_tutorial]*/
/* TODO: lock mutex before write and int */
#[no_mangle]
pub extern "C" fn exception_handler(reg: Registers) {
	let int_no: usize = reg.int_no as usize;
//	crate::kprintln!("{int_no}");
	if int_no < EXCEPTION_SIZE && STR_EXCEPTION[int_no] != "Reserved" {
		crate::kprintln!("\n{} exception (code: {}):\n{:#x?}", STR_EXCEPTION[int_no], int_no, reg);
		if int_no != 3 { /* breakpoint */
			unsafe{core::arch::asm!("hlt")};
		}
	} else if int_no == 0x80 {
		syscall_handler(reg);
	} else {
		if int_no < PIC1_IRQ_OFFSET as usize || int_no > PIC2_IRQ_OFFSET as usize + 7 {
			crate::kprintln!("\nUnknown exception (code: {}):\n{:#x?}", int_no, reg);
			unsafe{core::arch::asm!("hlt")};
		} else {
			crate::pic::handler(reg, int_no);
		}
	}
}

pub unsafe fn init_idt() {
	let mut i;

	IDT.idtr.offset = (&IDT.idt_entries[0] as *const _) as u32;
	IDT.idtr.size = (core::mem::size_of::<IDTR>() * IDT_MAX_DESCRIPTORS - 1) as u16;
	i = 0;
	while i < IDT_SIZE {
		let offset: u32 = isr_stub_table[i];
		IDT.idt_entries[i].init(offset, GDT_OFFSET_KERNEL_CODE, 0x8e);
		i += 1;
	}

	/* syscalls */
	IDT.idt_entries[0x80].init(isr_stub_syscall, GDT_OFFSET_KERNEL_CODE, 0xee);
	core::arch::asm!("lidt [{}]", in(reg) (&IDT.idtr as *const _) as u32);
}

#[repr(C, align(16))]
struct IDT {
	pub idt_entries: [InterruptDescriptor; IDT_MAX_DESCRIPTORS],
	pub idtr: IDTR
}

#[repr(packed)]
struct IDTR {
	pub size:			u16,
	pub offset:			u32
}

#[repr(packed)]
#[derive(Copy, Clone)]
struct InterruptDescriptor {
	offset_0:		u16,
	selector:		u16,
	zero:			u8,
	type_attr:		u8, /* gate type, dpl and p fields */
	offset_1:		u16
}

impl InterruptDescriptor {
	pub fn init(&mut self, offset: u32, select: u16, type_attr: u8)
	{
		self.set_offset(offset);
		self.selector = select;
		self.zero = 0;
		self.type_attr = type_attr;
	}

	#[inline]
	pub fn set_offset(&mut self, offset: u32) {
		self.offset_0 = (offset & 0x0000ffff) as u16;
		self.offset_1 = ((offset & 0xffff0000) >> 16) as u16;
	}

	#[inline]
	pub fn set_gate_type(&mut self, gate_type: u8) {
		self.type_attr &= 0b11110000;
		self.type_attr |= gate_type & 0b00001111;
	}

	#[inline]
	pub fn set_dpl(&mut self, dpl: u8) {
		self.type_attr &= 0b10011111;
		self.type_attr |= (dpl << 5) & 0b01100000;
	}

	#[inline]
	pub fn set_p(&mut self, p: u8) {
		self.type_attr &= 0b10000000;
		self.type_attr |= (p << 7) & 0b10000000;
	}

	#[inline]
	pub fn get_offset(&self) -> u32 {
		((self.offset_1 as u32) << 16) | self.offset_0 as u32
	}

	#[inline]
	pub fn get_selector(&self) -> u16 {
		self.selector
	}

	#[inline]
	pub fn get_gate_type(&self) -> u8 {
		self.type_attr & 0b00001111
	}

	#[inline]
	pub fn get_dpl(&self) -> u8 {
		(self.type_attr & 0b01100000) >> 5
	}

	#[inline]
	pub fn get_p(&self) -> u8 {
		(self.type_attr & 0b10000000) >> 7
	}
}

#[naked]
#[no_mangle]
unsafe extern "C" fn isr_common_stub() {
	core::arch::asm!("
	pusha

	mov ax, ds					// Lower 16-bits of eax = ds.
	push eax					// save the data segment descriptor

	mov ax, 0x10				// load the kernel data segment descriptor
	mov ds, ax
	mov es, ax
	mov fs, ax
	mov gs, ax

	call exception_handler

	pop eax						// reload the original data segment descriptor
	mov ds, ax
	mov es, ax
	mov fs, ax
	mov gs, ax

	add esp, 8
	popa
	iretd",
	options(noreturn));
}
