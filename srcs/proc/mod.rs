use crate::memory::allocator::Box;
use crate::vec::Vec;
use crate::VirtAddr;

pub mod task;
pub mod process;
pub mod signal;

use process::{Process, zombify_running_process};
use task::{Task, RUNNING_TASK, STACK_TASK_SWITCH, append_task, remove_running_task};

pub type Id = i32;

#[no_mangle]
pub unsafe extern "C" fn exit_fn() -> ! {
	core::arch::asm!("mov esp, {}", in(reg) (STACK_TASK_SWITCH - 256));
	zombify_running_process();
	remove_running_task();
	/* Never goes there */
}

#[naked]
#[no_mangle]
pub unsafe extern "C" fn wrapper_fn() {
	core::arch::asm!("
	mov eax, [esp]
	add esp, 4
	call eax
	cli
	jmp exit_fn",
	options(noreturn));
	/* Never goes there */
}

pub unsafe extern "C" fn exec_fn(func: VirtAddr, args_size: &Vec<usize>, mut args: ...) {
	core::arch::asm!("cli");
	let proc: Process =  Process::new();
	let parent: &mut Process = &mut *(*RUNNING_TASK).process;
	let childs: &mut Vec<Box<Process>> = &mut parent.childs;
	childs.push(Box::new(proc));
	let len = childs.len();
	let proc_ptr: *mut Process = childs[len - 1].as_mut();
	(*proc_ptr).init(&mut *parent, 0);
	let mut other_task: Task = Task::new();
	other_task.init((*RUNNING_TASK).regs.eflags, (*RUNNING_TASK).regs.cr3, proc_ptr);
	/* init_fn_task - Can't move to another function ??*/
	let sum: usize = args_size.iter().sum();
	other_task.regs.esp -= sum as u32;
	let mut nb = 0;
	for arg in args_size.iter() {
		let mut n: usize = *arg;
		while n > 0 {
			if arg / 4 > 0 {
				core::arch::asm!("mov [{esp} + {nb}], eax",
					esp = in(reg) other_task.regs.esp,
					nb = in(reg) nb,
					in("eax") args.arg::<u32>());
				n -= 4;
				nb += 4;
			}
			else if arg / 2 > 0 {
				core::arch::asm!("mov [{esp} + {nb}], ax",
					esp = in(reg) other_task.regs.esp,
					nb = in(reg) nb,
					in("ax") args.arg::<u16>());
				n -= 2;
				nb += 2;
			} else {
				todo!();
			}
		}
	}
	/* call function to wrapper_fn */
	other_task.regs.esp -= 4;
	core::arch::asm!("mov [{esp}], {func}",
		esp = in(reg) other_task.regs.esp,
		func = in(reg) func);
	append_task(other_task);
}

#[macro_export]
macro_rules! size_of_args {
	($vector:expr, $name:expr) => { $vector.push(core::mem::size_of_val(&$name)); };
	($vector: expr, $x:expr, $($rest:expr),+) => {
		crate::size_of_args!($vector, $x); crate::size_of_args!($vector, $($rest),+)
	}
}

#[macro_export]
macro_rules! exec_fn {
	($func:expr, $($rest:expr),+) => {
		let mut args_size: crate::vec::Vec<usize> = crate::vec::Vec::new();
		crate::size_of_args!(args_size, $($rest),+);
		crate::proc::exec_fn($func, &args_size, $($rest),+);
	}
}

