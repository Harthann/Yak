use crate::KHEAP;
use crate::vec::Vec;
use crate::memory::{MemoryZone, Stack};
use crate::memory::paging::{PAGE_WRITABLE, free_pages};
use crate::memory::allocator::Box;

use crate::proc::Id;
use crate::proc::task::RUNNING_TASK;
use crate::proc::signal::Signal;

pub static mut NEXT_PID: Id = 0;
pub static mut MASTER_PROCESS: Process = Process::new();

pub enum Status {
	Disable,
	Run,
	Zombie,
	Thread
}

pub struct Process {
	pub pid: Id,
	pub status: Status,
	pub parent: *mut Process,
	pub childs: Vec<Box<Process>>,
	pub stack: MemoryZone,
	pub heap: MemoryZone,
	pub signals: Vec<Box<Signal>>, /* TODO: VecDeque ? */
	pub owner: Id
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
		if self.parent.is_null() {
			todo!();
		}
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

pub unsafe fn remove_running_process() {
	let process: &mut Process = &mut *(*RUNNING_TASK).process;
	process.remove();
}
