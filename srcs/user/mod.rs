//! Testing user space code

use core::ptr::copy_nonoverlapping;

use crate::wrappers::{_cli, _sti};

use crate::memory::paging::{PAGE_USER, PAGE_WRITABLE};
use crate::memory::VirtAddr;

use crate::proc::process::{Pid, Process};
use crate::proc::task::{Task, TASKLIST};

use crate::memory::paging::page_directory::PageDirectory;

use crate::boxed::Box;

#[cfg(test)]
pub mod test;

extern "C" {
	fn userfunc();
	fn userfunc_end();
}

pub const USER_HEAP_ADDR: VirtAddr = 0x0800000;
pub const USER_STACK_ADDR: VirtAddr = 0xbfffffff;

#[naked]
unsafe extern "C" fn jump_usermode(func: VirtAddr) -> ! {
	core::arch::asm!(
		"mov ebx, DWORD PTR[esp + 4]",
		"mov ax, (5 * 8) | 3", // ring 3 data with bottom 2 bits set for ring 3
		"mov ds, ax",
		"mov es, ax",
		"mov fs, ax",
		"mov gs, ax", // ss is handled by iret
		// set up the stack frame iret expects
		"mov eax, esp",
		"push (5 * 8) | 3", // data selector
		"push eax",         // current esp
		"pushfd",           // eflags
		"push (4 * 8) | 3", /* code selector (ring 3 code with bottom 2 bits set for ring 3) */
		"push ebx",         // func
		"iretd",
		options(noreturn)
	);
}

pub unsafe fn exec_fn_userspace(func: VirtAddr, size: usize) -> Pid {
	_cli();
	let running_task: &mut Task = Task::get_running_task();
	let parent: &mut Process = Process::get_running_process();

	let mut process: Process = Process::new();
	process.init(parent);
	process.setup_kernel_stack(PAGE_WRITABLE | PAGE_USER);
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

	// TODO: free those when process ends ?
	let page_dir: &mut PageDirectory = process.setup_pagination();

	copy_nonoverlapping(func as *mut u8, process.heap.offset as *mut u8, size);
	new_task.regs.esp = process.stack.offset + process.stack.size as u32;
	new_task.regs.cr3 = get_paddr!(page_dir as *const _);
	new_task.regs.esp -= 4;
    process.test = true;

	core::arch::asm!("mov [{esp}], {func}",
		esp = in(reg) new_task.regs.esp,
		func = in(reg) USER_HEAP_ADDR);
	new_task.regs.esp = USER_STACK_ADDR - 7;
	new_task.regs.eip = jump_usermode as VirtAddr;
	new_task.regs.ds = running_task.regs.ds;

	TASKLIST.push_back(new_task);
	_sti();
	process.pid
}

core::arch::global_asm!(
	r#"
.globl userfunc
.globl userfunc_end
userfunc:
	mov eax, 2
	int 0x80
	cmp eax, 0
	jne .wait_child

	mov ebx, 42
	mov eax, 1
	int 0x80

	.wait_child:
	mov edx, 0
	mov ecx, 0
	mov ebx, eax
	mov eax, 7
	int 0x80
	mov ebx, eax
	mov eax, 1
	int 0x80
userfunc_end:
"#
);

pub fn test_user_page() {
	unsafe {
		exec_fn_userspace(
			userfunc as u32,
			userfunc_end as usize - userfunc as usize
		);
	}
	let ret = crate::syscalls::exit::sys_waitpid(-1, core::ptr::null_mut(), 0);
	crate::kprintln!("pid ret: {}", ret);
}
