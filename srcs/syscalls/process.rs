use crate::proc::process::{Pid, Process, PROCESS_TREE};
use crate::proc::task::{Task, TASKLIST};

use crate::utils::arcm::KArcm;

use crate::memory::paging::page_directory::PageDirectory;

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
		let running_task: &mut Task = Task::get_running_task();
		let binding = Process::get_running_process();
		let mut process: Process = Process::new();
		let mut new_task: Task = Task::new();

		process.init(&binding);

		let pid = process.pid;
		{
			// lock parent in scope
			let mut parent = binding.lock();
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
			process.copy_mem(&mut parent);
		}

		let page_dir: &mut PageDirectory = process.setup_pagination();

		new_task.regs = running_task.regs;
		new_task.regs.int_no = u32::MAX; // trigger for switch_task
		new_task.regs.cr3 = get_paddr!(page_dir as *const _);
		new_task.regs.eax = 0; // New forked process return 0

		new_task.process = KArcm::new(process);

		let mut parent = binding.lock();
		parent.childs.push(new_task.process.clone());
		PROCESS_TREE.insert(pid, new_task.process.clone());

		TASKLIST.push_back(new_task);
		pid
	}
}
