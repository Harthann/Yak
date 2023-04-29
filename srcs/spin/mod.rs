use core::cell::UnsafeCell;
use core::fmt;
use core::ops::{Deref, DerefMut, Drop};
use core::sync::atomic::{spin_loop_hint, AtomicBool, Ordering};

pub type Mutex<T> = RawMutex<T, false>;
pub type KMutex<T> = RawMutex<T, true>;

/// Mutex structure to prevent data races
/// # Generic
///
/// `T` Inner type to store and protect
///
/// `INT` constant boolean to allow or not Mutex to enable/disable interrupts
#[derive(Default)]
pub struct RawMutex<T: ?Sized, const INT: bool> {
	lock: AtomicBool,
	data: UnsafeCell<T>
}

/// MutexGuard that provide data mutable access
///
/// When the guard falls out of scope, lock is released.
#[derive(Debug)]
pub struct MutexGuard<'a, T: ?Sized + 'a, const INT: bool> {
	lock: &'a AtomicBool,
	data: &'a mut T
}

impl<T, const INT: bool> RawMutex<T, INT> {
	/// Create a new mutex with the given data stored inside
	pub const fn new(data: T) -> Self {
		Self { lock: AtomicBool::new(false), data: UnsafeCell::new(data) }
	}
}
unsafe impl<T: ?Sized + Send, const INT: bool> Sync for RawMutex<T, INT> {}
unsafe impl<T: ?Sized + Send, const INT: bool> Send for RawMutex<T, INT> {}

impl<T: ?Sized, const INT: bool> RawMutex<T, INT> {
	/// Loop until the inner lock as the value false then write true on it.
	/// Once the value as been written the mutex is successfully locked.
	/// If `const INT` as been set to `true`, interrupt flag is clear
	fn obtain_lock(&self) {
		while self.lock.compare_and_swap(false, true, Ordering::Acquire)
			!= false
		{
			while self.lock.load(Ordering::Relaxed) != false {
				spin_loop_hint();
			}
		}
		if INT == true {
			crate::wrappers::_cli();
		}
	}

	/// Lock the mutex if available otherwise wait until a lock is successfull
	/// If feature `mutex_debug` is enable, self if written to the debug output
	///
	/// The returned value can be dereference to access the data, once the guard falls
	/// out of scope, mutex will be unlocked
	pub fn lock(&self) -> MutexGuard<T, INT> {
		#[cfg(feature = "mutex_debug")]
		unsafe {
			crate::dprintln!("{:?}", self)
		};
		self.obtain_lock();
		MutexGuard { lock: &self.lock, data: unsafe { &mut *self.data.get() } }
	}

	/// Try to lock the mutex. Returning a Guard if successfull
	pub fn try_lock(&self) -> Option<MutexGuard<T, INT>> {
		if self.lock.compare_and_swap(false, true, Ordering::Acquire) == false {
			if INT == true {
				crate::wrappers::_cli();
			}
			Some(MutexGuard {
				lock: &self.lock,
				data: unsafe { &mut *self.data.get() }
			})
		} else {
			None
		}
	}
}

// Note this will probably cause deadlock since write need to lock a mutex
impl<T: ?Sized, const INT: bool> fmt::Debug for RawMutex<T, INT> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self.try_lock() {
			Some(_guard) => write!(f, "Mutex ({:#p}) {{ <Not locked> }}", self),
			None => write!(f, "Mutex ({:#p}) {{ <locked> }}", self)
		}
	}
}

impl<'a, T: ?Sized, const INT: bool> Deref for MutexGuard<'a, T, INT> {
	type Target = T;
	fn deref<'b>(&'b self) -> &'b T {
		&*self.data
	}
}

impl<'a, T: ?Sized, const INT: bool> DerefMut for MutexGuard<'a, T, INT> {
	fn deref_mut<'b>(&'b mut self) -> &'b mut T {
		&mut *self.data
	}
}

impl<'a, T: ?Sized, const INT: bool> Drop for MutexGuard<'a, T, INT> {
	fn drop(&mut self) {
		self.lock.store(false, Ordering::Release);
		if INT == true {
			crate::wrappers::_sti();
		}
	}
}
