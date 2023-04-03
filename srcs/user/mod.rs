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

const USER_HEAP_ADDR: VirtAddr = 0x0800000;
const USER_STACK_ADDR: VirtAddr = 0xbfffffff;

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

	let kernel_pt_paddr: PhysAddr =
		get_paddr!(page_directory.get_page_table(768).get_vaddr());
	crate::kprintln!("kernel_pt_paddr: {:#x}", kernel_pt_paddr);

	// TODO: free those when process ends ?
	let page_dir: &mut PageDirectory = PageDirectory::new();
	let process_heap: &mut PageTable = PageTable::new();
	let process_stack: &mut PageTable = PageTable::new();
	let process_kernel_stack: &mut PageTable = PageTable::new();
	crate::kprintln!("kernel_pt_paddr: {:#x?}", kernel_pt_paddr);

	// Setup heap + prg
	page_dir.set_entry(
		USER_HEAP_ADDR as usize >> 22,
		get_paddr!(process_heap as *const _)
			| PAGE_WRITABLE
			| PAGE_PRESENT
			| PAGE_USER
	);
	// Setup stack
	page_dir.set_entry(
		USER_STACK_ADDR as usize >> 22,
		get_paddr!(process_stack as *const _)
			| PAGE_WRITABLE
			| PAGE_PRESENT
			| PAGE_USER
	);
	page_dir.set_entry(
		768,
		kernel_pt_paddr | PAGE_WRITABLE | PAGE_PRESENT | PAGE_USER
	);
	page_dir.set_entry(
		KSTACK_ADDR as usize >> 22,
		get_paddr!(process_kernel_stack as *const _)
			| PAGE_WRITABLE
			| PAGE_PRESENT
			| PAGE_USER
	);
	page_dir.set_entry(
		1023,
		get_paddr!(page_dir as *const _)
			| PAGE_WRITABLE
			| PAGE_PRESENT
			| PAGE_USER
	);

	// Setup stack and heap
	process_heap.new_index_frame(
		(USER_HEAP_ADDR as usize & 0x3ff000) >> 12,
		get_paddr!(process.heap.offset),
		PAGE_WRITABLE | PAGE_USER
	);
	process_stack.new_index_frame(
		(USER_STACK_ADDR as usize & 0x3ff000) >> 12,
		get_paddr!(process.stack.offset),
		PAGE_WRITABLE | PAGE_USER
	);
	process_kernel_stack.new_index_frame(
		(KSTACK_ADDR as usize & 0x3ff000) >> 12,
		get_paddr!(process.kernel_stack.offset),
		PAGE_WRITABLE | PAGE_USER
	);

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
