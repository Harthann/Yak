use crate::proc::process::{Pid, Process};
use crate::proc::task::{Task, TASKLIST};

use crate::memory::paging::page_directory::PageDirectory;
use crate::memory::paging::page_table::PageTable;

use crate::memory::paging::{PAGE_PRESENT, PAGE_WRITABLE};

use crate::boxed::Box;

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
		process.copy_mem(parent);
		parent.childs.push(Box::new(process));
		let process: &mut Process = parent.childs.last_mut().unwrap();
		let mut new_task: Task = Task::new();
		new_task.regs = running_task.regs;
		new_task.process = process;

		let page_dir: &mut PageDirectory = PageDirectory::new();
		let handler_page_tab: &mut PageTable = PageTable::new();
		// Reference page table
		handler_page_tab.set_entry(
			0x0800000 >> 22,
			get_paddr!(process.heap.offset) | PAGE_WRITABLE | PAGE_PRESENT
		);
		handler_page_tab.set_entry(
			0xb000000 >> 22,
			get_paddr!(process.stack.offset) | PAGE_WRITABLE | PAGE_PRESENT
		);
		handler_page_tab.set_entry(
			1023,
			get_paddr!(handler_page_tab as *const _)
				| PAGE_WRITABLE | PAGE_PRESENT
		);
		// Setup heap + prg
		page_dir.set_entry(
			0x08000000 >> 22,
			get_paddr!(process.heap.offset) | PAGE_WRITABLE | PAGE_PRESENT
		);
		// Setup stack
		page_dir.set_entry(
			0xb0000000 >> 22,
			get_paddr!(process.stack.offset) | PAGE_WRITABLE | PAGE_PRESENT
		);
		page_dir.set_entry(
			1023,
			get_paddr!(handler_page_tab as *const _)
				| PAGE_WRITABLE | PAGE_PRESENT
		);

		new_task.regs.cr3 = (page_dir as *mut _) as u32;
		new_task.regs.eax = 0; // New forked process return 0

		TASKLIST.push(new_task);
		process.pid
	}
}
