use core::ptr;

use crate::interrupts::Registers;
use crate::memory::{init_heap, init_stack, VirtAddr};
use crate::proc::process::{Process, Status, MASTER_PROCESS, NEXT_PID};
use crate::proc::signal::{SignalHandler, SignalType};
use crate::proc::{change_kernel_stack, wrapper_fn};
use crate::utils::queue::Queue;
use crate::vec::Vec;
use crate::wrappers::{_cli, _rst};

use crate::memory::paging::PAGE_WRITABLE;
use crate::{KALLOCATOR, KSTACK_ADDR};

pub static mut TASKLIST: Queue<Task> = Queue::new();

extern "C" {
	pub fn switch_task(regs: *const Registers) -> !;
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum TaskStatus {
	Running,         // Normal state
	Uninterruptible, // In signal handler, could not be signaled again
	Interruptible    // Waiting for changing state (wait4 - waitpid)
}

#[derive(Copy, Clone)]
pub struct Task {
	pub regs:    Registers,
	pub state:   TaskStatus,
	pub process: *mut Process
}

impl Task {
	pub const fn new() -> Self {
		Self {
			regs:    Registers::new(),
			state:   TaskStatus::Running,
			process: ptr::null_mut()
		}
	}

	pub unsafe fn init(&mut self, regs: Registers, process: &mut Process) {
		self.regs.eip = wrapper_fn as VirtAddr;
		self.regs.eflags = regs.eflags;
		self.regs.cr3 = regs.cr3;
		self.process = process;
		self.regs.esp = process.stack.offset + (process.stack.size - 4) as u32;
	}

	pub fn init_multitasking(
		kstack_addr: VirtAddr,
		stack_addr: VirtAddr,
		heap_addr: VirtAddr
	) {
		let mut task = Task::new();
		unsafe {
			core::arch::asm!("
			mov {cr3}, cr3
			pushf
			mov {eflags}, [esp]
			popf",
			cr3 = out(reg) task.regs.cr3,
			eflags = out(reg) task.regs.eflags);
			MASTER_PROCESS.state = Status::Run;
			crate::kprintln!("kstack_addr: {:#x?}", kstack_addr);
			MASTER_PROCESS.kernel_stack =
				init_stack(kstack_addr, 0x1000, PAGE_WRITABLE, false);
			MASTER_PROCESS.stack =
				init_stack(stack_addr, 0x1000, PAGE_WRITABLE, false);
			MASTER_PROCESS.heap = init_heap(
				heap_addr,
				100 * 0x1000,
				PAGE_WRITABLE,
				true,
				&mut KALLOCATOR
			);
			MASTER_PROCESS.childs = Vec::with_capacity(8);
			MASTER_PROCESS.signals = Vec::with_capacity(8);
			MASTER_PROCESS.owner = 0;
			NEXT_PID += 1;
			task.process = &mut MASTER_PROCESS;
			TASKLIST.push(task);
		}
	}

	pub unsafe fn remove_task_from_process(process: &mut Process) {
		let process_ptr: *mut Process = &mut *process;
		let len = TASKLIST.len();
		let mut i = 0;
		while i < len {
			let task: &mut Task = Task::get_running_task();
			if task.process != process_ptr {
				TASKLIST.push(TASKLIST.pop());
			} else {
				TASKLIST.pop();
			}
			i += 1;
		}
	}

	pub unsafe fn get_running_task() -> &'static mut Task {
		let res = TASKLIST.front_mut();
		if res.is_none() {
			todo!();
		}
		&mut *res.unwrap()
	}
}

#[naked]
#[no_mangle]
unsafe extern "C" fn wrapper_handler() {
	core::arch::asm!("
	mov eax, [esp]
	add esp, 4
	call eax
	mov esp, {}
	cli
	jmp _end_handler",
	const KSTACK_ADDR,
	options(noreturn));
	// Never goes there
}

#[no_mangle]
unsafe fn _end_handler() {
	_cli();
	let task: &mut Task = Task::get_running_task();
	task.regs.esp += 8;
	let regs: &mut Registers = &mut *(task.regs.esp as *mut _);
	task.regs = *regs;
	task.regs.esp += core::mem::size_of::<Task>() as u32;
	task.state = TaskStatus::Running;
	change_kernel_stack(Process::get_running_process().kernel_stack.offset);
	_rst();
	crate::kprintln!("end_handler");
	switch_task(&task.regs);
}

unsafe fn handle_signal(task: &mut Task, handler: &mut SignalHandler) {
	task.regs.esp -= core::mem::size_of::<Task>() as u32;
	(task.regs.esp as *mut Registers).write(task.regs);
	task.regs.int_no = 0; // Reset int_no to return to new func (TODO: DO THIS BETTER)
					  // Setup args (int signal) and handler call
	task.regs.esp -= 4;
	core::arch::asm!("mov [{esp}], eax",
		esp = in(reg) task.regs.esp,
		in("eax") handler.signal);
	task.regs.esp -= 4;
	core::arch::asm!("mov [{esp}], eax",
		esp = in(reg) task.regs.esp,
		in("eax") handler.handler);
	task.regs.eip = wrapper_handler as u32;
	change_kernel_stack(Process::get_running_process().kernel_stack.offset);
	_rst();
	crate::kprintln!("handle_signal");
	switch_task(&task.regs);
}

unsafe fn do_signal(task: &mut Task) {
	let process = &mut *task.process;
	let len = process.signals.len();
	for i in 0..len {
		if task.state != TaskStatus::Uninterruptible
			&& process.signals[i].sigtype == SignalType::SIGKILL
		{
			todo!(); // sys_kill remove task etc.. ?
		} else if task.state == TaskStatus::Running {
			for handler in process.signal_handlers.iter_mut() {
				if handler.signal == process.signals[i].sigtype as i32 {
					process.signals.remove(i);
					task.state = TaskStatus::Uninterruptible;
					handle_signal(task, handler);
				}
			}
		} else if task.state == TaskStatus::Interruptible
			&& process.signals[i].sigtype == SignalType::SIGCHLD
		{
			task.state = TaskStatus::Running;
		}
	}
}

#[no_mangle]
pub unsafe extern "C" fn save_task(regs: &Registers) {
	_cli();
	let mut old_task: Task = TASKLIST.pop();
	old_task.regs = *regs;
	crate::kprintln!("old_regs: {:#x?}", old_task.regs);
	TASKLIST.push(old_task);
	_rst();
}

#[no_mangle]
pub unsafe extern "C" fn schedule_task() -> ! {
	_cli();
	loop {
		let new_task: &mut Task = Task::get_running_task();
		// TODO: IF SIGNAL JUMP ?
		if (*new_task.process).signals.len() > 0 {
			do_signal(new_task);
		}
		if new_task.state != TaskStatus::Interruptible {
			crate::kprintln!("new_task pid: {}", (*new_task.process).pid);
			let process: &mut Process = Process::get_running_process();
			_rst();
			core::ptr::copy(
				(KSTACK_ADDR - 0xfff) as *const u8,
				process.kernel_stack.offset as *mut u8,
				0x1000
			);
			change_kernel_stack(process.kernel_stack.offset);
			refresh_tlb!();
			// 			TODO NOT WORKING
			// 			/* Reload tss to get the new kernel stack */
			// 			reload_tss!();
			crate::kprintln!("new_regs: {:#x?}", new_task.regs);
			// Put registers into the stack
			let regs: Registers = new_task.regs;
			switch_task(&regs);
			// never goes there
		}
		let skipped_task: Task = TASKLIST.pop();
		TASKLIST.push(skipped_task);
	}
}
