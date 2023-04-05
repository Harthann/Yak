//! Testing user space code

use core::ptr::copy_nonoverlapping;

use crate::wrappers::{_cli, _sti};

use crate::memory::paging::{PAGE_PRESENT, PAGE_USER, PAGE_WRITABLE};
use crate::memory::{PhysAddr, VirtAddr};

use crate::proc::process::{Pid, Process};
use crate::proc::task::{Task, TASKLIST};

use crate::memory::paging::page_directory;
use crate::memory::paging::page_directory::PageDirectory;
use crate::memory::paging::page_table::PageTable;

use crate::memory::allocator::Box;

use crate::KSTACK_ADDR;

extern "C" {
	fn jump_usermode(func: VirtAddr);
	fn userfunc();
	fn userfunc_end();
}

pub const USER_HEAP_ADDR: VirtAddr = 0x0800000;
pub const USER_STACK_ADDR: VirtAddr = 0xbfffffff;

pub unsafe fn exec_fn_userspace(func: VirtAddr, size: usize) -> Pid {
	_cli();
	// 	let running_task: &mut Task = Task::get_running_task();
	let parent: &mut Process = Process::get_running_process();

	let mut process: Process = Process::new();
	process.init(parent);
	process.setup_kernel_stack(0x1000, PAGE_WRITABLE | PAGE_USER, false);
	process.setup_stack(0x1000, PAGE_WRITABLE | PAGE_USER, false);
	process.setup_heap(
		(size - (size % 0x1000)) + 0x1000,
		PAGE_WRITABLE | PAGE_USER,
		false
	);
	parent.childs.push(Box::new(process));

	let process: &mut Process = parent.childs.last_mut().unwrap();
	let mut new_task: Task = Task::new();

	new_task.process = process;

	crate::kprintln!("where is my user stack: {:#x?}", process.stack.offset);

	// TODO: free those when process ends ?
	let page_dir: &mut PageDirectory = process.setup_pagination(true);

	copy_nonoverlapping(func as *mut u8, process.heap.offset as *mut u8, size);
	new_task.regs.esp = process.stack.offset + process.stack.size as u32;
	new_task.regs.cr3 = get_paddr!(page_dir as *const _);
	new_task.regs.esp -= 4;
	crate::kprintln!(
		"write at the addr: {:#x}",
		process.stack.offset + process.stack.size as u32 - 4
	);
	core::arch::asm!("mov [{esp}], {func}",
		esp = in(reg) new_task.regs.esp,
		func = in(reg) USER_HEAP_ADDR);
	new_task.regs.esp = USER_STACK_ADDR - 7;
	crate::kprintln!("USER_HEAD_ADDR: {:#x?}", USER_HEAP_ADDR);
	crate::kprintln!("USER_STACK_ADDR: {:#x?}", USER_STACK_ADDR);
	new_task.regs.eip = jump_usermode as VirtAddr;
	TASKLIST.push(new_task);
	_sti();
	process.pid
}

pub fn test_user_page() {
	unsafe {
		exec_fn_userspace(
			userfunc as u32,
			userfunc_end as usize - userfunc as usize
		);
	}
		let mut status: i32 = 0;
		let ret = crate::syscalls::exit::sys_waitpid(-1, &mut status, 0);
		crate::kprintln!("pid ret: {}", ret);
		crate::kprintln!(
			"status: {}",
			crate::syscalls::exit::__WEXITSTATUS!(status)
		);
	// let userpage = mem::alloc_pages_at_addr(0x400000, 1, PAGE_WRITABLE | PAGE_USER).expect("");
	// let funclen = userfunc_end as usize - userfunc as usize;
	//
	// unsafe {
	// core::ptr::copy_nonoverlapping(userfunc as *const u8, userpage as *mut u8, funclen);
	// }
	// mem::print_pdentry(1);
	// unsafe {
	// jump_usermode(0x400000);
	// }
}
