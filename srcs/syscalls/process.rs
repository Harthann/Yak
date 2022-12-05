use crate::proc::process::{Process, Pid};
use crate::proc::task::{Task, TASKLIST};

use crate::memory::paging::page_directory;
use crate::memory::paging::page_directory::PageDirectory;

use crate::memory::allocator::Box;
use crate::vec::Vec;

use crate::get_paddr;

pub fn sys_fork() -> Pid {
	unsafe {
		let res = TASKLIST.peek();
		if res.is_none() {
			todo!();
		}
		let running_task = res.unwrap();
		let parent: &mut Process = &mut *running_task.process;
		let childs: &mut Vec<Box<Process>> = &mut parent.childs;
		childs.push(Box::new(Process::new()));
		let len = childs.len();
		let proc_ptr: *mut Process = childs[len - 1].as_mut();
		(*proc_ptr).init(&mut *parent);
		let mut new_task: Task = Task::new();
		new_task.regs = running_task.regs;
		new_task.process = proc_ptr;

		let page_dir: *mut PageDirectory = PageDirectory::new();
		(*page_dir).set_entry(0xb0000000 >> 22, get_paddr!((*proc_ptr).stack.offset));
		(*page_dir).set_entry(0x08000000 >> 22, get_paddr!((*proc_ptr).heap.offset));

		new_task.regs.cr3 = page_dir as u32;

		TASKLIST.push(new_task);
		(*proc_ptr).pid
	}
}
