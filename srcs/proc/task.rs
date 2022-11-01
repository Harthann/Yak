use core::ptr;
use crate::vec::Vec;
use crate::utils::queue::Queue;
use crate::interrupts::Registers;
use crate::proc::wrapper_fn;
use crate::memory::VirtAddr;
use crate::memory::paging::{PAGE_WRITABLE, alloc_page};

pub static mut TASKLIST: Queue<Task> = Queue::new();

pub type SigHandler = extern "C" fn (i32);

pub struct SignalHandler {
	signal: u32,
	handler: SigHandler
}

enum Status {
	Running,
	Uninterruptible,
	interruptible
}

#[derive(Copy, Clone)]
pub struct Task {
	pub regs: Registers,
	pub status: Status,
	pub process: *mut Process
}

impl Task {
	pub const fn new() -> Self {
		Self {
			regs: Registers::new(),
			status: Status::Running,
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
		let task: &Task = TASKLIST.peek().unwrap();
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
	cli
	jmp _end_handler",
	options(noreturn));
	/* Never goes there */
}

fn _end_handler() {
	
}

fn handle_signal(task: &mut Task, handler: SigHandler) {
	task.regs.esp -= core::mem::size_of<Task>();
	(task.regs.esp as *mut Registers).write(task.regs);
	task.regs.esp -= 4;
	core::arch::asm!("mov [{esp}], {func}",
		esp = in(reg) task.regs.esp,
		func = in(reg) handler);
	task.regs.esp -= 4;
	core::arch::asm!("mov [{esp}], {func}",
		esp = in(reg) task.regs.esp,
		func = in(reg) handler);
	task.regs.eip = wrapper_handler;
	_rst();
	switch_task(&mut regs);
}

fn do_signal(task: &mut Task) {
	let process = (*task.process);
	let len = process.signals.len();
	for i in 0..len {
		if task.status != SignalType::Uninterruptible &&
process.signals[i].sigtype == SignalType::SIGKILL {
			todo!(); /* sys_kill remove task etc.. ? */
		} else if task.status == SignalType::Running {
			process.signals[i].sigtype as u32;

			let signal = process.signals.remove(i);
			for handler in process.signal_handlers {
				if handler.signal == signal.sigtype as u32 {
					task.status = SignalType::Interruptible;
					handle_signal(task, handler);
				}
			}
		}
	}
}

#[no_mangle]
pub unsafe extern "C" fn next_task(regs: &mut Registers) -> ! {
	_cli();
	let mut task = TASKLIST.pop();
	task.regs = *regs;
	TASKLIST.push(task);
	let res = &TASKLIST.peek();
	if res.is_none() {
		todo!();
	}
	let task = res.unwrap();
	let mut regs = task.regs;
	/* TODO: IF SIGNAL JUMP ? */
	if ((*task.process).signals.len() > 0) {
		do_signal(task);
	}
	_rst();
	switch_task(&mut regs);
	/* Never goes there */
	loop {}
