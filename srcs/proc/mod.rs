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

pub fn		exec_fn(_addr: VirtAddr, _func: VirtAddr, _size: u32) {
}
