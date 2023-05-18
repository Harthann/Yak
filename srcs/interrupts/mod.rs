//! Setup interrupts and exception handler

use crate::proc::task::Task;
use crate::syscalls::syscall_handler;

mod idt;
pub mod int;

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
	static irq_stub_0: u32;
}

static mut IDT: IDT = IDT {
	idt_entries: [InterruptDescriptor {
		offset_0:  0,
		selector:  0,
		zero:      0,
		type_attr: 0,
		offset_1:  0
	}; IDT_MAX_DESCRIPTORS],
	idtr:        IDTR { size: 0, offset: 0 }
};

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct Registers {
	pub ds:       u32,
	pub cr3:      u32,
	pub edi:      u32,
	pub esi:      u32,
	pub ebp:      u32,
	pub esp:      u32,
	pub ebx:      u32,
	pub edx:      u32,
	pub ecx:      u32,
	pub eax:      u32,
	pub int_no:   u32,
	pub err_code: u32,
	pub eip:      u32,
	pub cs:       u32,
	pub eflags:   u32,
	pub useresp:  u32,
	pub ss:       u32
}

impl Registers {
	pub const fn new() -> Self {
		Self {
			ds:       0,
			cr3:      0,
			edi:      0,
			esi:      0,
			ebp:      0,
			esp:      0,
			ebx:      0,
			edx:      0,
			ecx:      0,
			eax:      0,
			int_no:   0,
			err_code: 0,
			eip:      0,
			cs:       0,
			eflags:   0,
			useresp:  0,
			ss:       0
		}
	}
}

use crate::pic::{PIC1_IRQ_OFFSET, PIC2_IRQ_OFFSET};

fn page_fault_handler(reg: &Registers) {
	unsafe {
		let cr2: usize;
		core::arch::asm!("mov {}, cr2", out(reg) cr2);
		crate::kprintln!("at addr {:#x}", cr2);
	}
	crate::kprintln!("{:#x?}", reg);
}

use crate::wrappers::{_cli, _rst, hlt};

// [https://wiki.osdev.org/Interrupts_tutorial]
// TODO: lock mutex before write and int
#[no_mangle]
pub unsafe extern "C" fn exception_handler(regs: &mut Registers) {
	_cli();
	let task = Task::get_running_task();
	task.regs = *regs; // dump regs for syscall (e.g: fork)
	let int_no: usize = regs.int_no as usize;
	if int_no < EXCEPTION_SIZE && STR_EXCEPTION[int_no] != "Reserved" {
		crate::kprintln!(
			"\n{} exception (code: {}):",
			STR_EXCEPTION[int_no],
			int_no
		);
		match int_no {
			// TODO: enum exceptions
			14 => page_fault_handler(regs),
			_ => {
				crate::kprintln!("{:#x?}", regs);
			}
		}
		if int_no != 3 && int_no != 1 {
			// TODO: HOW TO GET IF IT'S A TRAP OR NOT
			hlt!();
		}
	} else if int_no == 0x80 {
		syscall_handler(regs);
	} else {
		if int_no < PIC1_IRQ_OFFSET as usize
			|| int_no > PIC2_IRQ_OFFSET as usize + 7
		{
			crate::kprintln!(
				"\nUnknown exception (code: {}):\n{:#x?}",
				int_no,
				regs
			);
			hlt!();
		} else {
			crate::pic::handler(regs, int_no);
		}
	}
	// Rust VecDeque seems to move reference when push/pop so we'll make a new one
	let task = Task::get_running_task();
	task.regs = *regs; // get back registers if updated by syscall (e.g: waitpid)
	task.regs.int_no = u32::MAX; // identifier for switch_task
	_rst();
}

pub unsafe fn init_idt() {
	let mut i;

	IDT.idtr.offset = (&IDT.idt_entries[0] as *const _) as u32;
	IDT.idtr.size =
		(core::mem::size_of::<IDTR>() * IDT_MAX_DESCRIPTORS - 1) as u16;
	i = 0;
	while i < IDT_SIZE {
		let offset: u32 = isr_stub_table[i];
		IDT.idt_entries[i].init(offset, GDT_OFFSET_KERNEL_CODE, 0x8e);
		i += 1;
	}

	// syscalls
	IDT.idt_entries[0x80].init(isr_stub_syscall, GDT_OFFSET_KERNEL_CODE, 0xee);
	IDT.idt_entries[32].init(irq_stub_0, GDT_OFFSET_KERNEL_CODE, 0x8e);
	core::arch::asm!("lidt [{}]", in(reg) (&IDT.idtr as *const _) as u32);
}

#[repr(C, align(16))]
struct IDT {
	pub idt_entries: [InterruptDescriptor; IDT_MAX_DESCRIPTORS],
	pub idtr:        IDTR
}

#[repr(packed)]
struct IDTR {
	pub size:   u16,
	pub offset: u32
}

#[repr(packed)]
#[derive(Copy, Clone)]
struct InterruptDescriptor {
	offset_0:  u16,
	selector:  u16,
	zero:      u8,
	type_attr: u8, // gate type, dpl and p fields
	offset_1:  u16
}

impl InterruptDescriptor {
	pub fn init(&mut self, offset: u32, select: u16, type_attr: u8) {
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
