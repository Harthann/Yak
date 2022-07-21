use crate::paging::VirtAddr;
use core::alloc::{Layout, GlobalAlloc};
use crate::allocator::{Allocator, align_up};

impl Allocator for LinkedListAllocator {
	unsafe fn init(&mut self, heap_start: VirtAddr, heap_size: usize) {
		self.add_free_region(heap_start, heap_size);
	}
}

unsafe impl GlobalAlloc for LinkedListAllocator {
	unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
		let vaddr: u32 = self as *const Self as u32;
		let mut mut_self: &mut Self = &mut *(vaddr as *mut _);

		let (size, align) = LinkedListAllocator::size_align(layout);
		let mut allocator = mut_self;

		if let Some((region, alloc_start)) = allocator.find_region(size, align) { 
			let alloc_end = alloc_start.checked_add(size as u32).expect("overflow");
			let excess_size = region.end_addr() - alloc_end;
			if excess_size > 0 {
				allocator.add_free_region(alloc_end, excess_size as usize);
			}
			alloc_start as *mut u8
		} else {
			core::ptr::null_mut()
		}
	}

	unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
		let vaddr: u32 = self as *const Self as u32;
		let mut mut_self: &mut Self = &mut *(vaddr as *mut _);

		let (size, _) = LinkedListAllocator::size_align(layout);
		mut_self.add_free_region(ptr as u32, size)
	}
}

struct ListNode {
	size: usize,
	next: Option<&'static mut ListNode>
}

impl ListNode {
	const fn new(size: usize) -> Self {
		Self {size, next: None}
	}

	fn start_addr(&self) -> VirtAddr {
		self as *const Self as VirtAddr
	}

	fn end_addr(&self) -> VirtAddr {
		self.start_addr() + self.size as u32
	}
}

pub struct LinkedListAllocator {
	head: ListNode
}

impl LinkedListAllocator {
	pub const fn new() -> Self {
		Self {head: ListNode::new(0)}
	}

	fn size_align(layout: Layout) -> (usize, usize) {
			let layout = layout
				.align_to(core::mem::align_of::<ListNode>())
				.expect("adjusting alignment failed")
				.pad_to_align();
			let size = layout.size().max(core::mem::size_of::<ListNode>());
			(size, layout.align())
	}

	fn alloc_from_region(region: &ListNode, size: usize, align: usize) -> Result<VirtAddr, ()> {
		let alloc_start = align_up(region.start_addr(), align);
		let alloc_end = alloc_start.checked_add(size as u32).ok_or(())?;
		if alloc_end > region.end_addr() {
			return Err(());
		}
		let excess_size = region.end_addr() - alloc_end;
		if excess_size > 0 && (excess_size as usize) < core::mem::size_of::<ListNode>() {
			return Err(());
		}
		Ok(alloc_start as VirtAddr)
	}

	fn find_region(&mut self, size: usize, align: usize) -> Option<(&'static mut ListNode, VirtAddr)> {
		let mut current = &mut self.head;

		while let Some(ref mut region) = current.next {
			if let Ok(alloc_start) = Self::alloc_from_region(&region, size, align) {
				let next = region.next.take();
				let ret = Some((current.next.take().unwrap(), alloc_start));
				current.next = next;
				return ret;
			} else {
				current = current.next.as_mut().unwrap();
			}
		}
		None
	}

	unsafe fn add_free_region(&mut self, addr: VirtAddr, size: usize) {
		/* TODO: Check if the region is large enough */
		let mut node: ListNode = ListNode::new(size);
		node.next = self.head.next.take();
		let node_ptr: *mut ListNode = addr as *mut ListNode;
		node_ptr.write(node);
		self.head.next = Some(&mut *node_ptr);
	}
}
