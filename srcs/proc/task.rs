use crate::memory::VirtAddr;
use crate::memory::paging::{PAGE_WRITABLE, alloc_page};

use crate::proc::wrapper_fn;
use crate::proc::process::{Process, MASTER_PROCESS, NEXT_PID, Status};

pub static mut RUNNING_TASK: *mut Task = core::ptr::null_mut();
#[no_mangle]
pub static mut STACK_TASK_SWITCH: VirtAddr = 0;
use crate::interrupts::Registers;

extern "C" {
	fn switch_task(regs: *const Registers);
}

pub struct Task {
	pub regs: Registers,
	pub process: *mut Process,
	pub next_ptr: *mut Task,
	pub next: Option<Box<Task>>
}

impl Task {
	pub const fn new() -> Self {
		Self {
			regs: Registers::new(),
			process: core::ptr::null_mut(),
			next_ptr: core::ptr::null_mut(),
			next: None
		}
	}

	/* TODO: handle args and setup stack there */
	pub unsafe extern "C" fn init(&mut self, flags: u32, page_dir: u32, process: *mut Process) {
		self.regs.eip = wrapper_fn as VirtAddr;
		self.regs.eflags = flags;
		self.regs.cr3 = page_dir;
		self.process = process;
		self.regs.esp = (*process).stack.offset + ((*process).stack.size - 4) as u32;
	}
}

use crate::memory::allocator::Box;
use crate::{KSTACK, KHEAP};

pub unsafe fn init_tasking(master_process: &mut Process, main_task: &mut Task) {
	core::arch::asm!("mov {}, cr3", out(reg) main_task.regs.cr3);
	core::arch::asm!("pushf",
					"mov {}, [esp]",
					"popf", out(reg) main_task.regs.eflags);
	let res = alloc_page(PAGE_WRITABLE);
	if !res.is_ok() {
		todo!();
	}
	STACK_TASK_SWITCH = res.unwrap() + 0x1000;
	crate::kprintln!("STACK_TASK_SWITCH: {:#x?}", STACK_TASK_SWITCH);
	master_process.status = Status::Run;
	master_process.stack = KSTACK;
	master_process.heap = KHEAP;
	master_process.owner = 0;
	NEXT_PID += 1;
	main_task.process = &mut *master_process;
	RUNNING_TASK = &mut *main_task;
}

pub unsafe fn append_task(mut new_task: Task) {
	let mut task: &mut Task = &mut *RUNNING_TASK;
	if !task.next_ptr.is_null() {
		while !task.next.is_none() {
			task = &mut *task.next_ptr;
		}
		new_task.next_ptr = task.next_ptr;
	} else {
		new_task.next_ptr = &mut *task;
	}
	new_task.next = None;
	task.next = Some(Box::new(new_task));
	task.next_ptr = &mut *(task.next.as_mut().unwrap()).as_mut();
}

pub unsafe fn remove_running_task() -> ! {
	let mut prev_task: &mut Task = &mut *RUNNING_TASK;
	while prev_task.next_ptr != &mut *RUNNING_TASK {
		prev_task = &mut *prev_task.next_ptr;
	}
	let ptr: *mut Task = &mut *prev_task;
	if ptr == (*RUNNING_TASK).next_ptr {
		(*prev_task).next_ptr = core::ptr::null_mut();
	} else {
		(*prev_task).next_ptr = (*RUNNING_TASK).next_ptr;
	}
	if (*RUNNING_TASK).next.is_none() {
		(*prev_task).next = None;
	} else {
		(*prev_task).next = Some((*RUNNING_TASK).next.take().unwrap());
	}
	RUNNING_TASK = ptr;
	switch_task(&(*RUNNING_TASK).regs);
	/* Never goes there */
	loop {}
}

#[no_mangle]
pub unsafe extern "C" fn next_task(regs: &mut Registers) -> !{
	(*RUNNING_TASK).regs = *regs;
	if !(*RUNNING_TASK).next_ptr.is_null() {
		RUNNING_TASK = (*RUNNING_TASK).next_ptr;
	}
	switch_task(&(*RUNNING_TASK).regs);
	/* Never goes there */
	loop {}
}
