//! Testing user space code

use core::ptr::copy_nonoverlapping;

use crate::wrappers::{_cli, _sti};

use crate::memory::{MemoryZone, Heap};

use crate::memory::paging as mem;
use crate::memory::paging::{PAGE_WRITABLE, PAGE_USER, PAGE_PRESENT};
use crate::VirtAddr;

use crate::proc::process::{Process, Pid};
use crate::proc::task::{Task, TASKLIST};

use crate::memory::paging::page_directory;
use crate::memory::paging::page_directory::PageDirectory;
use crate::memory::paging::page_table::PageTable;

use crate::memory::allocator::Box;

extern "C" {
	fn jump_usermode(func: VirtAddr);
	fn userfunc();
	fn userfunc_end();
}

pub unsafe fn exec_fn_userspace(func: VirtAddr, size: usize) -> Pid {
	_cli();
	let running_task: &mut Task = Task::get_running_task();
	let parent: &mut Process = Process::get_running_process();

	let mut process: Process = Process::new();
	process.init(parent);
	process.heap = <MemoryZone as Heap>::init_no_allocator(
		size,
		PAGE_WRITABLE | PAGE_USER,
		false
	);
	copy_nonoverlapping(
		func as *mut u8,
		process.heap.offset as *mut u8,
		size
	);
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
		get_paddr!(handler_page_tab as *const _) | PAGE_WRITABLE | PAGE_PRESENT
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
		get_paddr!(handler_page_tab as *const _) | PAGE_WRITABLE | PAGE_PRESENT
	);
	new_task.regs.esp -= 4;
	core::arch::asm!("mov [{esp}], {func}",
		esp = in(reg) new_task.regs.esp,
		func = in(reg) func);
	new_task.regs.esp -= 4;
	core::arch::asm!("mov [{esp}], {func}",
		esp = in(reg) new_task.regs.esp,
		func = in(reg) jump_usermode);
	TASKLIST.push(new_task);
	_sti();
	process.pid
}

pub fn test_user_page() {
	let userpage = mem::alloc_pages_at_addr(0x400000, 1, PAGE_WRITABLE | PAGE_USER).expect("");
	let funclen = userfunc_end as usize - userfunc as usize;

	unsafe {
		core::ptr::copy_nonoverlapping(userfunc as *const u8, userpage as *mut u8, funclen);
	}
	mem::print_pdentry(1);
	unsafe {
		jump_usermode(0x400000);
	}
}
