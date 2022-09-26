use crate::io::{inb, outb, io_wait};

pub mod handlers;
pub mod pit;
pub use pit::set_pit;
pub use handlers::{handler, JIFFIES};
/* References: [https://wiki.osdev.org/8259_PIC] */

/*
**	PIC2 Interrupt		BIT	 ____________		PIC1 Interrupt		BIT	 ____________
** Real Time Clock -->	[0]	|			 |   Timer ------------->	[0]	|			 |
** ACPI ------------->	[1]	|            |   Keyboard----------->	[1]	|            |      _____
** Available -------->	[2]	| Secondary  |---------------------->	[2]	| Primary    |     |     |
** Available -------->	[3]	| Interrupt  |   Serial Port 2 ----->	[3]	| Interrupt  |---> | CPU |
** Mouse ------------>	[4]	| Controller |   Serial Port 1 ----->	[4]	| Controller |     |_____|
** Co-Processor ----->	[5]	|            |   Parallel Port 2/3 ->	[5]	|            |
** Primary ATA ------>	[6]	|            |   Floppy disk ------->	[6]	|            |
** Secondary ATA ---->	[7]	|____________|   Parallel Port 1---->	[7]	|____________|
*/

#[repr(u8)]
pub enum Pic1 {
	Timer			= 0x01,	// 0b0000 0001
	Keyboard		= 0x02,	// 0b0000 0010
	Slave			= 0x04,	// 0b0000 0100
	Serial1			= 0x08,	// 0b0000 1000
	Serial2			= 0x10,	// 0b0001 0000
	ParallelPort23	= 0x20,	// 0b0010 0000
	FloppyDisk		= 0x40,	// 0b0100 0000
	ParallelPort1	= 0x80,	// 0b1000 0000
}

#[repr(u8)]
pub enum Pic2 {
	RealTimeClock	= 0x01,	// 0b0000 0001
	ACPI			= 0x02,	// 0b0000 0010
	Available0		= 0x04,	// 0b0000 0100
	Available1		= 0x08,	// 0b0000 1000
	Mouse			= 0x10,	// 0b0001 0000
	CoProcessor		= 0x20,	// 0b0010 0000
	PrimaryATA		= 0x40,	// 0b0100 0000
	SecondaryATA	= 0x80,	// 0b1000 0000
}

const PIC1: u16 = 0x20; /* io base addr for master PIC */
const PIC2: u16 = 0xa0; /* io base addr for slave PIC */
const PIC1_CMD: u16 = PIC1;
const PIC1_DATA: u16 = PIC1 + 1;
const PIC2_CMD: u16 = PIC2;
const PIC2_DATA: u16 = PIC2 + 1;

pub const PIC1_IRQ_OFFSET:		u8 = 0x20;
pub const PIC1_IRQ_TIMER:		u8 = PIC1_IRQ_OFFSET + 0;
pub const PIC1_IRQ_KEYBOARD:	u8 = PIC1_IRQ_OFFSET + 1;
pub const PIC1_IRQ_SLAVE:		u8 = PIC1_IRQ_OFFSET + 2;
pub const PIC1_IRQ_SERIAL1:		u8 = PIC1_IRQ_OFFSET + 3;
pub const PIC1_IRQ_SERIAL2:		u8 = PIC1_IRQ_OFFSET + 4;
pub const PIC1_IRQ_PARA23:		u8 = PIC1_IRQ_OFFSET + 5;
pub const PIC1_IRQ_FLOPPY:		u8 = PIC1_IRQ_OFFSET + 6;
pub const PIC1_IRQ_PARA1:		u8 = PIC1_IRQ_OFFSET + 7;

pub const PIC2_IRQ_OFFSET:		u8 = 0x28;
pub const PIC2_IRQ_CLOCK:		u8 = PIC2_IRQ_OFFSET + 0;
pub const PIC2_IRQ_ACPI:		u8 = PIC2_IRQ_OFFSET + 1;
pub const PIC2_IRQ_AVAILABLE0:	u8 = PIC2_IRQ_OFFSET + 2;
pub const PIC2_IRQ_AVAILABLE1:	u8 = PIC2_IRQ_OFFSET + 3;
pub const PIC2_IRQ_MOUSE:		u8 = PIC2_IRQ_OFFSET + 4;
pub const PIC2_IRQ_COPROC:		u8 = PIC2_IRQ_OFFSET + 5;
pub const PIC2_IRQ_PRIMATA:		u8 = PIC2_IRQ_OFFSET + 6;
pub const PIC2_IRQ_SECATA:		u8 = PIC2_IRQ_OFFSET + 7;


