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
		crate::kprintln!("alloc !");
		let vaddr: u32 = self as *const Self as u32;
		let mut mut_self: &mut Self = &mut *(vaddr as *mut _);

		let (size, align) = LinkedListAllocator::size_align(layout);
		let mut allocator = mut_self;

		if let Some((region, alloc_start)) = allocator.find_region(size, align) { 
			let alloc_end = alloc_start.checked_add(size as u32).expect("overflow");
			let excess_size: usize = (region.end_addr() - alloc_end) as usize;
			if excess_size > 0 {
				allocator.add_free_region(alloc_end, excess_size);
			}
			alloc_start as *mut u8
		} else {
			crate::kprintln!("return null");
			core::ptr::null_mut()
		}
	}

	unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
		let vaddr: u32 = self as *const Self as u32;
		let mut mut_self: &mut Self = &mut *(vaddr as *mut _);

		let (size, _) = LinkedListAllocator::size_align(layout);
		mut_self.add_free_region(ptr as VirtAddr, size)
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
		crate::kprintln!("self.start_addr(): {:#010x} - self.size: {}", self.start_addr(), self.size);
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

	/* Adjust the layout to contain a ListNode */
	fn size_align(layout: Layout) -> (usize, usize) {
			let layout = layout
				.align_to(core::mem::align_of::<ListNode>())
				.expect("adjusting alignment failed")
				.pad_to_align();
			let size = layout.size().max(core::mem::size_of::<ListNode>());
			(size, layout.align())
	}

	/* check if the given region has the size needed */
	fn alloc_from_region(region: &ListNode, size: usize, align: usize)
		-> Result<VirtAddr, ()> {
		let alloc_start = align_up(region.start_addr(), align);
		let alloc_end = alloc_start.checked_add(size as u32).ok_or(())?;
		crate::kprintln!("size: {}", size);
		crate::kprintln!("alloc_start: {:#010x} - alloc_end: {:#010x}", alloc_start, alloc_end);
		crate::kprintln!("region.end_addr: {:#010x}", region.end_addr());
		if alloc_end > region.end_addr() {
			return Err(());
		}
		let excess_size: usize = (region.end_addr() - alloc_end) as usize;
		crate::kprintln!("excess_size: {}", excess_size);
		if excess_size > 0 && (excess_size) < core::mem::size_of::<ListNode>() {
			return Err(());
		}
		Ok(alloc_start)
	}

	/* find a region and remove it from the linked list */
	fn find_region(&mut self, size: usize, align: usize)
		-> Option<(&'static mut ListNode, VirtAddr)> {
		let mut current = &mut self.head;

		crate::kprintln!("current.start_addr: {:#010x}", current.start_addr());
		while let Some(ref mut region) = current.next {
			crate::kprintln!("boop");
			if let Ok(alloc_start) = Self::alloc_from_region(&region, size, align) {
				let next = region.next.take();
				let ret = Some((current.next.take().unwrap(), alloc_start));
				current.next = next;
				return ret;
			} else {
				crate::kprintln!("what");
				current = current.next.as_mut().unwrap();
			}
		}
		None
	}

	/* add a free region to the linked list */
	unsafe fn add_free_region(&mut self, addr: VirtAddr, size: usize) {
		assert_eq!(align_up(addr, core::mem::align_of::<ListNode>()), addr);
		assert!(size >= core::mem::size_of::<ListNode>());

		crate::kprintln!("SIZE: {}", size);
		let mut node: ListNode = ListNode::new(size);
		crate::kprintln!("start_addr: {:#010x}, end_addr: {:#010x}", node.start_addr(), node.end_addr());
		node.next = self.head.next.take();
		let node_ptr: *mut ListNode = addr as *mut ListNode;
		node_ptr.write(node);
		self.head.next = Some(&mut *node_ptr);
		crate::kprintln!("next: {:p}", &self.head.next);
	}
}
