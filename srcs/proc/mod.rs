use crate::memory::MemoryZone;
use crate::memory::paging::{alloc_page, PAGE_WRITABLE};
use crate::vec::Vec;
use crate::VirtAddr;

enum Status {
	Run,
	Zombie,
	Thead
}

type Id = u32;

struct Signal {
}

struct Process {
	pid: Id,
	status: Status,
	parent: *const Process,
	childs: Vec<*mut Process>,
	stack: MemoryZone,
	heap: MemoryZone,
	signals: Vec<Signal>, /* TODO: VecDeque ? */
	owner: Id
}

static mut RUNNING_TASK: *mut Task = core::ptr::null_mut();
pub static mut MAIN_TASK: Task = Task::new();

#[repr(C, packed)]
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

pub struct Task {
	pub regs: Registers,
	pub prev: Option<&'static mut Task>,
	pub next: Option<&'static mut Task>
}

impl Task {
	pub const fn new() -> Self {
		Self {
			regs: Registers {
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
			},
			prev: None,
			next: None
		}
	}

	pub fn init(&mut self, addr: VirtAddr, func: u32, size: u32, flags: u32, page_dir: u32) {
		self.regs.eip = func;
		self.regs.eflags = flags;
		self.regs.cr3 = page_dir;
		self.regs.esp = addr + size;
	}
}

use crate::memory::allocator::Box;

pub unsafe fn init_tasking() {
	core::arch::asm!("mov {}, cr3", out(reg) MAIN_TASK.regs.cr3);
	core::arch::asm!("pushf",
					"mov {}, [esp]",
					"popf", out(reg) MAIN_TASK.regs.eflags);
	RUNNING_TASK = &mut MAIN_TASK;
}

pub unsafe fn append_task(new_task: &'static mut Task) {
	/* TODO: mutex lock prevent switch_task .. */
	let task: &mut Task = &mut *RUNNING_TASK;
	new_task.prev = Some(&mut *RUNNING_TASK);
	new_task.next = Some((*RUNNING_TASK).next.take().unwrap());
	let next = new_task.next.take().unwrap();
	next.prev = Some(new_task);
	task.next = Some(next.prev.take().unwrap());
}

pub unsafe fn remove_task() {/* exit ? */
	/* TODO: mutex lock prevent switch_task .. */
	let task: &mut Task = &mut *RUNNING_TASK;
	RUNNING_TASK = &mut *task.next.take().unwrap();
	let prev = task.prev.take().unwrap();
	prev.next = Some(task.next.take().unwrap());
	(*RUNNING_TASK).prev = Some(task.prev.take().unwrap());
	loop {} /* waiting for switch - TODO: replace by int ? */
}

extern "C" {
	fn switch_task(reg_from: *const Registers, reg_to: *const Registers);
}

#[no_mangle]
pub unsafe extern "C" fn next_task() {
	let last: *const Task = RUNNING_TASK;
	RUNNING_TASK = &mut *(*RUNNING_TASK).next.take().unwrap();
//	crate::kprintln!("Running task: {:#x?}", *RUNNING_TASK);
	core::arch::asm!("cli");
	crate::kprintln!("switching...");
	switch_task(&(*last).regs, &(*RUNNING_TASK).regs);
	core::arch::asm!("sti");
}

pub fn		exec_fn(addr: VirtAddr, func: VirtAddr, size: u32) {
	unsafe {
		let mut other_task: Task = Task::new();
		other_task.init(addr, func, size, MAIN_TASK.regs.eflags, MAIN_TASK.regs.cr3);
		let boxed = Box::new(other_task);
		let stask: &'static mut Task = Box::leak(boxed);
		append_task(stask);
	}
}