const PIC_EOI: u8 = 0x20; /* End of Interrupts command code */

pub fn end_of_interrupts(irq: usize) {
	if irq >= 8 { /* Slave interrupt request */
		outb(PIC2_CMD, PIC_EOI);
	}
	outb(PIC1_CMD, PIC_EOI);
}

const ICW1_ICW4: u8 = 0x01;			/* ICW4 (not needed) */
const ICW1_SINGLE: u8 = 0x02;		/* Single (cascade) mode */
const ICW1_INTERVAL4: u8 = 0x04;	/* Call address interval 4 (8) */
const ICW1_LEVEL: u8 = 0x08;		/* Level triggered (edge) mode */
const ICW1_INIT: u8 = 0x10;			/* Initiliazation - required! */

const ICW4_8086: u8 = 0x01;			/* 8086/88 (MCS-80/85) mode */
const ICW4_AUTO: u8 = 0x02;			/* Auto (normal EOI) */
const ICW4_BUF_SLAVE: u8 = 0x08;	/* Buffered mode/slave */
const ICW4_BUF_MASTER: u8 = 0x0c;	/* Buffered mode/master */
const ICW4_SFNM: u8 = 0x10;			/* Special fully nested (not) */

pub fn pic_remap(offset1: u8, offset2: u8) {
	/* save masks */
	let a1: u8 = inb(PIC1_DATA);
	let a2: u8 = inb(PIC2_DATA);

/*	Init Control Word 1, tell to pic we start init process */
	outb(PIC1_CMD, ICW1_INIT | ICW1_ICW4);
	io_wait();
	outb(PIC2_CMD, ICW1_INIT | ICW1_ICW4);
	io_wait();

/*	Init Control Word 2, tell to pic offset of interrupt */
	outb(PIC1_DATA, offset1);
	io_wait();
	outb(PIC2_DATA, offset2);
	io_wait();

/*	Init Control Word 3, tell to pic who's master/slave */
	outb(PIC1_DATA, 4);
	io_wait();
	outb(PIC2_DATA, 2);
	io_wait();

/*	Init Control Word 4, tell to pic we use mode 8086 */
	outb(PIC1_DATA, ICW4_8086);
	io_wait();
	outb(PIC2_DATA, ICW4_8086);
	io_wait();

	/* restore masks */
	outb(PIC1_DATA, a1);
	outb(PIC2_DATA, a2);
}

pub fn irq_set_mask(mut irq: usize)
{
	let port: u16;
	let value: u8;

	if irq < 8 {
		port = PIC1_DATA;
	} else {
		port = PIC2_DATA;
		irq -= 8;
	}
	value = inb(port) | (1 << irq);
	outb(port, value);
}

pub fn irq_clear_mask(mut irq: usize)
{
	let port: u16;
	let value: u8;

	if irq < 8 {
		port = PIC1_DATA;
	} else {
		port = PIC2_DATA;
		irq -= 8;
	}
	value = inb(port) & !(1 << irq);
	outb(port, value);
}

pub fn pic_init_masks()
{
/* To activate specific irqs */
	let pic1mask: u8 =	Pic1::Timer as u8 |
						Pic1::Keyboard as u8 |
						Pic1::Slave as u8;

	let pic2mask: u8 = Pic2::RealTimeClock as u8;

/* We need the bit to be 0 to activate */
	outb(PIC1_DATA, 0xff ^ pic1mask);
	outb(PIC2_DATA, 0xff ^ pic2mask);

	unsafe{core::arch::asm!("sti")};
}

pub fn setup_pic8259() {
	pic_init_masks();
	pic_remap(PIC1_IRQ_OFFSET, PIC2_IRQ_OFFSET);
}
