use core::ptr;

use crate::interrupts::Registers;
use crate::memory::paging::page_directory;
use crate::memory::{Heap, MemoryZone, Stack, VirtAddr};
use crate::proc::process::{Process, Status, MASTER_PROCESS, NEXT_PID};
use crate::proc::signal::{SignalHandler, SignalType};
use crate::proc::wrapper_fn;
use crate::utils::queue::Queue;
use crate::vec::Vec;
use crate::wrappers::{_cli, _rst};

use crate::memory::paging::{PAGE_GLOBAL, PAGE_WRITABLE};
use crate::{KALLOCATOR, KSTACK_ADDR};

pub static mut TASKLIST: Queue<Task> = Queue::new();

#[no_mangle]
pub static mut tmp_registers: Registers = Registers::new();

extern "C" {
	pub fn switch_task() -> !;
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

	pub fn init_multitasking(stack_addr: VirtAddr, heap_addr: VirtAddr) {
		let mut task = Task::new();
		unsafe {
			core::arch::asm!(
				"mov {cr3}, cr3",
				"pushf",
				"mov {eflags}, [esp]",
				"popf",
				cr3 = out(reg) task.regs.cr3,
				eflags = out(reg) task.regs.eflags
			);
			MASTER_PROCESS.state = Status::Run;
			MASTER_PROCESS.setup_kernel_stack(PAGE_WRITABLE);
			page_directory.claim_index_page_table(
				KSTACK_ADDR as usize >> 22,
				PAGE_WRITABLE
			);
			change_kernel_stack(&MASTER_PROCESS);
			MASTER_PROCESS.stack = <MemoryZone as Stack>::init_addr(
				stack_addr,
				0x1000,
				PAGE_WRITABLE,
				false
			);
			MASTER_PROCESS.heap = <MemoryZone as Heap>::init_addr(
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
	core::arch::asm!(
		"mov eax, [esp]",
		"add esp, 4",
		"call eax",
		"mov esp, {}",
		"cli",
		"jmp _end_handler",
		const KSTACK_ADDR,
		options(noreturn),
	);
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
	_rst();
	schedule_task();
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
	_rst();
	schedule_task();
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
	TASKLIST.push(old_task);
	_rst();
}

use crate::proc::change_kernel_stack;

#[no_mangle]
pub unsafe extern "C" fn schedule_task() -> ! {
	_cli();
	loop {
		let new_task: &mut Task = Task::get_running_task();
		let process: &mut Process = &mut *new_task.process;
		// TODO: IF SIGNAL JUMP ?
		if process.signals.len() > 0 {
			do_signal(new_task);
		}
		if new_task.state != TaskStatus::Interruptible {
			// Copy registers to shared memory
			tmp_registers = new_task.regs;
			change_kernel_stack(process);
			// Avoid using stack below that
			_rst();
			core::arch::asm!("jmp switch_task");
			// never goes there
		}
		let skipped_task: Task = TASKLIST.pop();
		TASKLIST.push(skipped_task);
	}
}
