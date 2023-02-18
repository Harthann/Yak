use crate::pic::{
    PIC1_IRQ_OFFSET,
};
use crate::interrupts::Registers;

#[no_mangle]
pub static mut JIFFIES: usize = 0;

#[allow(unused)]
pub fn handler(reg: &Registers, int_no: usize) {
    if crate::keyboard::keyboard_event() {
        let charcode = crate::keyboard::handle_event();
        crate::clihandle!(charcode);
    }
    crate::pic::end_of_interrupts(int_no - PIC1_IRQ_OFFSET as usize);
}

#[naked]
#[no_mangle]
unsafe extern "C" fn irq_0() {
    #[cfg(not(feature = "multitasking"))]
    core::arch::asm!("
        cli
        add dword ptr[JIFFIES], 1
        sti
        iretd
    ", options(noreturn));

    #[cfg(feature = "multitasking")]
    core::arch::asm!("
    //; iretd allow to return directly after the interrupts
    //;iretd


    cli
    push 0  //; err_code
    push -1 //; int_no

    pusha

    mov eax, cr3
    push eax

    xor eax, eax
    mov ax, ds
    push eax

    add dword ptr[JIFFIES], 1

    mov dx, 0x20
    mov al, 0x20
    out dx, al

    mov eax, esp

    // Setup temp task
    mov esp, dword ptr[STACK_TASK_SWITCH]

    // (regs: &mut Registers)
    push eax

    call next_task
//	; Never goes there
    ", options(noreturn));
}

