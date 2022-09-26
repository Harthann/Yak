use crate::interrupts::Registers;

pub fn syscall_handler(reg: Registers)
{
	crate::kprintln!("Syscall: {:#x?}", reg);
}
