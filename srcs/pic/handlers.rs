use crate::pic::{
PIC1_IRQ_OFFSET,
};
use crate::interrupts::Registers;

#[no_mangle]
pub static mut JIFFIES: usize = 0;

#[allow(unused)]
pub fn handler(reg: &Registers, int_no: usize) {

	//if int_no == PIC1_IRQ_TIMER as usize { 
	//	unsafe{ JIFFIES += 1 };
	//} else  {
	if crate::keyboard::keyboard_event() {
		let charcode = crate::keyboard::handle_event();
		crate::clihandle!(charcode);
	}
//	}
	crate::pic::end_of_interrupts(int_no - PIC1_IRQ_OFFSET as usize);
}

