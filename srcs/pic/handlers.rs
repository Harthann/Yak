use crate::interrupts::Registers;
use crate::pic::PIC1_IRQ_OFFSET;

#[allow(unused)]
pub fn handler(reg: &Registers, int_no: usize) {
	if crate::keyboard::keyboard_event() {
		if let Some(event) = crate::keyboard::handle_event() {
			match &mut *crate::cli::INPUT_BUFFER.lock() {
				Some(buffer) => buffer.push(event),
				None => {}
			}
		}
		// vga_buffer::clihandle!(charcode);
	}
	crate::pic::end_of_interrupts(int_no - PIC1_IRQ_OFFSET as usize);
}

#[naked]
#[no_mangle]
pub unsafe extern "C" fn irq_0() {
	#[cfg(not(feature = "multitasking"))]
	core::arch::asm!(
		"
		cli

        pusha

        add dword ptr[JIFFIES], 1
        // call end_of_interrupt(0);

        push 0
        call end_of_interrupts
        add esp, 4

        popa

        iretd
    ",
		options(noreturn)
	);

	#[cfg(feature = "multitasking")]
	core::arch::asm!(
		"
	cli
	push 0
	push -1

	jmp swap_task
	",
		options(noreturn)
	);
}
