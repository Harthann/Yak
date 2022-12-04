use crate::proc::process::{Process, Pid};
use crate::proc::task::TASKLIST;

use crate::memory::allocator::Box;
use crate::vec::Vec;

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
	}
//	parent.
	0
}
