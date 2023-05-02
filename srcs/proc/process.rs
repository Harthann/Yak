use core::fmt;
use core::ptr::copy_nonoverlapping;

use crate::boot::KERNEL_BASE;

use crate::boxed::Box;
use crate::memory::paging::free_pages;
use crate::memory::{Heap, MemoryZone, Stack};
use crate::vec::Vec;

use crate::proc::task::Task;

use crate::proc::signal::{Signal, SignalHandler, SignalType};
use crate::proc::Id;

use crate::errno::ErrNo;

use crate::memory::paging::page_directory::PageDirectory;
use crate::memory::paging::page_table::PageTable;
use crate::memory::paging::{
	page_directory,
	PAGE_PRESENT,
	PAGE_USER,
	PAGE_WRITABLE
};
use crate::memory::PhysAddr;

use crate::user::{USER_HEAP_ADDR, USER_STACK_ADDR};
use crate::KSTACK_ADDR;

use crate::fs::FileInfo;
use alloc::sync::Arc;

pub static mut NEXT_PID: Id = 0;
pub static mut MASTER_PROCESS: Process = Process::new();

pub type Pid = Id;

#[derive(Debug, PartialEq)]
pub enum Status {
	Disable,
	Run,
	Zombie,
	Thread
}

pub const MAX_FD: usize = 32;
pub struct Process {
	pub pid:             Pid,
	pub state:           Status,
	pub parent:          *mut Process,
	pub childs:          Vec<Box<Process>>,
	pub stack:           MemoryZone,
	pub heap:            MemoryZone,
	pub kernel_stack:    MemoryZone,
	pub fds:             [Option<Arc<FileInfo>>; MAX_FD],
	pub signals:         Vec<Signal>,
	pub signal_handlers: Vec<SignalHandler>,
	pub owner:           Id
}

const DEFAULT_FILE: Option<Arc<FileInfo>> = None;
impl Process {
	pub const fn new() -> Self {
		Self {
			pid:             0,
			state:           Status::Disable,
			parent:          core::ptr::null_mut(),
			childs:          Vec::new(),
			stack:           MemoryZone::new(),
			heap:            MemoryZone::new(),
			kernel_stack:    MemoryZone::new(),
			fds:             [DEFAULT_FILE; MAX_FD],
			signals:         Vec::new(),
			signal_handlers: Vec::new(),
			owner:           0
		}
	}

	pub fn get_nb_subprocess(&self) -> usize {
		let mut ret: usize = 0;
		for process in self.childs.iter() {
			ret += 1;
			ret += process.get_nb_subprocess()
		}
		ret
	}

	pub fn print_tree(&self) {
		crate::kprintln!("{}", self);
		for process in self.childs.iter() {
			process.print_tree();
		}
	}

	pub fn search_from_pid(&mut self, pid: Id) -> Result<&mut Process, ErrNo> {
		if self.pid == pid {
			return Ok(self);
		}
		for process in self.childs.iter_mut() {
			let res = process.search_from_pid(pid);
			if res.is_ok() {
				return res;
			}
		}
		Err(ErrNo::ESRCH)
	}

	// TODO: next_pid need to check overflow and if other pid is available
	pub unsafe fn init(&mut self, parent: &mut Process) {
		self.pid = NEXT_PID;
		self.state = Status::Run;
		self.parent = parent;
		self.owner = parent.owner;
		NEXT_PID += 1;
	}

	pub fn setup_stack(&mut self, size: usize, flags: u32, kphys: bool) {
		self.stack = <MemoryZone as Stack>::init(size, flags, kphys);
	}

	pub fn setup_heap(&mut self, size: usize, flags: u32, kphys: bool) {
		self.heap = <MemoryZone as Heap>::init_no_allocator(size, flags, kphys);
	}

	pub fn setup_kernel_stack(&mut self, flags: u32) {
		self.kernel_stack = <MemoryZone as Stack>::init(0x1000, flags, false);
	}

	pub unsafe fn copy_mem(&mut self, parent: &mut Process) {
		copy_nonoverlapping(
			parent.stack.offset as *const u8,
			self.stack.offset as *mut u8,
			self.stack.size
		);
		copy_nonoverlapping(
			parent.heap.offset as *const u8,
			self.heap.offset as *mut u8,
			self.heap.size
		);
		copy_nonoverlapping(
			parent.kernel_stack.offset as *const u8,
			self.kernel_stack.offset as *mut u8,
			self.kernel_stack.size
		);
	}

	pub unsafe fn zombify(&mut self, wstatus: i32) {
		if self.parent.is_null() {
			todo!();
		}
		let parent: &mut Process = &mut *self.parent;
		while self.childs.len() > 0 {
			// TODO: DON'T MOVE THREADS AND REMOVE THEM
			let res = self.childs.pop();
			if res.is_none() {
				todo!();
			}
			parent.childs.push(res.unwrap());
			let len = parent.childs.len();
			parent.childs[len - 1].parent = self.parent;
		}
		// Don't remove and wait for the parent process to do wait4() -> Zombify
		self.state = Status::Zombie;
		Signal::send_to_process(parent, self.pid, SignalType::SIGCHLD, wstatus);
	}

