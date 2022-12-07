use crate::proc::process::{Process, Pid};
use crate::proc::task::{Task, TASKLIST};

use crate::memory::paging::page_directory;
use crate::memory::paging::page_directory::PageDirectory;

use crate::memory::allocator::Box;
use crate::vec::Vec;

use crate::get_paddr;

pub fn sys_fork() -> Pid {
	unsafe {
		let running_task: &mut Task = Task::get_running_task();
		let parent: &mut Process = Process::get_running_process();
		let mut process: Process = Process::new();
		process.init(parent);
		parent.childs.push(Box::new(process));
		let len = parent.childs.len();
		let process: &mut Process = parent.childs[len - 1].as_mut();
		let mut new_task: Task = Task::new();
		new_task.regs = running_task.regs;
		new_task.process = &mut *process;

		let page_dir: &mut PageDirectory = PageDirectory::new();
		page_dir.set_entry(0xb0000000 >> 22, get_paddr!(process.stack.offset));
		page_dir.set_entry(0x08000000 >> 22, get_paddr!(process.heap.offset));

		new_task.regs.cr3 = (page_dir as *mut _) as u32;

		TASKLIST.push(new_task);
		process.pid
	}
}
