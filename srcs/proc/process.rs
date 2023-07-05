use core::fmt;
use core::ptr::copy_nonoverlapping;

use crate::boot::KERNEL_BASE;

use crate::alloc::string::String;

use crate::memory::{MemoryZone, TypeZone};
use crate::vec::Vec;

use crate::alloc::collections::btree_map::BTreeMap;
use crate::alloc::collections::LinkedList;
use crate::utils::arcm::KArcm;

use crate::proc::task::{Task, TASK_STACK};

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
use crate::utils::arcm::Arcm;

use crate::user::{USER_HEAP_ADDR, USER_STACK_ADDR};
use crate::KSTACK_ADDR;

use crate::fs::FileInfo;
use alloc::sync::Arc;

pub type Pid = Id;

pub static mut NEXT_PID: Pid = 0;
pub static mut PROCESS_TREE: BTreeMap<Pid, KArcm<Process>> = BTreeMap::new();

#[derive(Debug, PartialEq)]
pub enum Status {
	Disable,
	Run,
	Zombie,
	Thread
}

/// Arcm is needed to protect MemoryZone only if the memory zone is shared between threads
/// otherwise it will be useless.
pub const MAX_FD: usize = 32;

pub struct Process {
	pub pid:             Pid,
	pub exe:             String,
	pub state:           Status,
	pub parent:          Option<KArcm<Process>>,
	pub childs:          Vec<KArcm<Process>>,
	pub stack:           MemoryZone,
	pub heap:            MemoryZone,
	pub kernel_stack:    MemoryZone,
	pub mem_map:         LinkedList<Arcm<MemoryZone>>,
	pub fds:             [Option<Arc<FileInfo>>; MAX_FD],
	pub signals:         Vec<Signal>,
	pub signal_handlers: Vec<SignalHandler>,
	pub page_tables:     Vec<&'static mut PageTable>,
	pub pd:              *mut PageDirectory,
	pub owner:           Id
}

const DEFAULT_FILE: Option<Arc<FileInfo>> = None;
impl Process {
	pub fn new() -> Self {
		Self {
			pid:             0,
			exe:             String::new(),
			state:           Status::Disable,
			parent:          None,
			childs:          Vec::new(),
			stack:           MemoryZone::new(),
			heap:            MemoryZone::new(),
			kernel_stack:    MemoryZone::new(),
			mem_map:         LinkedList::new(),
			fds:             [DEFAULT_FILE; MAX_FD],
			signals:         Vec::new(),
			signal_handlers: Vec::new(),
			page_tables:     Vec::new(),
			pd:              0x0 as *mut PageDirectory,
			owner:           0
		}
	}

	pub fn get_nb_subprocess(&self) -> usize {
		let mut ret: usize = 0;
		for process in self.childs.iter() {
			ret += 1;
			ret += process.lock().get_nb_subprocess()
		}
		ret
	}

	pub fn print_tree() {
		unsafe {
			for process in PROCESS_TREE.values() {
				crate::kprintln!("{}", *process.lock());
			}
		}
	}

	pub fn search_from_pid(pid: Id) -> Result<KArcm<Process>, ErrNo> {
		unsafe {
			match PROCESS_TREE.get_mut(&pid) {
				Some(process) => Ok(process.clone()),
				None => Err(ErrNo::ESRCH)
			}
		}
	}

	// TODO: next_pid need to check overflow and if other pid is available
	pub unsafe fn init(&mut self, parent: &KArcm<Process>) {
		self.pid = NEXT_PID;
		self.exe = parent.lock().exe.clone();
		self.state = Status::Run;
		self.parent = Some(parent.clone());
		self.owner = parent.lock().owner;
		NEXT_PID += 1;
	}

	pub fn setup_stack(&mut self, size: usize, flags: u32, kphys: bool) {
		self.stack = MemoryZone::init(TypeZone::Stack, size, flags, kphys);
	}

	pub fn setup_heap(&mut self, size: usize, flags: u32, kphys: bool) {
		self.heap = MemoryZone::init(TypeZone::Heap, size, flags, kphys);
	}

