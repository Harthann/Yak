use crate::io::{inb, outb, io_wait};

/* References: [https://wiki.osdev.org/8259_PIC] */

const PIC1: u16 = 0x20; /* io base addr for master PIC */
const PIC2: u16 = 0xa0; /* io base addr for slave PIC */
const PIC1_CMD: u16 = PIC1;
const PIC1_DATA: u16 = PIC1 + 1;
const PIC2_CMD: u16 = PIC2;
const PIC2_DATA: u16 = PIC2 + 1;

const PIC_EOI: u8 = 0x20; /* End of Interrupts command code */

pub fn end_of_interrupts(irq: usize) {
	if irq >= 8 { /* Slave interrupt request */
		outb(PIC2_CMD, PIC_EOI);
	}
	outb(PIC1_CMD, PIC_EOI);
}

const ICW1_ICW4: u8 = 0x01; /* ICW4 (not needed) */
const ICW1_SINGLE: u8 = 0x02; /* Single (cascade) mode */
const ICW1_INTERVAL4: u8 = 0x04; /* Call address interval 4 (8) */
const ICW1_LEVEL: u8 = 0x08; /* Level triggered (edge) mode */
const ICW1_INIT: u8 = 0x10; /* Initiliazation - required! */

const ICW4_8086: u8 = 0x01; /* 8086/88 (MCS-80/85) mode */
const ICW4_AUTO: u8 = 0x02; /* Auto (normal EOI) */
const ICW4_BUF_SLAVE: u8 = 0x08; /* Buffered mode/slave */
const ICW4_BUF_MASTER: u8 = 0x0c; /* Buffered mode/master */
const ICW4_SFNM: u8 = 0x10; /* Special fully nested (not) */

pub fn pic_remap(offset1: u8, offset2: u8) {
	/* save masks */
	let a1: u8 = inb(PIC1_DATA);
	let a2: u8 = inb(PIC2_DATA);

	outb(PIC1_CMD, ICW1_INIT | ICW1_ICW4);
	io_wait();
	outb(PIC2_CMD, ICW1_INIT | ICW1_ICW4);
	io_wait();

	outb(PIC1_DATA, offset1);
	io_wait();
	outb(PIC2_DATA, offset2);
	io_wait();

	outb(PIC1_DATA, 4);
	io_wait();
	outb(PIC2_DATA, 2);
	io_wait();

	outb(PIC1_DATA, ICW4_8086);
	io_wait();
	outb(PIC2_DATA, ICW4_8086);
	io_wait();

	/* restore masks */
	outb(PIC1_DATA, a1);
	outb(PIC2_DATA, a2);
}

pub fn irq_set_mask(mut irq: usize, port: u16)
{
	//let port: u16;
	let value: u8;

	//if irq < 8 {
	//	port = PIC1_DATA;
	//} else {
	//	port = PIC2_DATA;
	//	irq -= 8;
	//}
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

pub const PIC1_INTERRUPT: u8 = 0x20;
pub const PIC2_INTERRUPT: u8 = 0x28;

pub fn pic_set_interrupt_masks()
{
	//irq_set_mask((PIC1_INTERRUPT - PIC1_INTERRUPT) as usize);
//	irq_set_mask(0b00000000 as usize, PIC1_DATA);
	crate::kprintln!("PIC1: {:#b} {0}", inb(PIC1_DATA));
	crate::kprintln!("PIC2: {:#b} {0}", inb(PIC2_DATA));
	outb(PIC1_DATA, 0b001111000);
	outb(PIC2_DATA, 0b01111111);
//	irq_set_mask(0b00000000 as usize, PIC2_DATA);
	//irq_set_mask((PIC2_INTERRUPT - PIC1_INTERRUPT) as usize);
	unsafe{core::arch::asm!("sti")};
}

pub fn setup_pic8259() {
	pic_set_interrupt_masks();
	pic_remap(PIC1_INTERRUPT, PIC2_INTERRUPT);
}
