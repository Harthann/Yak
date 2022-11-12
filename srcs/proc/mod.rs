//! Processus, Tasks and Signals

use crate::memory::allocator::Box;
use crate::vec::Vec;
use crate::VirtAddr;
use crate::wrappers::{_cli, _sti, _rst};

pub mod task;
pub mod process;
pub mod signal;

use process::{Process, Pid, zombify_running_process};
use task::{Task, TASKLIST, switch_task};

pub type Id = i32;

#[no_mangle]
pub unsafe extern "C" fn _exit(status: i32) -> ! {
	_cli();
	zombify_running_process(status);
	TASKLIST.pop();
	let res = &TASKLIST.peek();
	if res.is_none() {
		todo!();
	}
	_rst();
	switch_task(&res.as_ref().unwrap().regs);
	/* Never goes there */
	loop {}
}

#[naked]
#[no_mangle]
pub unsafe extern "C" fn wrapper_fn() {
	core::arch::asm!("
	mov eax, [esp]
	add esp, 4
	call eax
	cli
	mov esp, STACK_TASK_SWITCH
	sub esp, 256
	push eax
	call _exit",
	options(noreturn));
	/* Never goes there */
}

pub unsafe extern "C" fn exec_fn(func: VirtAddr, args_size: &Vec<usize>, mut args: ...) -> Pid {
	_cli();
	let proc: Process =  Process::new();
	let res = TASKLIST.peek();
	if res.is_none() {
		todo!();
	}
	let running_task = res.unwrap();
	let parent: &mut Process = &mut *running_task.process;
	let childs: &mut Vec<Box<Process>> = &mut parent.childs;
	childs.push(Box::new(proc));
	let len = childs.len();
	let proc_ptr: *mut Process = childs[len - 1].as_mut();
	(*proc_ptr).init(&mut *parent, 0);
	let mut new_task: Task = Task::new();
	new_task.init(running_task.regs.eflags, running_task.regs.cr3, proc_ptr);
	/* init_fn_task - Can't move to another function ??*/
	let sum: usize = args_size.iter().sum();
	new_task.regs.esp -= sum as u32;
	let mut nb = 0;
	for arg in args_size.iter() {
		let mut n: usize = *arg;
		while n > 0 {
			if arg / 4 > 0 {
				core::arch::asm!("mov [{esp} + {nb}], eax",
					esp = in(reg) new_task.regs.esp,
					nb = in(reg) nb,
					in("eax") args.arg::<u32>());
				n -= 4;
				nb += 4;
			}
			else if arg / 2 > 0 {
				core::arch::asm!("mov [{esp} + {nb}], ax",
					esp = in(reg) new_task.regs.esp,
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
	new_task.regs.esp -= 4;
	core::arch::asm!("mov [{esp}], {func}",
		esp = in(reg) new_task.regs.esp,
		func = in(reg) func);
	TASKLIST.push(new_task);
	_sti();
	(*proc_ptr).pid
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
		{
			let mut args_size: crate::vec::Vec<usize> = crate::vec::Vec::new();
			crate::size_of_args!(args_size, $($rest),+);
			crate::proc::exec_fn($func, &args_size, $($rest),+)
		}
	}
}
