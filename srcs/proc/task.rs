use crate::alloc::string::String;
use crate::interrupts::Registers;
use crate::memory::allocator::AllocatorInit;
use crate::memory::paging::page_directory;
use crate::memory::{MemoryZone, TypeZone, VirtAddr};
use crate::proc::process::{Process, Status, NEXT_PID, PROCESS_TREE};
use crate::proc::signal::{SignalHandler, SignalType};
use crate::proc::Pid;

use crate::vec::Vec;
use crate::wrappers::{_cli, _rst};

use crate::memory::paging::PAGE_WRITABLE;
use crate::{KALLOCATOR, KSTACK_ADDR};

use crate::alloc::collections::vec_deque::VecDeque;
use crate::utils::arcm::KArcm;

pub static mut TASKLIST: VecDeque<Task> = VecDeque::new();

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum TaskStatus {
	Running,         // Normal state
	Uninterruptible, // In signal handler, could not be signaled again
	Interruptible    // Waiting for changing state (wait4 - waitpid)
}

pub struct Task {
	pub regs:    Registers,
	pub state:   TaskStatus,
	pub process: KArcm<Process>
}

impl Task {
	pub fn new() -> Self {
		Self {
			regs:    Registers::new(),
			state:   TaskStatus::Running,
			process: KArcm::new(Process::new())
		}
	}

	pub fn init_multitasking(stack_addr: VirtAddr) {
		unsafe {
			// Init kernel heap
			let heap = MemoryZone::init(
				TypeZone::Heap,
				100 * 0x1000,
				PAGE_WRITABLE,
				true
			);
			// Init allocator with addr &mut KALLOCATOR
			KALLOCATOR.init(heap.offset, heap.size);
			let mut task = Task::new();
			core::arch::asm!(
				"mov {}, cr3",
				out(reg) task.regs.cr3
			);
			let mut process: Process = Process::new();
			process.state = Status::Run;
			process.exe = String::from("kernel");
			process.setup_kernel_stack(PAGE_WRITABLE);
			page_directory
				.claim_index_page_table(
					KSTACK_ADDR as usize >> 22,
					PAGE_WRITABLE
				)
				.expect("Could not claim kernel stack page");
			process.stack = MemoryZone::init_addr(
				stack_addr - (crate::STACK_SIZE - 1),
				TypeZone::Stack,
				crate::STACK_SIZE as usize * 5,
				PAGE_WRITABLE,
				false
			);
			process.heap = heap;
			process.childs = Vec::with_capacity(8);
			process.signals = Vec::with_capacity(8);
			process.owner = 0;

			task.process = KArcm::new(process);
			PROCESS_TREE.insert(NEXT_PID, task.process.clone());
			task.process.execute(|mutex| {
				let process = &mutex.lock();
				change_kernel_stack(process);
			});

			TASKLIST.push_back(task);
			NEXT_PID += 1;

			// Init switch_task stack
			TASK_STACK = MemoryZone::init_addr(
				TASK_STACK_OFFSET,
				TypeZone::Stack,
				STACK_SIZE,
				PAGE_WRITABLE,
				false
			);
		}
	}

	pub unsafe fn remove_task_from_process(pid: Pid) {
		let len = TASKLIST.len();
		let mut i = 0;
		while i < len {
			if TASKLIST[i].process.lock().pid == pid {
				TASKLIST.remove(i);
				break;
			}
			i += 1;
		}
	}

	unsafe fn handle_signal(
		regs: &mut Registers,
		handler: &SignalHandler
	) -> ! {
		regs.esp -= core::mem::size_of::<Task>() as u32;
		(regs.esp as *mut Registers).write(*regs);
		regs.int_no = 0; // Reset int_no to return to new func (TODO: DO THIS BETTER)
				 // Setup args (int signal) and handler call
		regs.esp -= 4;
		core::arch::asm!("mov [{esp}], eax",
			esp = in(reg) regs.esp,
			in("eax") handler.signal);
		regs.esp -= 4;
		core::arch::asm!("mov [{esp}], eax",
			esp = in(reg) regs.esp,
			in("eax") handler.handler);
		regs.eip = wrapper_handler as u32;
		_rst();
		schedule_task()
	}

