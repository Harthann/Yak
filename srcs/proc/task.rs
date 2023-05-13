use core::ptr;

use crate::interrupts::Registers;
use crate::memory::paging::page_directory;
use crate::memory::{Heap, MemoryZone, Stack, VirtAddr};
use crate::proc::process::{Process, Status, MASTER_PROCESS, NEXT_PID};
use crate::proc::signal::{SignalHandler, SignalType};

use crate::utils::queue::Queue;
use crate::vec::Vec;
use crate::wrappers::{_cli, _rst};

use crate::memory::paging::PAGE_WRITABLE;
use crate::{KALLOCATOR, KSTACK_ADDR};

use crate::alloc::collections::vec_deque::VecDeque;

pub static mut TASKLIST: VecDeque<Task> = VecDeque::new();

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

	pub fn init_multitasking(stack_addr: VirtAddr) {
		let mut task = Task::new();
		unsafe {
			core::arch::asm!(
				"mov {cr3}, cr3",
				cr3 = out(reg) task.regs.cr3
			);
			MASTER_PROCESS.state = Status::Run;
			MASTER_PROCESS.setup_kernel_stack(PAGE_WRITABLE);
			page_directory
				.claim_index_page_table(
					KSTACK_ADDR as usize >> 22,
					PAGE_WRITABLE
				)
				.expect("Could not claim kernel stack page");
			change_kernel_stack(&MASTER_PROCESS);
			MASTER_PROCESS.stack = <MemoryZone as Stack>::init_addr(
				stack_addr,
				0x1000,
				PAGE_WRITABLE,
				false
			);
			MASTER_PROCESS.heap = <MemoryZone as Heap>::init(
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
			TASKLIST.push_back(task);
		}
	}

	pub unsafe fn remove_task_from_process(process: &mut Process) {
		let process_ptr: *mut Process = &mut *process;
		let len = TASKLIST.len();
		let mut i = 0;
		while i < len {
			let task: &mut Task = Task::get_running_task();
			if task.process != process_ptr {
				TASKLIST.push_back(TASKLIST.pop_front().unwrap());
			} else {
				TASKLIST.pop_front();
			}
			i += 1;
		}
	}

	#[inline]
	pub unsafe fn get_running_task() -> &'static mut Task {
		match TASKLIST.front_mut() {
			Some(x) => x,
			None => todo!()
		}
	}
}

#[naked]
#[no_mangle]
unsafe extern "C" fn wrapper_handler() {
	core::arch::asm!(
		"mov eax, [esp]",
		"add esp, 4",
		"call eax",
		"cli",
		"jmp _end_handler",
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

#[naked]
#[no_mangle]
unsafe extern "C" fn swap_task() -> ! {
	core::arch::asm!(
		"pusha",

		"mov eax, cr3",
		"push eax",

		"xor eax, eax",
		"mov ax, ds",
		"push eax",

		"mov eax, offset page_directory - {}",
		"mov ebx, cr3",
		"cmp eax, ebx",
		"je 2f", // if cr3 is kernel don't swap

		"mov cr3, eax",

		"2:",
		"add DWORD PTR[JIFFIES], 1",

		"mov eax, 0x10",
		"mov ds, ax",
		"mov es, ax",
		"mov fs, ax",
		"mov gs, ax",

		"mov eax, esp",

		// (regs: &Registers)
		"push eax",
		"call save_task",
		"pop eax",

		"jmp schedule_task", // never returns
		const crate::boot::KERNEL_BASE,
		options(noreturn)
	)
}

#[no_mangle]
pub unsafe extern "C" fn save_task(regs: &Registers) {
	_cli();
	let mut old_task: Task = TASKLIST.pop_front().unwrap();
	old_task.regs = *regs;
	crate::dprintln!("saved: {:#x?}", regs);
	TASKLIST.push_back(old_task);
	_rst();
}

use crate::pic::end_of_interrupts;
use crate::proc::change_kernel_stack;

#[naked]
#[no_mangle]
pub unsafe extern "C" fn schedule_task() -> ! {
	core::arch::asm!(
		"mov esp, offset task_stack + {}",
		"jmp find_task",
		const STACK_SIZE,
		options(noreturn)
	);
}

#[no_mangle]
unsafe extern "C" fn find_task() -> ! {
	_cli();
	crate::kprintln!("len: {}", TASKLIST.len());
	loop {
		let new_task: &mut Task = Task::get_running_task();
		let process: &mut Process = &mut *new_task.process;
		// TODO: IF SIGNAL JUMP ?
		if process.signals.len() > 0 {
			do_signal(new_task);
		}
		if new_task.state != TaskStatus::Interruptible {
			// Copy registers to shared memory
			let task: Registers = new_task.regs;
			change_kernel_stack(process);
			switch_task(&task);
			// never goes there
		}
		TASKLIST.push_back(TASKLIST.pop_front().unwrap());
	}
}

unsafe fn switch_task(regs: &Registers) -> ! {
	if regs.cr3 != get_paddr!((&page_directory) as *const _) {
		load_cr3!(regs.cr3);
	}
	get_segments!(regs.ds);
	end_of_interrupts(0x20);
	crate::dprintln!("Registers: {:#x?}", regs);
	crate::vga_buffer::hexdump(regs.esp as *const u8, 44);
	_rst();
	if regs.int_no != u32::MAX {
		// new task
		core::arch::asm!(
			"mov esp, {esp}",
			"push {eip}",
			"ret", // Recover sti in wrappers
			esp = in(reg) regs.esp,
			eip = in(reg) regs.eip,
			options(noreturn)
		);
	}
	let old: &mut Registers = &mut *((regs.esp as *mut u8).offset(-40) as *mut _);
	crate::dprintln!("old: {:#x?}", old);
	core::arch::asm!(
		"mov esp, {}",
		"mov edi, DWORD PTR[esp - 32]",
		"mov esi, DWORD PTR[esp - 28]",
		"mov ebp, DWORD PTR[esp - 24]",
		"mov ebx, DWORD PTR[esp - 16]",
		"mov edx, DWORD PTR[esp - 12]",
		"mov ecx, DWORD PTR[esp - 8]",
		"mov eax, DWORD PTR[esp - 4]",
		"add esp, 8", // int_no and err_code
		"iretd", // no sti: iretd enable interrupt itself
		in(reg) regs.esp,
		options(noreturn)
	);
}

const STACK_SIZE: usize = 0x1000;

#[repr(align(4096))]
struct TaskStack([u8; STACK_SIZE]);

#[no_mangle]
#[link_section = ".bss"]
/// Task stack
static mut task_stack: TaskStack = TaskStack([0; STACK_SIZE]);

macro_rules! load_cr3 {
	($cr3: expr) => {
		core::arch::asm!(
			"mov cr3, {}",
			in(reg) $cr3
		);
	}
}

macro_rules! get_segments {
	($ds: expr) => {
		core::arch::asm!(
			"mov ds, ax",
			"mov es, ax",
			"mov fs, ax",
			"mov gs, ax",
			in("eax") $ds
		);
	}
}

use {get_segments, load_cr3};
