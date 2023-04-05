use crate::proc::process::{Pid, Process};
use crate::proc::task::{Task, TASKLIST};

use crate::memory::paging::page_directory::PageDirectory;
use crate::memory::paging::page_table::PageTable;

use crate::wrappers::{_cli, _sti};

use crate::memory::paging::{PAGE_PRESENT, PAGE_WRITABLE, PAGE_USER};

use crate::memory::allocator::Box;

pub fn sys_fork() -> Pid {
	//! Create a new process from the calling process,
	//! copy stack, heap and registers
	//!
	//! Heap contains the prg and the heap allocated
	unsafe {
		_cli();
		let running_task: &mut Task = Task::get_running_task();
		let parent: &mut Process = Process::get_running_process();

		crate::kprintln!("parent pid: {}", parent.pid);
		let mut process: Process = Process::new();
		process.init(parent);
		process.setup_kernel_stack(
			parent.kernel_stack.size,
			parent.kernel_stack.flags,
			parent.kernel_stack.kphys
		);
		process.setup_stack(
			parent.stack.size,
			parent.stack.flags,
			parent.stack.kphys
		);
		process.setup_heap(
			parent.heap.size,
			parent.heap.flags,
			parent.heap.kphys
		);
		process.copy_mem(parent);
		crate::kprintln!("flags: {:#x?}", process.stack.flags);
		parent.childs.push(Box::new(process));

		let process: &mut Process = parent.childs.last_mut().unwrap();
		let mut new_task: Task = Task::new();

		new_task.process = process;

		let page_dir: &mut PageDirectory =
			process.setup_pagination(parent.stack.flags & PAGE_USER != 0);

		new_task.regs = running_task.regs;
		new_task.regs.int_no = u32::MAX; // trigger for switch_task
		new_task.regs.cr3 = get_paddr!(page_dir as *const _);
		new_task.regs.eax = 0; // New forked process return 0

		crate::kprintln!("new_task: {:#x?}", new_task.regs);

		TASKLIST.push(new_task);
		_sti();
		process.pid
	}
}
