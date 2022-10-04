use crate::memory::{MemoryZone, Stack};
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

	pub fn search_from_pid(&self, pid: Id) -> Result<&Process, ()> {
		if self.pid == pid {
			return Ok(self);
		}
		for process in self.childs.iter() {
			let res = process.search_from_pid(pid);
			if res.is_ok() {
				return res;
			}
		}
		Err(())
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

	pub unsafe fn remove(&mut self) {
		let parent: &mut Process = &mut *self.parent;
		while self.childs.len() > 0 {
			let res = self.childs.pop();
			if res.is_none() {
				todo!();
			}
			parent.childs.push(res.unwrap());
			let len = parent.childs.len();
			parent.childs[len - 1].parent = self.parent;
		}
		let mut i = 0;
		while i < parent.childs.len() {
			let ptr: *mut Process = parent.childs[i].as_mut();
			if ptr == &mut *self {
				break ;
			}
			i += 1;
		}
		if i == parent.childs.len() {
			todo!(); // Problem
		}
		parent.childs.remove(i);
		free_pages(self.stack.offset, self.stack.size / 0x1000);
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

pub unsafe fn remove_running_process() {
	let process: &mut Process = &mut *(*RUNNING_TASK).process;
	process.remove();
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

#[no_mangle]
pub unsafe extern "C" fn exit_fn() -> ! {
	core::arch::asm!("mov esp, {}", in(reg) (STACK_TASK_SWITCH - 256));
	remove_running_process();
	remove_running_task();
	/* Never goes there */
}

#[naked]
#[no_mangle]
pub unsafe extern "C" fn wrapper_fn() {
	core::arch::asm!("
	mov eax, [esp]
	add esp, 4
	call eax
	cli
	jmp exit_fn",
	options(noreturn));
	/* Never goes there */
}

pub unsafe extern "C" fn		exec_fn(func: VirtAddr, args_size: &Vec<usize>, mut args: ...) {
	crate::cli!();
	let proc: Process =  Process::new();
	let parent: &mut Process = &mut *(*RUNNING_TASK).process;
	let childs: &mut Vec<Box<Process>> = &mut parent.childs;
	childs.push(Box::new(proc));
	let len = childs.len();
	let proc_ptr: *mut Process = childs[len - 1].as_mut();
	(*proc_ptr).init(&mut *parent, 0);
	let mut other_task: Task = Task::new();
	other_task.init((*RUNNING_TASK).regs.eflags, (*RUNNING_TASK).regs.cr3, proc_ptr);
	/* init_fn_task - Can't move to another function ??*/
	let sum: usize = args_size.iter().sum();
	other_task.regs.esp -= sum as u32;
	let mut nb = 0;
	for arg in args_size.iter() {
		let mut n: usize = *arg;
		while n > 0 {
			if arg / 4 > 0 {
				core::arch::asm!("mov [{esp} + {nb}], eax",
					esp = in(reg) other_task.regs.esp,
					nb = in(reg) nb,
					in("eax") args.arg::<u32>());
				n -= 4;
				nb += 4;
			}
			else if arg / 2 > 0 {
				core::arch::asm!("mov [{esp} + {nb}], ax",
					esp = in(reg) other_task.regs.esp,
					nb = in(reg) nb,
					in("ax") args.arg::<u16>());
				n -= 2;
				nb += 2;
			} else {
				todo!();
			}
		}
	}
	/* call function to wrapper_fn */
	other_task.regs.esp -= 4;
	core::arch::asm!("mov [{esp}], {func}",
		esp = in(reg) other_task.regs.esp,
		func = in(reg) func);
	append_task(other_task);
}

#[macro_export]
macro_rules! size_of_args {
	($vector:expr, $name:expr) => { $vector.push(core::mem::size_of_val(&$name)); };
	($vector: expr, $x:expr, $($rest:expr),+) => {
		crate::size_of_args!($vector, $x); crate::size_of_args!($vector, $($rest),+)
	}
}

#[macro_export]
macro_rules! exec_fn {
	($func:expr, $($rest:expr),+) => {
		let mut args_size: crate::vec::Vec<usize> = Vec::new();
		crate::size_of_args!(args_size, $($rest),+);
		crate::proc::exec_fn($func, &args_size, $($rest),+);
	}
}

