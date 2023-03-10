use crate::pic::{
    PIC1_IRQ_OFFSET,
};
use crate::interrupts::Registers;
use crate::vga_buffer;

#[no_mangle]
pub static mut JIFFIES: usize = 0;

#[allow(unused)]
pub fn handler(reg: &Registers, int_no: usize) {
	if crate::keyboard::keyboard_event() {
		let charcode = crate::keyboard::handle_event();
		vga_buffer::clihandle!(charcode);
	}
	crate::pic::end_of_interrupts(int_no - PIC1_IRQ_OFFSET as usize);
}

extern "C" {
	 fn swap_task();
}

#[naked]
#[no_mangle]
unsafe extern "C" fn irq_0() {
    #[cfg(not(feature = "multitasking"))]
    core::arch::asm!("
		cli

        pusha

        add dword ptr[JIFFIES], 1
        // call end_of_interrupt(0);

        push 0
        call end_of_interrupts
        add esp, 4

        popa

		sti
        iretd
    ",
    options(noreturn));

    #[cfg(feature = "multitasking")]
    core::arch::asm!("
	cli
	push 0
	push -1

	jmp swap_task
	", options(noreturn));
}