	unsafe fn do_signal(&mut self) {
		let len = self.process.lock().signals.len();
		for i in 0..len {
			if self.state != TaskStatus::Uninterruptible
				&& self.process.lock().signals[i].sigtype == SignalType::SIGKILL
			{
				todo!(); // sys_kill remove task etc.. ?
			} else if self.state == TaskStatus::Running {
				let res = {
					let process = self.process.lock();
					let get_handler =
						|process: &Process| -> Option<SignalHandler> {
							for handler in process.signal_handlers.iter() {
								if handler.signal
									== process.signals[i].sigtype as i32
								{
									return Some(handler.clone());
								}
							}
							None
						};
					get_handler(&process)
				};
				if res.is_some() {
					self.process.lock().signals.remove(i);
					self.state = TaskStatus::Uninterruptible;
					Task::handle_signal(&mut self.regs, &res.unwrap())
				}
			} else if self.state == TaskStatus::Interruptible
				&& self.process.lock().signals[i].sigtype == SignalType::SIGCHLD
			{
				self.state = TaskStatus::Running;
			}
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
		"call jiffies_inc",

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

		"call schedule_task", // never returns
		const crate::boot::KERNEL_BASE,
		options(noreturn)
	)
}

#[no_mangle]
pub unsafe extern "C" fn save_task(regs: &Registers) {
	_cli();
	let mut old_task: Task = TASKLIST.pop_front().unwrap();
	old_task.regs = *regs;
	TASKLIST.push_back(old_task);
	_rst();
}

use crate::pic::end_of_interrupts;
use crate::proc::change_kernel_stack;

#[naked]
#[no_mangle]
pub unsafe extern "C" fn schedule_task() -> ! {
	core::arch::asm!(
		"mov esp, {}",
		"add esp, {}",
		"call find_task",
		const TASK_STACK_OFFSET,
		const STACK_SIZE,
		options(noreturn)
	);
}

#[no_mangle]
unsafe extern "C" fn find_task() -> ! {
	_cli();
	loop {
		let new_task: &mut Task = Task::get_running_task();
		// TODO: IF SIGNAL JUMP ?
		if new_task.process.lock().signals.len() > 0 {
			new_task.do_signal();
		}
		if new_task.state != TaskStatus::Interruptible {
			// Copy registers to shared memory
			let new_regs: Registers = new_task.regs;
			new_task.process.execute(|mutex| {
				let process = &mutex.lock();
				change_kernel_stack(process);
			});
			switch_task(&new_regs);
			// never goes there
		}
		TASKLIST.push_back(TASKLIST.pop_front().unwrap());
	}
}

unsafe fn switch_task(regs: &Registers) -> ! {
	// We must write on _cli counter from the kernel process, so we must do it
	// before switching user context
	_rst();
	// don't println! from here
	if regs.cr3 != get_paddr!((&page_directory) as *const _) {
		load_cr3!(regs.cr3);
	}
	get_segments!(regs.ds);
	end_of_interrupts(0x20);
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
	core::arch::asm!(
		"mov esp, {}",
		"add esp, 8",
		"mov edi, DWORD PTR[esp]",
		"mov esi, DWORD PTR[esp + 4]",
		"mov ebp, DWORD PTR[esp + 8]",
		"mov ebx, DWORD PTR[esp + 16]",
		"mov edx, DWORD PTR[esp + 20]",
		"mov ecx, DWORD PTR[esp + 24]",
		"mov eax, DWORD PTR[esp + 28]",
		"mov esp, DWORD PTR[esp + 12]",
		"add esp, 8", // int_no and err_code
		"iretd", // no sti: iretd enable interrupt itself
		in(reg) regs,
		options(noreturn)
	);
}

const STACK_SIZE: usize = 0x1000;
const TASK_STACK_OFFSET: VirtAddr = 0xffb00000;
pub static mut TASK_STACK: MemoryZone = MemoryZone::new();

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
