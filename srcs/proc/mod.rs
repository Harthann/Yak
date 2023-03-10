//! Processus, Tasks and Signals

use crate::memory::allocator::Box;
use crate::vec::Vec;
use crate::{KSTACK_ADDR, VirtAddr};
use crate::wrappers::{_cli, _sti, _rst};

use crate::memory::paging::page_directory;
use crate::memory::paging::{PAGE_WRITABLE, PAGE_USER, PAGE_PRESENT};

pub mod task;
pub mod process;
pub mod signal;

#[cfg(test)]
pub mod test;

use process::{Process, Pid};
use task::{Task, TASKLIST, schedule_task};

use crate::syscalls::exit::{__W_EXITCODE};

pub type Id = i32;

#[no_mangle]
pub unsafe extern "C" fn _exit(status: i32) -> ! {
	_cli();
	let task: Task = TASKLIST.pop();
	(*task.process).zombify(__W_EXITCODE!(status as i32, 0));
	_rst();
	schedule_task();
	/* Never goes there */
}

#[naked]
#[no_mangle]
pub unsafe extern "C" fn wrapper_fn() {
	core::arch::asm!("
	mov eax, [esp]
	add esp, 4
	call eax
	mov ebx, eax
	mov eax, 1 // exit
	int 0x80",
	options(noreturn));
	/* Never goes there */
}

pub unsafe extern "C" fn exec_fn(func: VirtAddr, args_size: &Vec<usize>, mut args: ...) -> Pid {
	_cli();
	let running_task: &mut Task = Task::get_running_task();
	let parent: &mut Process = Process::get_running_process();

	let mut process = Process::new();
	process.init(parent);
	process.setup_kernel_stack(0x1000, parent.stack.flags, parent.stack.kphys);
	process.setup_stack(0x1000, parent.stack.flags, parent.stack.kphys);
	parent.childs.push(Box::new(process));
	let process: &mut Process = parent.childs.last_mut().unwrap();
	let mut new_task: Task = Task::new();
	new_task.init(running_task.regs, process);
	/* init_fn_task - Can't move to another function ??*/
	let sum: usize = args_size.iter().sum();
	new_task.regs.esp = (process.stack.offset + process.stack.size as u32) - sum as u32;
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
	process.pid
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
	($func:expr) => {
		{
			let args_size: crate::vec::Vec<usize> = crate::vec::Vec::new();
			crate::proc::exec_fn($func as u32, &args_size)
		}
	};
	($func:expr, $($rest:expr),+) => {
		{
			let mut args_size: crate::vec::Vec<usize> = crate::vec::Vec::new();
			crate::size_of_args!(args_size, $($rest),+);
			crate::proc::exec_fn($func as u32, &args_size, $($rest),+)
		}
	}
}

pub fn change_kernel_stack(addr: VirtAddr) {
	unsafe {
		page_directory.get_page_table(KSTACK_ADDR as usize >> 22).set_entry((KSTACK_ADDR as usize & 0x3ff000) >> 12, get_paddr!(addr) | PAGE_WRITABLE | PAGE_USER | PAGE_PRESENT);
	}
}
