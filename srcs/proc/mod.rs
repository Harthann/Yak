struct Process {
	pid: usize,
	status: u8,
	parent: &Process,
	childs: Vec<&Process>,
	stack: &MemoryZone,
	heap: &MemoryZone,
	signals: Vec<&Process>, /* TODO: VecDeque ? */
	owner: usize
}
