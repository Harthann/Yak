use core::fmt;

use crate::KHEAP;
use crate::vec::Vec;
use crate::memory::{MemoryZone, Stack};
use crate::memory::paging::{PAGE_WRITABLE, free_pages};
use crate::memory::allocator::Box;

use crate::proc::Id;
use crate::proc::signal::{Signal, SignalType};

pub static mut NEXT_PID: Id = 0;
pub static mut MASTER_PROCESS: *mut Process = core::ptr::null_mut();

#[derive(Debug)]
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

	pub fn print_tree(&self) {
		crate::kprintln!("{}", self);
		for process in self.childs.iter() {
			process.print_tree();
		}
	}

	pub fn search_from_pid(&mut self, pid: Id) -> Result<&mut Process, ()> {
		if self.pid == pid {
			return Ok(self);
		}
		for process in self.childs.iter_mut() {
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

	pub unsafe fn zombify(&mut self) {
		if self.parent.is_null() {
			todo!();
		}
		let parent: &mut Process = &mut *self.parent;
		while self.childs.len() > 0 {
			/* TODO: DON'T MOVE THREADS AND REMOVE THEM */
			let res = self.childs.pop();
			if res.is_none() {
				todo!();
			}
			parent.childs.push(res.unwrap());
			let len = parent.childs.len();
			parent.childs[len - 1].parent = self.parent;
		}
		/* Don't remove and wait for the parent process to do wait4() -> Zombify */
		self.status = Status::Zombie;
		Signal::send_to_process(parent, self.pid, SignalType::SIGCHLD);
		free_pages(self.stack.offset, self.stack.size / 0x1000);
	}

	pub unsafe fn remove(&mut self) {
		let parent: &mut Process = &mut *self.parent;
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
	}

	pub unsafe fn get_signal(&mut self) -> Result<Signal, ()> {
		if self.signals.len() > 0 {
			Ok(*self.signals.pop().unwrap().as_mut())
		} else {
			Err(())
		}
	}

	pub unsafe fn get_signal_from_pid(&mut self, pid: Id) -> Result<Signal, ()> {
		let mut i = 0;
		while i < self.signals.len() {
			if self.signals[i].as_mut().sender == pid {
				return Ok(*self.signals.remove(i));
			}
			i += 1;
		}
		Err(())
	}
}

impl fmt::Display for Process {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:10} - {:10} - {:?}", self.pid, self.owner, self.status)
	}
}

pub unsafe fn get_running_process() -> *mut Process {
	//TODO: remove
	MASTER_PROCESS
//	&mut *(*RUNNING_TASK).process
}

pub unsafe fn zombify_running_process() {
//	let process: &mut Process = &mut *(*RUNNING_TASK).process;
//	process.zombify();
}

pub unsafe fn get_signal_running_process(pid: Id) -> Result<Signal, ()> {
	//TODO: remove
	(*MASTER_PROCESS).get_signal()
//	let process: &mut Process = &mut *(*RUNNING_TASK).process;
//	if pid == -1  {
//		process.get_signal()
//	} else {
//		process.get_signal_from_pid(pid)
//	}
}

pub unsafe fn print_all_process() {
	crate::kprintln!("       PID        OWNER   STATUS");
	(*MASTER_PROCESS).print_tree();
}
