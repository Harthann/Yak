pub mod linked_list;
pub mod bump;
pub mod boxed;

use core::alloc::{
GlobalAlloc,
Layout
};
use core::ptr::NonNull;
use crate::ALLOCATOR;

use crate::memory::VirtAddr;

/*	Trait definitions to inialize a global allocator */
pub trait AllocatorInit: GlobalAlloc {
	unsafe fn init(&mut self, offset: VirtAddr, size: usize);
}

/* Custom trait to define Allocator design */
pub trait Allocator {
	fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, AllocError>;
	fn kallocate(&self, layout: Layout) -> Result<NonNull<u8>, AllocError>;
	fn deallocate(&self, ptr: NonNull<u8>, layout: Layout);
	fn kdeallocate(&self, ptr: NonNull<u8>, layout: Layout);
	fn allocate_zeroed(&self, layout: Layout) -> Result<NonNull<u8>, AllocError>;
	fn kallocate_zeroed(&self, layout: Layout) -> Result<NonNull<u8>, AllocError>;
}

/* Struct to wrap allocation */
#[derive(Copy, Clone, Debug)]
pub struct Global;
#[derive(Copy, Clone, Debug)]
/* Allocation errordefinition */
pub struct AllocError;



/* The three next function use VALLOCATOR to perform
** Virtual allocation
** Virtual deallocation
** Virtual reallocation
*/
#[inline]
pub unsafe fn valloc(layout: Layout) -> *mut u8 {
	ALLOCATOR.alloc(layout)
}

#[inline]
pub unsafe fn valloc_zeroed(layout: Layout) -> *mut u8 {
	let ptr = ALLOCATOR.alloc(layout);
	core::ptr::write_bytes(ptr, 0x0, layout.size());
	ptr
}

#[inline]
pub unsafe fn vdealloc(ptr: *mut u8, layout: Layout) {
	ALLOCATOR.dealloc(ptr, layout);
}

#[inline]
pub unsafe fn vrealloc(ptr: *mut u8, old: Layout, new_size: usize) -> *mut u8 {
	let new_ptr: *mut u8 = valloc(Layout::from_size_align(new_size, old.align()).unwrap());
	if new_ptr.is_null() { return core::ptr::null_mut(); }

	core::ptr::copy_nonoverlapping(ptr, new_ptr, old.size());

	vdealloc(ptr, old);
	new_ptr
}

/* The three next function use KALLOCATOR to perform
** Physical allocation
** Physical deallocation
** Physical reallocation
*/
#[inline]
pub unsafe fn kalloc(layout: Layout) -> *mut u8 {
	ALLOCATOR.alloc(layout)
}

#[inline]
pub unsafe fn kalloc_zeroed(layout: Layout) -> *mut u8 {
	let ptr = ALLOCATOR.alloc(layout);
	core::ptr::write_bytes(ptr, 0x0, layout.size());
	ptr
}

#[inline]
pub unsafe fn kdealloc(ptr: *mut u8, layout: Layout) {
	ALLOCATOR.dealloc(ptr, layout);
}

#[inline]
pub unsafe fn krealloc(ptr: *mut u8, old: Layout, new_size: usize) -> *mut u8 {
	let new_ptr: *mut u8 = kalloc(Layout::from_size_align(new_size, old.align()).unwrap());
	if new_ptr.is_null() { return core::ptr::null_mut(); }

	core::ptr::copy_nonoverlapping(ptr, new_ptr, old.size());

	kdealloc(ptr, old);
	new_ptr
}

impl Global {
	/* Mem type determine if you use virtual or physical memory */
	#[inline]
	pub fn alloc_impl(&self, layout: Layout, zeroed: bool, phys_memory: bool) -> Result<NonNull<u8>, AllocError> {
		match layout.size() {
			0 => { return Ok(NonNull::dangling()); },
			_size => {
				let raw_ptr: *mut u8;
				if phys_memory == true {
					raw_ptr = unsafe{ if zeroed {kalloc_zeroed(layout)} else {kalloc(layout)} };
				} else {
					raw_ptr = unsafe{ if zeroed {valloc_zeroed(layout)} else {valloc(layout)} };
				}
				let ptr = NonNull::new(raw_ptr).ok_or(AllocError{})?;
				Ok(ptr)
			}
		}
	}

	#[inline]
	pub fn dealloc_impl(&self, ptr: *mut u8, layout: Layout, phys_memory: bool) {
		if phys_memory {
			unsafe{ kdealloc(ptr, layout) };
		} else {
			unsafe{ vdealloc(ptr, layout) };
		}
	}
}

impl Allocator for Global {
	#[inline]
	fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, AllocError> {
		self.alloc_impl(layout, false, false)
	}

	#[inline]
	fn allocate_zeroed(&self, layout: Layout) -> Result<NonNull<u8>, AllocError> {
		self.alloc_impl(layout, true, false)
	}

	#[inline]
	fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
		self.dealloc_impl(ptr.as_ptr(), layout, false);
	}

	#[inline]
	fn kallocate(&self, layout: Layout) -> Result<NonNull<u8>, AllocError> {
		self.alloc_impl(layout, false, true)
	}

	#[inline]
	fn kallocate_zeroed(&self, layout: Layout) -> Result<NonNull<u8>, AllocError> {
		self.alloc_impl(layout, true, true)
	}

	#[inline]
	fn kdeallocate(&self, ptr: NonNull<u8>, layout: Layout) {
		self.dealloc_impl(ptr.as_ptr(), layout, true);
	}



}

/*
	Align is a power of 2 so if we substract 1 his binary representation contain
	only 1 (0b1111..). We can then AND it with addr to get the right alignment.
	(add it with the addr to get the next alignment - align_up()) */
fn align_down(addr: VirtAddr, align: usize) -> VirtAddr {
	if align.is_power_of_two() {
		addr & !(align as u32 - 1)
	} else if align == 0 {
		addr
	} else {
		panic!("`align` must be a power of 2");
	}
}

fn align_up(addr: VirtAddr, align: usize) -> VirtAddr {
	(addr + align as u32 - 1) & !(align as u32 - 1)
}
