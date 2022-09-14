use crate::memory::MemoryZone;
use crate::vec::Vec;
use crate::VirtAddr;

enum Status {
	Run,
	Zombie,
	Thead
}

type Id = u32;

struct Signal {
}

struct Process {
	pid: Id,
	status: Status,
	parent: *const Process,
	childs: Vec<*mut Process>,
	stack: MemoryZone,
	heap: MemoryZone,
	signals: Vec<Signal>, /* TODO: VecDeque ? */
	owner: Id
}

enum State {
}

struct ProcessControlBlock {
	addr: VirtAddr,
	name: String,
	state: State,
	stack: MemoryZone,
	next_task: *mut ProcessControlBlock
}

pub fn		exec_fn(addr: VirtAddr, func: VirtAddr, size: u32) {
}
