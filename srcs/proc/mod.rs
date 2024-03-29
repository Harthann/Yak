//! Processus, Tasks and Signals

use crate::alloc::string::ToString;
use crate::utils::arcm::KArcm;
use crate::vec::Vec;
use crate::wrappers::{_cli, _rst};
use crate::{VirtAddr, KSTACK_ADDR};
use alloc::sync::Arc;
use core::ffi::CStr;

use crate::memory::paging::PAGE_WRITABLE;

use crate::memory::paging::page_directory;

pub mod process;
pub mod signal;
pub mod task;

#[cfg(test)]
pub mod test;

use process::{Pid, Process, PROCESS_TREE};
use task::{schedule_task, Task, TASKLIST};

use crate::syscalls::exit::__W_EXITCODE;

pub type Id = i32;

#[no_mangle]
pub unsafe extern "C" fn _exit(status: i32) -> ! {
	_cli();
	{
		let task: Task = TASKLIST.pop_front().unwrap();
		let pid = task.process.lock().pid;
		Process::zombify(pid, __W_EXITCODE!(status as i32, 0));
	}
	_rst();
	schedule_task()
	// Never goes there
}

#[naked]
#[no_mangle]
pub unsafe extern "C" fn wrapper_fn(fn_addr: VirtAddr) {
	core::arch::asm!(
		"mov eax, [esp + 4]",
		"add esp, 8",
		"sti",
		"call eax",
		"sub esp, 8",
		"push eax",
		"call _exit",
		options(noreturn)
	);
	// Never goes there
}

pub unsafe extern "C" fn exec_fn(
	name: *const u8,
	func: VirtAddr,
	args_size: &Vec<usize>,
	mut args: ...
) -> Pid {
	let running_task: &mut Task = Task::get_running_task();
	let binding = Process::get_running_process();
	let mut process = Process::new();
	let mut new_task: Task = Task::new();

	process.init(&binding);
	process.exe = CStr::from_ptr(name as *const i8)
		.to_str()
		.unwrap()
		.to_string();
	let mut parent = binding.lock();

	let pid = process.pid;
	process.setup_kernel_stack(parent.kernel_stack.flags); // not needed
	process.setup_stack(
		parent.stack.size,
		parent.stack.flags,
		parent.stack.kphys
	);
	// Copying all open fd from parent. Should not copy 0 and 1 but create new one instead
	for i in 0..process::MAX_FD {
		process.fds[i] = match &parent.fds[i] {
			Some(fd) => Some(Arc::clone(&fd)),
			None => None
		};
	}

	// init_fn_task - Can't move to another function ??
	let sum: usize = args_size.iter().sum();
	new_task.regs.esp =
		(process.stack.offset + process.stack.size as u32) - (sum + 4) as u32;
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
			} else if arg / 2 > 0 {
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
	// call function to wrapper_fn
	new_task.regs.esp -= 4;
	core::arch::asm!("mov [{esp}], {func}",
		esp = in(reg) new_task.regs.esp,
		func = in(reg) func);
	new_task.regs.esp -= 4;
	new_task.regs.eip = wrapper_fn as VirtAddr;
	new_task.regs.cr3 = running_task.regs.cr3;
	new_task.regs.ds = running_task.regs.ds;

	new_task.process = KArcm::new(process);
	parent.childs.push(new_task.process.clone());
	PROCESS_TREE.insert(pid, new_task.process.clone());

	TASKLIST.push_back(new_task);
	pid
}

#[macro_export]
macro_rules! size_of_args {
	($vector:expr, $name:expr) => { $vector.push(core::mem::size_of_val(&$name)); };
	($vector: expr, $x:expr, $($rest:expr),+) => {
		crate::size_of_args!($vector, $x); crate::size_of_args!($vector, $($rest),+)
	}
}

#[macro_export]
macro_rules! exec_fn_name {
	($name:expr, $func:expr) => {
		{
			let args_size: crate::vec::Vec<usize> = crate::vec::Vec::new();
			$name.push_str("\0");
			crate::proc::exec_fn($name.as_ptr(), $func as u32, &args_size)
		}
	};
	($name:expr, $func:expr, $($rest:expr),+) => {
		{
			let mut args_size: crate::vec::Vec<usize> = crate::vec::Vec::new();
			crate::size_of_args!(args_size, $($rest),+);
			$name.push_str("\0");
			crate::proc::exec_fn($name.as_ptr(), $func as u32, &args_size, $($rest),+)
		}
	}
}

#[macro_export]
macro_rules! exec_fn {
	($func:expr) => {
		{
			let args_size: crate::vec::Vec<usize> = crate::vec::Vec::new();
			crate::proc::exec_fn(concat!(stringify!($func), "\0").as_ptr(), $func as u32, &args_size)
		}
	};
	($func:expr, $($rest:expr),+) => {
		{
			let mut args_size: crate::vec::Vec<usize> = crate::vec::Vec::new();
			crate::size_of_args!(args_size, $($rest),+);
			crate::proc::exec_fn(concat!(stringify!($func), "\0").as_ptr(), $func as u32, &args_size, $($rest),+)
		}
	}
}

// don't refresh tlb - let it for switch_task
#[inline(always)]
pub fn change_kernel_stack(process: &Process) {
	unsafe {
		page_directory
			.get_page_table((KSTACK_ADDR as usize) >> 22)
			.new_index_frame(
				((KSTACK_ADDR as usize) & 0x3ff000) >> 12,
				get_paddr!(process.kernel_stack.offset as u32),
				PAGE_WRITABLE
			);
		refresh_tlb!();
	}
}
