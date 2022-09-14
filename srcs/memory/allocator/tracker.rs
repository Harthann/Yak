/* Allocation tracking */
pub struct Tracker {
	pub allocation:			usize,
	pub allocated_bytes:	usize,
	pub freed:				usize,
	pub freed_bytes:		usize
}

pub static mut TRACKER: Tracker = Tracker {
	allocation: 0,
	allocated_bytes: 0,
	freed: 0,
	freed_bytes: 0
};

pub static mut KTRACKER: Tracker = Tracker {
	allocation: 0,
	allocated_bytes: 0,
	freed: 0,
	freed_bytes: 0
};

pub fn memory_state() {
	unsafe {
		crate::kprintln!("\nAllocation:  {} for {} bytes", KTRACKER.allocation, KTRACKER.allocated_bytes);
		crate::kprintln!("Free:        {} for {} bytes", KTRACKER.freed, KTRACKER.freed_bytes);
	}
}
