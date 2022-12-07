use crate::proc::process::{Process, Pid};
use crate::proc::task::{Task, TASKLIST};

use crate::memory::paging::page_directory;
use crate::memory::paging::page_directory::PageDirectory;

use crate::memory::allocator::Box;

pub fn sys_fork() -> Pid {
	//! Create a new process from the calling process,
	//! copy stack, heap and registers
	//!
	//! Heap contains the prg and the heap allocated
	unsafe {
		let running_task: &mut Task = Task::get_running_task();
		let parent: &mut Process = Process::get_running_process();
		let mut process: Process = Process::new();
		process.init(parent);
		parent.childs.push(Box::new(process));
		let process: &mut Process = parent.childs.last_mut().unwrap();
		let mut new_task: Task = Task::new();
		new_task.regs = running_task.regs;
		new_task.process = process;

		let page_dir: &mut PageDirectory = PageDirectory::new();
		page_dir.set_entry(0xb0000000 >> 22, get_paddr!(process.stack.offset));
		page_dir.set_entry(0x08000000 >> 22, get_paddr!(process.heap.offset));

		new_task.regs.cr3 = (page_dir as *mut _) as u32;

		TASKLIST.push(new_task);
		process.pid
	}
}