	pub fn setup_kernel_stack(&mut self, flags: u32) {
		self.kernel_stack =
			MemoryZone::init(TypeZone::Stack, 0x1000, flags, false);
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

	pub unsafe fn zombify(pid: Pid, wstatus: i32) {
		let binding = Process::search_from_pid(pid).unwrap();
		let binding_parent = {
			let process = binding.lock();
			match &process.parent {
				Some(x) => Process::search_from_pid(x.lock().pid),
				None => panic!("Process has no parent")
			}
		}
		.unwrap();
		let mut process = binding.lock();
		let mut parent = binding_parent.lock();
		while process.childs.len() > 0 {
			// TODO: DON'T MOVE THREADS AND REMOVE THEM
			let res = process.childs.pop();
			if res.is_none() {
				todo!();
			}
			parent.childs.push(res.unwrap());
			let len = parent.childs.len();
			parent.childs[len - 1].lock().parent = Some(binding_parent.clone());
		}
		// Transfer signals from childs to parent
		while process.signals.len() != 0 {
			let signal = process.signals.remove(0);
			if signal.sigtype == SignalType::SIGCHLD {
				parent.signals.push(signal);
			}
		}
		// Don't remove and wait for the parent process to do wait4() -> Zombify
		process.state = Status::Zombie;
		Signal::send_to_process(
			&mut parent,
			process.pid,
			SignalType::SIGCHLD,
			wstatus
		);
	}

	pub unsafe fn remove(pid: Pid) {
		let binding = Process::search_from_pid(pid).unwrap();
		let binding_parent = {
			let process = binding.lock();
			match &process.parent {
				Some(x) => Process::search_from_pid(x.lock().pid),
				None => panic!("Process has no parent")
			}
		}
		.unwrap();
		let mut parent = binding_parent.lock();
		let mut i = 0;
		while i < parent.childs.len() {
			if parent.childs[i].lock().pid == pid {
				break;
			}
			i += 1;
		}
		if i == parent.childs.len() {
			todo!(); // Problem
		}
		let mut process = binding.lock();
		if process.owner != 0 {
			use crate::memory::paging::bitmap;
			let pd = &mut *process.pd;
			for i in &process.page_tables {
				let vaddr = i.get_vaddr() as usize;
				bitmap::physmap_as_mut().free_page(get_paddr!(vaddr));
				page_directory
					.get_page_table(vaddr >> 22)
					.set_entry((vaddr & 0x3ff000) >> 12, 0);
			}
			let vaddr = pd.get_vaddr() as usize;
			bitmap::physmap_as_mut().free_page(get_paddr!(vaddr));
			page_directory
				.get_page_table(vaddr >> 22)
				.set_entry((vaddr & 0x3ff000) >> 12, 0);
		}
		parent.childs.remove(i);
		PROCESS_TREE.remove(&pid);
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
		Process::search_from_pid(pid)?; // Return ErrNo::ESRCH if doesn't exist
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

	pub fn get_running_process() -> KArcm<Self> {
		unsafe { Task::get_running_task().process.clone() }
	}

	pub unsafe fn get_signal_running_process(
		pid: Id,
		signal: SignalType
	) -> Result<Signal, ErrNo> {
		let binding = Process::get_running_process();
		let mut process = binding.lock();
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

	pub unsafe fn setup_pagination(&mut self) -> &'static mut PageDirectory {
		let parent = match &self.parent {
			Some(x) => x.lock(),
			None => panic!("Process has no parent")
		};
		let kernel_pt_paddr: PhysAddr = get_paddr!(page_directory
			.get_page_table(KERNEL_BASE >> 22)
			.get_vaddr());

		let page_dir: &'static mut PageDirectory =
			PageDirectory::new(PAGE_WRITABLE | PAGE_USER);
		let process_heap: &'static mut PageTable = PageTable::new();
		let process_stack: &'static mut PageTable = PageTable::new();
		let process_kernel_stack: &'static mut PageTable = PageTable::new();
		// Setup heap + prg
		page_dir.set_entry(
			USER_HEAP_ADDR as usize >> 22,
			get_paddr!(process_heap as *const _)
				| parent.heap.flags
				| PAGE_USER | PAGE_PRESENT
		);
		// Setup stack
		page_dir.set_entry(
			USER_STACK_ADDR as usize >> 22,
			get_paddr!(process_stack as *const _)
				| parent.stack.flags
				| PAGE_USER | PAGE_PRESENT
		);
		// TODO: Kernel must not be writable but need task page so map it in
		// another page_table ?
		page_dir.set_entry(
			KERNEL_BASE >> 22,
			kernel_pt_paddr | PAGE_PRESENT | PAGE_USER
		);
		page_dir.set_entry(
			KSTACK_ADDR as usize >> 22,
			get_paddr!(process_kernel_stack as *const _)
				| parent.kernel_stack.flags
				| PAGE_USER | PAGE_PRESENT
		);
		// Setup stack and heap
		process_heap.new_index_frame(
			(USER_HEAP_ADDR as usize & 0x3ff000) >> 12,
			get_paddr!(self.heap.offset),
			PAGE_WRITABLE | PAGE_USER
		);
		process_stack.new_index_frame(
			(USER_STACK_ADDR as usize & 0x3ff000) >> 12,
			get_paddr!(self.stack.offset),
			PAGE_WRITABLE | PAGE_USER
		);
		process_kernel_stack.new_index_frame(
			(TASK_STACK.offset as usize & 0x3ff000) >> 12,
			get_paddr!(TASK_STACK.offset),
			PAGE_WRITABLE | PAGE_USER
		);
		process_kernel_stack.new_index_frame(
			(KSTACK_ADDR as usize & 0x3ff000) >> 12,
			get_paddr!(self.kernel_stack.offset),
			PAGE_WRITABLE | PAGE_USER
		);
		self.pd = page_dir;
		self.page_tables.push(process_heap);
		self.page_tables.push(process_stack);
		self.page_tables.push(process_kernel_stack);
		refresh_tlb!();
		page_dir
	}

	pub fn get_nb_process() -> usize {
		unsafe { PROCESS_TREE.len() }
	}

	pub unsafe fn print_all_process() {
		crate::kprintln!(
			"       PID                   NAME        OWNER   STATUS"
		);
		Self::print_tree();
	}

	pub fn add_memory_zone(&mut self, mz: Arcm<MemoryZone>) {
		self.mem_map.push_back(mz);
	}
}

impl fmt::Display for Process {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(
			f,
			"{:10} - {:>20} - {:10} - {:?}",
			self.pid, self.exe, self.owner, self.state
		)
	}
}
