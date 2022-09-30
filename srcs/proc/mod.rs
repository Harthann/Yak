use crate::memory::{MemoryZone, Stack, Heap};
use crate::KALLOCATOR;
use crate::vec::Vec;
use crate::{VirtAddr, PAGE_WRITABLE, alloc_page};
use crate::memory::paging::free_pages;

type Id = u32;

static mut next_pid: Id = 0;

enum Status {
	Run,
	Zombie,
	Thread
}

struct Signal {
}

struct Process {
	pid: Id,
	status: Status,
	parent: *const Process,
	childs: Vec<Process>,
	stack: MemoryZone,
	heap: MemoryZone,
	signals: Vec<Signal>, /* TODO: VecDeque ? */
	owner: Id
}

/*
impl Process {
	pub const fn new() -> Self {
		Self {
			pid: 0,
			status: 0,
			parent: core::ptr::null(),
			childs: Vec::new(),
			stack: ,
			heap: ,
			signals: Vec::new(),
			owner: 0
		}
	}
	/* TODO: next_pid need to check overflow and if other pid is available */
}
*/

static mut RUNNING_TASK: *mut Task = core::ptr::null_mut();
static mut STACK_TASK_SWITCH: VirtAddr = 0;

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct Registers {
	pub eax:	u32,
	pub ebx:	u32,
	pub ecx:	u32,
	pub edx:	u32,
	pub esi:	u32,
	pub edi:	u32,
	pub esp:	u32,
	pub ebp:	u32,
	pub eip:	u32,
	pub eflags:	u32,
	pub cr3:	u32
}

impl Registers {
	const fn new() -> Self {
		Self {
			eax: 0,
			ebx: 0,
			ecx: 0,
			edx: 0,
			esi: 0,
			edi: 0,
			esp: 0,
			ebp: 0,
			eip: 0,
			eflags: 0,
			cr3: 0
		}
	}
}

pub struct Task {
	pub regs: Registers,
	pub stack_ptr: u32, /* TODO: replace by MemoryZone ? */
	pub stack_size: usize,
	pub next_ptr: *mut Task,
	pub next: Option<Box<Task>>
}

impl Task {
	pub const fn new() -> Self {
		Self {
			regs: Registers::new(),
			stack_ptr: 0,
			stack_size: 0,
			next_ptr: core::ptr::null_mut(),
			next: None
		}
	}

	/* TODO: handle args and setup stack there */
	pub fn init(&mut self, addr: VirtAddr, func: VirtAddr, size: usize, flags: u32, page_dir: u32) {
		self.regs.eip = wrapper_fn as VirtAddr;
		self.regs.eflags = flags;
		self.regs.cr3 = page_dir;
		self.stack_ptr = addr;
		self.stack_size = size;
		self.regs.esp = addr + (size - 8) as u32;
		unsafe {
			core::arch::asm!("mov [{esp} + 4], {func}",
				esp = in(reg) self.regs.esp,
				func = in(reg) func);
		}
	}
}

use crate::memory::allocator::Box;

pub unsafe fn init_tasking(main_task: &mut Task) {
	core::arch::asm!("mov {}, cr3", out(reg) main_task.regs.cr3);
	core::arch::asm!("pushf",
					"mov {}, [esp]",
					"popf", out(reg) main_task.regs.eflags);
	let res = alloc_page(PAGE_WRITABLE);
	if !res.is_ok() {
		todo!();
	}
	STACK_TASK_SWITCH = res.unwrap() + 0x1000;
	RUNNING_TASK = &mut *main_task;
}

pub unsafe fn append_task(mut new_task: Task) {
	crate::cli!();
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
	crate::sti!();
}

pub unsafe fn print_tasks() {
	crate::kprintln!("PRINT_TASKS ==>");
	let mut i = 0;
	let mut prev_task: &mut Task = &mut *RUNNING_TASK;
	while prev_task.next_ptr != &mut *RUNNING_TASK {
		crate::kprintln!("task.regs {}: {:#x?}", i, prev_task.regs);
		crate::kprintln!("task.next_ptr: {:#x?}", prev_task.next_ptr);
		if prev_task.next_ptr.is_null() {
			return ;
		}
		prev_task = &mut *prev_task.next_ptr;
		i += 1;
	}
	crate::kprintln!("task.regs {}: {:#x?}", i, prev_task.regs);
	crate::kprintln!("task.next_ptr: {:#x?}", prev_task.next_ptr);
}


pub unsafe fn remove_task() -> ! {
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
	free_pages((*RUNNING_TASK).stack_ptr, (*RUNNING_TASK).stack_size / 0x1000);
	if (*RUNNING_TASK).next.is_none() {
		(*prev_task).next = None;
	} else {
		(*prev_task).next = Some((*RUNNING_TASK).next.take().unwrap());
	}
	RUNNING_TASK = ptr;
	let regs: Registers = Registers::new();
	switch_task(&regs, &(*RUNNING_TASK).regs);
	/* Never goes there */
	loop {}
}

/* switch_case contains `sti` */
extern "C" {
	fn switch_task(reg_from: *const Registers, reg_to: *const Registers);
}

#[no_mangle]
pub unsafe extern "C" fn next_task() {
	if !(*RUNNING_TASK).next_ptr.is_null() {
		let last: *const Task = RUNNING_TASK;
		RUNNING_TASK = (*RUNNING_TASK).next_ptr;
		switch_task(&(*last).regs, &(*RUNNING_TASK).regs);
	}
}

/* TODO: handle args */
#[no_mangle]
pub unsafe extern "C" fn wrapper_fn(func: VirtAddr) -> ! {
	core::arch::asm!("call {}", in(reg) func);
	crate::cli!();
	core::arch::asm!("mov esp, {}", in(reg) (STACK_TASK_SWITCH - 256));
	remove_task();
	/* Never goes there */
}

pub fn		exec_fn(addr: VirtAddr, func: VirtAddr, size: usize) {
	unsafe {
		let mut other_task: Task = Task::new();
		other_task.init(addr, func, size, (*RUNNING_TASK).regs.eflags, (*RUNNING_TASK).regs.cr3);
		append_task(other_task);
	}
}
