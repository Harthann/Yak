use crate::memory::{MemoryZone, Stack, Heap};
use crate::KALLOCATOR;
use crate::vec::Vec;
use crate::{VirtAddr, PAGE_WRITABLE, alloc_page};
use crate::memory::paging::free_pages;

type Id = u32;

static mut NEXT_PID: Id = 0;
static mut MASTER_PROCESS: Process = Process::new();

enum Status {
	Disable,
	Run,
	Zombie,
	Thread
}

struct Signal {
}

pub struct Process {
	pid: Id,
	status: Status,
	parent: *mut Process,
	childs: Vec<Box<Process>>,
	stack: MemoryZone,
	heap: MemoryZone,
	signals: Vec<Box<Signal>>, /* TODO: VecDeque ? */
	owner: Id
}

impl Process {
	pub const fn new() -> Self {
		Self {
			pid: 0,
			status: Status::Disable,
			parent: core::ptr::null_mut(),
			childs: Vec::new(),
			stack: MemoryZone::new(),
			heap: MemoryZone::new(),
			signals: Vec::new(),
			owner: 0
		}
	}

	/* TODO: next_pid need to check overflow and if other pid is available */
	pub unsafe fn init(&mut self, parent: *mut Process, owner: Id) {
		self.pid = NEXT_PID;
		self.status = Status::Run;
		self.parent = parent;
		self.stack = <MemoryZone as Stack>::init(0x1000, PAGE_WRITABLE, false);
		self.heap = KHEAP;
//		self.heap = <MemoryZone as Heap>::init(0x1000, PAGE_WRITABLE, false, &mut KALLOCATOR);
		self.owner = owner;
		NEXT_PID += 1;
	}
}

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
	pub unsafe fn init(&mut self, addr: VirtAddr, func: VirtAddr, size: usize, flags: u32, page_dir: u32, process: *mut Process) {
		self.regs.eip = wrapper_fn as VirtAddr;
		self.regs.eflags = flags;
		self.regs.cr3 = page_dir;
		self.process = process;
		self.regs.esp = (*process).stack.offset + ((*process).stack.size - 8) as u32;
		unsafe {
			core::arch::asm!("mov [{esp} + 4], {func}",
				esp = in(reg) self.regs.esp,
				func = in(reg) func);
		}
	}
}

use crate::memory::allocator::Box;
use crate::{KSTACK, KHEAP};

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
	MASTER_PROCESS.status = Status::Run;
	MASTER_PROCESS.stack = KSTACK;
	MASTER_PROCESS.heap = KHEAP;
	MASTER_PROCESS.owner = 0;
	NEXT_PID += 1;
	main_task.process = &mut MASTER_PROCESS;
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
	crate::sti!();
}

pub unsafe extern "C" fn print_tasks() {
	crate::kprintln!("PRINT_TASKS ==>");
	let mut i = 0;
	let mut prev_task: &mut Task = &mut *RUNNING_TASK;
	while prev_task.next_ptr != &mut *RUNNING_TASK {
		crate::kprintln!("task ptr {}: {:p}", i, &mut *prev_task);
		crate::kprintln!("task.regs: {:#x?}", prev_task.regs);
		crate::kprintln!("task.next_ptr: {:#x?}", prev_task.next_ptr);
		if prev_task.next_ptr.is_null() {
			return ;
		}
		prev_task = &mut *prev_task.next_ptr;
		i += 1;
	}
	crate::kprintln!("task ptr {}: {:p}", i, &mut *prev_task);
	crate::kprintln!("task.regs: {:#x?}", prev_task.regs);
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

pub unsafe fn remove_process() {
	let process: &mut Process = &mut *(*RUNNING_TASK).process;
	while process.childs.len() > 0 {
		let res = process.childs.pop();
		if res.is_none() {
			todo!();
		}
		let u = res.unwrap();
		(*process.parent).childs.push(u);
		let len = (*process.parent).childs.len();
		(*process.parent).childs[len - 1].parent = process.parent;
	}
	let mut i = 0;
	while i < (*process.parent).childs.len() {
		let ptr: *mut Process = (*process.parent).childs[i].as_mut();
		if ptr == process {
			break ;
		}
		i += 1;
	}
	if i == (*process.parent).childs.len() {
		todo!(); // Problem
	}
	(*process.parent).childs.remove(i);
	free_pages(process.stack.offset, process.stack.size / 0x1000);
}

/* TODO: handle args */
#[no_mangle]
pub unsafe extern "C" fn wrapper_fn(func: VirtAddr) -> ! {
	core::arch::asm!("call {}", in(reg) func);
	crate::cli!();
	core::arch::asm!("mov esp, {}", in(reg) (STACK_TASK_SWITCH - 256));
	remove_process();
	remove_task();
	/* Never goes there */
}

pub fn		exec_fn(addr: VirtAddr, func: VirtAddr, size: usize) {
	crate::cli!();
	unsafe {
		let proc: Process =  Process::new();
		let parent: &mut Process = &mut *(*RUNNING_TASK).process;
		let mut childs: &mut Vec<Box<Process>> = &mut parent.childs;
		childs.push(Box::new(proc));
		let len = childs.len();
		let proc_ptr: *mut Process = childs[len - 1].as_mut();
		(*proc_ptr).init(&mut *parent, 0);
		let mut other_task: Task = Task::new();
		other_task.init(addr, func, size, (*RUNNING_TASK).regs.eflags, (*RUNNING_TASK).regs.cr3, proc_ptr);
		append_task(other_task);
	}
}
