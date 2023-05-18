use crate::proc::process::{Pid, Process};
use crate::proc::task::{Task, TASKLIST};

use crate::memory::paging::page_directory::PageDirectory;

use crate::wrappers::{_cli, _sti};

use crate::boxed::Box;

// kernel fork:
// same cr3:
// - Can't copy heap
// other cr3:
// => Copy heap
// => Copy stack
// must go to cr3 main kernel on sys
// must dump registers

/// Create a new process from the calling process,
/// copy stack, heap and registers
///
/// Heap contains the prg and the heap allocated
pub fn sys_fork() -> Pid {
	unsafe {
		_cli();
		let running_task: &mut Task = Task::get_running_task();
		let parent: &mut Process = Process::get_running_process();

		let mut process: Process = Process::new();
		process.init(parent);
		process.setup_kernel_stack(parent.kernel_stack.flags);
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
		parent.childs.push(Box::new(process));

		let process: &mut Process = parent.childs.last_mut().unwrap();
		let mut new_task: Task = Task::new();

		new_task.process = process;

		let page_dir: &mut PageDirectory = process.setup_pagination();

		new_task.regs = running_task.regs;
		new_task.regs.int_no = u32::MAX; // trigger for switch_task
		new_task.regs.cr3 = get_paddr!(page_dir as *const _);
		new_task.regs.eax = 0; // New forked process return 0

		TASKLIST.push_back(new_task);
		_sti();
		process.pid
	}
}
