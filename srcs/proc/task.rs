//use crate::proc::process::{Process, MASTER_PROCESS, NEXT_PID, Status};
use crate::utils::queue::Queue;
use crate::interrupts::Registers;
use crate::proc::wrapper_fn;
use crate::memory::VirtAddr;
use crate::memory::paging::{PAGE_WRITABLE, alloc_page};

pub static mut TASKLIST: Queue<Task> = Queue::new();

#[derive(Copy, Clone)]
pub struct Task {
	pub regs: Registers
}

impl Task {
	pub const fn new() -> Self {
		Self {
			regs: Registers::new()
		}
	}

	pub unsafe extern "C" fn init(&mut self, flags: u32, page_dir: u32) {//, process: *mut Process) {
		self.regs.eip = wrapper_fn as VirtAddr;
		self.regs.eflags = flags;
		self.regs.cr3 = page_dir;
		self.regs.esp = alloc_page(PAGE_WRITABLE).unwrap() + 0x1000;
	}
}

extern "C" {
	pub fn switch_task(regs: *const Registers);
}

#[no_mangle]
pub static mut STACK_TASK_SWITCH: VirtAddr = 0;

use crate::memory::allocator::Box;
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
		TASKLIST.push(task);
	}
}

use crate::wrappers::{_cli, _rst};

#[no_mangle]
pub unsafe extern "C" fn next_task(regs: &mut Registers) -> !{
	_cli();
	let mut task = TASKLIST.pop();
	task.regs = *regs;
//	crate::kprintln!("prev_regs: {:#x?}", *regs);
	TASKLIST.push(task);
	let res = &TASKLIST.peek();
	if res.is_none() {
		todo!();
	}
	let mut regs = res.unwrap().regs;
//	crate::kprintln!("new_regs: {:#x?}", regs);
	_rst();
	switch_task(&mut regs);
	/* Never goes there */
	loop {}
}