	pub unsafe fn remove(&mut self) {
		let parent: &mut Process = &mut *self.parent;
		let mut i = 0;
		while i < parent.childs.len() {
			let ptr: *mut Process = parent.childs[i].as_mut();
			if ptr == &mut *self {
				break;
			}
			i += 1;
		}
		if i == parent.childs.len() {
			todo!(); // Problem
		}
		free_pages(self.stack.offset, self.stack.size / 0x1000);
		free_pages(self.heap.offset, self.heap.size / 0x1000);
		free_pages(self.kernel_stack.offset, self.kernel_stack.size / 0x1000);
		parent.childs.remove(i);
	}

	pub unsafe fn get_signal(
		&mut self,
		signal: SignalType
	) -> Result<Signal, ErrNo> {
		let mut i = 0;
		while i < self.signals.len() {
			if self.signals[i].sigtype == signal {
				return Ok(self.signals.remove(i));
			}
			i += 1;
		}
		Err(ErrNo::EAGAIN)
	}

	pub unsafe fn get_signal_from_pid(
		&mut self,
		pid: Id,
		signal: SignalType
	) -> Result<Signal, ErrNo> {
		MASTER_PROCESS.search_from_pid(pid)?; // Return ErrNo::ESRCH if doesn't exist
		let mut i = 0;
		while i < self.signals.len() {
			if self.signals[i].sender == pid
				&& self.signals[i].sigtype == signal
			{
				return Ok(self.signals.remove(i));
			}
			i += 1;
		}
		Err(ErrNo::EAGAIN)
	}

	pub fn get_running_process() -> &'static mut Self {
		unsafe { &mut *Task::get_running_task().process }
	}

	pub unsafe fn get_signal_running_process(
		pid: Id,
		signal: SignalType
	) -> Result<Signal, ErrNo> {
		let process: &mut Process = Process::get_running_process();
		if pid == -1 {
			process.get_signal(signal)
		} else if pid > 0 {
			process.get_signal_from_pid(pid, signal)
		} else if pid == 0 {
			todo!();
		} else {
			todo!();
		}
	}

	pub unsafe fn setup_pagination(&self, user: bool) -> &'static mut PageDirectory {
		let parent: &Process = &(*self.parent);
		let kernel_pt_paddr: PhysAddr =
			get_paddr!(page_directory.get_page_table(KERNEL_BASE >> 22).get_vaddr());

		let flag_user;
		let heap_addr;
		let stack_addr;
		if user {
			flag_user = PAGE_USER;
			heap_addr = USER_HEAP_ADDR;
			stack_addr = USER_STACK_ADDR;
		} else {
			flag_user = 0;
			heap_addr = parent.heap.offset;
			stack_addr = parent.stack.offset;
		}
		let page_dir: &'static mut PageDirectory =
			PageDirectory::new(PAGE_WRITABLE | flag_user);
		let process_heap: &'static mut PageTable = PageTable::new();
		let process_stack: &'static mut PageTable = PageTable::new();
		let process_kernel_stack: &'static mut PageTable = PageTable::new();
		// Setup heap + prg
		page_dir.set_entry(
			heap_addr as usize >> 22,
			get_paddr!(process_heap as *const _)
				| parent.heap.flags
				| flag_user | PAGE_PRESENT
		);
		// Setup stack
		page_dir.set_entry(
			stack_addr as usize >> 22,
			get_paddr!(process_stack as *const _)
				| parent.stack.flags
				| flag_user | PAGE_PRESENT
		);
		// TODO: Kernel must not be writable but need task page so map it in
		// another page_table ?
		page_dir.set_entry(
			KERNEL_BASE >> 22,
			kernel_pt_paddr | PAGE_WRITABLE | PAGE_PRESENT | flag_user
		);
		crate::kprintln!("{}", page_dir.get_entry(KERNEL_BASE >> 22));
		crate::kprintln!("{}", page_dir.get_entry(KERNEL_BASE >> 22));
		page_dir.set_entry(
			KSTACK_ADDR as usize >> 22,
			get_paddr!(process_kernel_stack as *const _)
				| parent.kernel_stack.flags
				| flag_user | PAGE_PRESENT
		);
		crate::kprintln!("{:p}", page_dir);
		//for i in 0..1024 {
		//	crate::kprintln!("{}: {}", i, page_dir.get_entry(i));
		//}
		crate::kprintln!("{}", page_dir.get_entry(KERNEL_BASE >> 22));
		// Setup stack and heap
		process_heap.new_index_frame(
			(heap_addr as usize & 0x3ff000) >> 12,
			get_paddr!(self.heap.offset),
			PAGE_WRITABLE | flag_user
		);
		process_stack.new_index_frame(
			(stack_addr as usize & 0x3ff000) >> 12,
			get_paddr!(self.stack.offset),
			PAGE_WRITABLE | flag_user
		);
		process_kernel_stack.new_index_frame(
			(KSTACK_ADDR as usize & 0x3ff000) >> 12,
			get_paddr!(self.kernel_stack.offset),
			PAGE_WRITABLE | flag_user
		);
		refresh_tlb!();
		page_dir
	}

	pub unsafe fn get_nb_process() -> usize {
		MASTER_PROCESS.get_nb_subprocess() + 1
	}

	pub unsafe fn print_all_process() {
		crate::kprintln!("       PID        OWNER   STATUS");
		MASTER_PROCESS.print_tree();
	}
}

impl fmt::Display for Process {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:10} - {:10} - {:?}", self.pid, self.owner, self.state)
	}
}
