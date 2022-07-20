use crate::paging::VirtAddr;

struct ListNode {
	size: usize,
	next: Option<&'static mut ListNode>
}

impl ListNode {
	const fn new(size: usize) -> Self {
		Self {size, next: None}
	}

	fn get_vaddr(&self) -> VirtAddr {
		self as *const Self as VirtAddr
	}
}

pub struct LinkedListAllocator {
	head: ListNode
}

impl LinkedListAllocator {
	pub const fn new() -> Self {
		Self {head: ListNode::new(0)}
	}

	pub unsafe fn init(&mut self, heap_start: VirtAddr, heap_size: usize) {
		self.add_free_region(heap_start, heap_size);
	}

	unsafe fn add_free_region(&mut self, addr: VirtAddr, size: usize) {
		let mut node: ListNode = ListNode::new(size);
		node.next = self.head.next.take();
		let node_ptr: *mut ListNode = addr as *mut ListNode;
		node_ptr.write(node);
		self.head.next = Some(&mut *node_ptr);
	}

}
