use core::ptr;
use crate::vec::Vec;
use crate::utils::queue::Queue;
use crate::interrupts::Registers;
use crate::proc::wrapper_fn;
use crate::memory::VirtAddr;
use crate::memory::paging::{PAGE_WRITABLE, alloc_page};
use crate::proc::signal::{SignalHandler};

pub static mut TASKLIST: Queue<Task> = Queue::new();

#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
	Running,
	Uninterruptible,
	Interruptible
}

#[derive(Copy, Clone)]
pub struct Task {
	pub regs: Registers,
	pub state: TaskStatus,
	pub process: *mut Process
}

impl Task {
	pub const fn new() -> Self {
		Self {
			regs: Registers::new(),
			state: TaskStatus::Running,
			process: ptr::null_mut()
		}
	}

	pub unsafe extern "C" fn init(&mut self, flags: u32, page_dir: u32, process: *mut Process) {
		self.regs.eip = wrapper_fn as VirtAddr;
		self.regs.eflags = flags;
		self.regs.cr3 = page_dir;
		self.process = process;
		self.regs.esp = (*process).stack.offset + ((*process).stack.size - 4) as u32;
	}
}

extern "C" {
	pub fn switch_task(regs: *const Registers);
}

#[no_mangle]
pub static mut STACK_TASK_SWITCH: VirtAddr = 0;

use crate::proc::process::{Process, MASTER_PROCESS, NEXT_PID, Status};
use crate::{KSTACK, KHEAP};

pub fn init_tasking() {
	let mut task = Task::new();
	unsafe {
		core::arch::asm!("
		mov {cr3}, cr3
		pushf
		mov {eflags}, [esp]
		popf",
		cr3 = out(reg) task.regs.cr3,
		eflags = out(reg) task.regs.eflags);
		let res = alloc_page(PAGE_WRITABLE);
		if !res.is_ok() {
			todo!();
		}
		STACK_TASK_SWITCH = res.unwrap() + 0x1000;
		MASTER_PROCESS.state = Status::Run;
		MASTER_PROCESS.childs = Vec::with_capacity(8);
		MASTER_PROCESS.signals = Vec::with_capacity(8);
		MASTER_PROCESS.stack = KSTACK;
		MASTER_PROCESS.heap = KHEAP;
		MASTER_PROCESS.owner = 0;
		NEXT_PID += 1;
		task.process = &mut MASTER_PROCESS;
		TASKLIST.push(task);
	}
}

pub unsafe fn remove_task_from_process(process: &mut Process) {
	let process_ptr: *mut Process= &mut *process;
	let len = TASKLIST.len();
	let mut i = 0;
	while i < len {
		let res = TASKLIST.peek();
		if res.is_none() {
			todo!();
		}
		let task: Task = res.unwrap();
		if task.process != process_ptr {
			TASKLIST.push(TASKLIST.pop());
		} else {
			TASKLIST.pop();
		}
		i += 1;
	}
}

use crate::wrappers::{_cli, _rst};
use crate::proc::signal::SignalType;

#[naked]
#[no_mangle]
pub unsafe extern "C" fn wrapper_handler() {
	core::arch::asm!("
	mov eax, [esp]
	add esp, 4
	call eax
	sub esp, 4
	cli
	jmp _end_handler",
	options(noreturn));
	/* Never goes there */
}

#[no_mangle]
unsafe extern "C" fn _end_handler() {
	crate::kprintln!("end_handler");
	let res = TASKLIST.peek();
	if res.is_none() {
		todo!();
	}
	let mut task: Task = res.unwrap();
	_cli();
	task.regs.esp += core::mem::size_of::<Task>() as u32;
	let regs: &mut Registers = &mut *(task.regs.esp as *mut _);
	task.regs = *regs;
	_rst();
	switch_task(&mut task.regs);
}

unsafe fn handle_signal(task: &mut Task, handler: &mut SignalHandler) {
	crate::kprintln!("handle_signal()");
	task.regs.esp -= core::mem::size_of::<Task>() as u32;
	(task.regs.esp as *mut Registers).write(task.regs);
	task.regs.esp -= 4;
	core::arch::asm!("mov [{esp}], {func}",
		esp = in(reg) task.regs.esp,
		func = in(reg) handler.signal);
	task.regs.esp -= 4;
	core::arch::asm!("mov [{esp}], {func}",
		esp = in(reg) task.regs.esp,
		func = in(reg) handler.handler);
	task.regs.eip = wrapper_handler as u32;
	_rst();
	switch_task(&mut task.regs);
}

unsafe fn do_signal(task: &mut Task) {
	let process = &mut *task.process;
	let len = process.signals.len();
	crate::kprintln!("len: {}", len);
	for i in 0..len {
		crate::kprintln!("bop");
		if task.state != TaskStatus::Uninterruptible &&
process.signals[i].sigtype == SignalType::SIGKILL {
			todo!(); /* sys_kill remove task etc.. ? */
		} else if task.state == TaskStatus::Running {

			crate::kprintln!("sigtype: {}", process.signals[i].sigtype as u32);
			for handler in process.signal_handlers.iter_mut() {
				crate::kprintln!("handler.signal: {}", handler.signal);
				if handler.signal == process.signals[i].sigtype as u32 {
					crate::kprintln!("removed");
					process.signals.remove(i);
					task.state = TaskStatus::Interruptible;
					handle_signal(task, handler);
				}
			}
		} else if task.state == TaskStatus::Interruptible &&
process.signals[i].sigtype == SignalType::SIGCHLD {
			task.state = TaskStatus::Running;
		}
	}
}

#[no_mangle]
pub unsafe extern "C" fn next_task(regs: &mut Registers) -> ! {
	_cli();
	let mut old_task: Task = TASKLIST.pop();
	old_task.regs = *regs;
	TASKLIST.push(old_task);
	loop {
		let res = TASKLIST.peek();
		if res.is_none() {
			todo!();
		}
		let new_task: &mut Task = &mut res.unwrap();
		/* TODO: IF SIGNAL JUMP ? */
		if (*new_task.process).signals.len() > 0 {
			crate::kprintln!("do_signal()");
			do_signal(new_task);
		}
		if new_task.state == TaskStatus::Running {
			_rst();
			switch_task(&new_task.regs);
		}
		let skipped_task: Task = TASKLIST.pop();
		TASKLIST.push(skipped_task);
	}
}
