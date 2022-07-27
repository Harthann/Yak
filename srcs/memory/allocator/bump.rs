use crate::memory::VirtAddr;
use core::alloc::{GlobalAlloc, Layout};
use crate::memory::allocator::{AllocatorInit, align_up};

impl AllocatorInit for BumpAllocator {
	unsafe fn init(&mut self, heap_start: VirtAddr, heap_size: usize) {
		self.heap_start = heap_start;
		self.heap_end = heap_start + heap_size as u32;
		self.next = heap_start;
	}
}

unsafe impl GlobalAlloc for BumpAllocator {
	unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
		let vaddr: u32 = self as *const Self as u32;
		let mut mut_self: &mut Self = &mut *(vaddr as *mut _);

		let alloc_start = align_up(mut_self.next, layout.align());
		let alloc_end: VirtAddr = match alloc_start.checked_add(layout.size() as u32) {
			Some(end) => end,
			None => return core::ptr::null_mut()
		};

		if alloc_end > mut_self.heap_end {
			core::ptr::null_mut() // out of memory
		} else {
			mut_self.next = alloc_end;
			mut_self.allocations += 1;
			alloc_start as *mut u8
		}
	}

	unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
		let vaddr: u32 = self as *const Self as u32;
		let mut mut_self: &mut Self = &mut *(vaddr as *mut _);

		mut_self.allocations -= 1;
		if mut_self.allocations == 0 {
			mut_self.next = mut_self.heap_start;
		}
	}
}

#[derive(Debug)]
pub struct BumpAllocator {
	heap_start: VirtAddr,
	heap_end: VirtAddr,
	next: VirtAddr,
	allocations: usize
}

impl BumpAllocator {
	pub const fn new() -> Self {
		BumpAllocator {
				heap_start: 0,
				heap_end: 0,
				next: 0,
				allocations: 0
		}
	}
}
