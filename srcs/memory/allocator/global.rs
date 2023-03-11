use crate::memory::allocator::{AllocError, Allocator};
use crate::ALLOCATOR;
use core::alloc::{GlobalAlloc, Layout};
use core::ptr::NonNull;

// Struct to wrap allocation
#[derive(Copy, Clone, Debug)]
pub struct Global;

// The three next function use VALLOCATOR to perform
// Virtual allocation
// Virtual deallocation
// Virtual reallocation
#[inline]
unsafe fn alloc(layout: Layout) -> *mut u8 {
	let ptr = ALLOCATOR.alloc(layout);
	if !ptr.is_null() {
		crate::TRACKER.allocation += 1;
		crate::TRACKER.allocated_bytes += layout.size();
	}
	ptr
}

#[inline]
unsafe fn alloc_zeroed(layout: Layout) -> *mut u8 {
	let ptr = ALLOCATOR.alloc(layout);
	core::ptr::write_bytes(ptr, 0x0, layout.size());
	ptr
}

#[inline]
unsafe fn dealloc(ptr: *mut u8, layout: Layout) {
	ALLOCATOR.dealloc(ptr, layout);
	crate::TRACKER.freed += 1;
	crate::TRACKER.freed_bytes += layout.size();
}

#[inline]
unsafe fn realloc(ptr: *mut u8, old: Layout, new_size: usize) -> *mut u8 {
	let new_ptr: *mut u8 =
		alloc(Layout::from_size_align(new_size, old.align()).unwrap());
	if new_ptr.is_null() {
		return core::ptr::null_mut();
	}

	core::ptr::copy_nonoverlapping(ptr, new_ptr, old.size());

	dealloc(ptr, old);
	new_ptr
}

impl Global {
	#[inline]
	pub fn alloc_impl(
		&self,
		layout: Layout,
		zeroed: bool
	) -> Result<NonNull<u8>, AllocError> {
		match layout.size() {
			0 => {
				return Ok(NonNull::dangling());
			},
			_size => {
				let raw_ptr = unsafe {
					if zeroed {
						alloc_zeroed(layout)
					} else {
						alloc(layout)
					}
				};
				let ptr = NonNull::new(raw_ptr).ok_or(AllocError {})?;
				Ok(ptr)
			}
		}
	}

	#[inline]
	pub fn dealloc_impl(&self, ptr: *mut u8, layout: Layout) {
		unsafe { dealloc(ptr, layout) };
	}
}

impl Allocator for Global {
	#[inline]
	fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, AllocError> {
		self.alloc_impl(layout, false)
	}

	#[inline]
	fn allocate_zeroed(
		&self,
		layout: Layout
	) -> Result<NonNull<u8>, AllocError> {
		self.alloc_impl(layout, true)
	}

	#[inline]
	fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
		self.dealloc_impl(ptr.as_ptr(), layout);
	}

	#[inline]
	fn realloc(
		&self,
		ptr: NonNull<u8>,
		oldlayout: Layout,
		new_size: usize
	) -> Result<NonNull<u8>, AllocError> {
		if oldlayout.size() >= new_size {
			Ok(ptr)
		} else {
			let raw_ptr = unsafe { realloc(ptr.as_ptr(), oldlayout, new_size) };
			NonNull::new(raw_ptr).ok_or(AllocError {})
		}
	}
}
