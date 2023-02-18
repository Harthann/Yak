use core::cell::UnsafeCell;
use core::fmt;
use core::ops::{Drop, Deref, DerefMut};
use core::sync::atomic::{
AtomicBool,
spin_loop_hint,
Ordering
};


pub struct Mutex<T: ?Sized, const INT: bool> {
	lock: AtomicBool,
	data: UnsafeCell<T>
}

#[derive(Debug)]
pub struct MutexGuard<'a, T: ?Sized +  'a, const INT: bool> {
	lock: &'a AtomicBool,
	data: &'a mut T
}

impl<T, const INT: bool> Mutex<T, INT> {
	pub const fn new(data: T) -> Mutex<T, INT> {
		Mutex {
			lock: AtomicBool::new(false),
			data: UnsafeCell::new(data)
		}
	}
}

impl<T: ?Sized, const INT: bool> Mutex<T, INT> {

	fn obtain_lock(&mut self) {
		while self.lock.compare_and_swap(false, true, Ordering::Acquire) != false {
			while self.lock.load(Ordering::Relaxed) != false {
				spin_loop_hint();
			}
		}
        if INT == true { crate::wrappers::_cli(); }
	}

	pub fn lock(&mut self) -> MutexGuard<T, INT> {
		self.obtain_lock();
		MutexGuard {
			lock: &self.lock,
			data: unsafe{ &mut *self.data.get() }
		}
	}

	pub fn try_lock(&self) -> Option<MutexGuard<T, INT>>
    {
        if self.lock.compare_and_swap(false, true, Ordering::Acquire) == false
        {
            if INT == true { crate::wrappers::_cli(); }
            Some(
                MutexGuard {
                    lock: &self.lock,
                    data: unsafe { &mut *self.data.get() },
                }
            )
        }
        else
        {
            None
        }
    }
}

/* Note this will probably cause deadlock since write need to lock a mutex */
impl<T: ?Sized + fmt::Debug, const INT: bool> fmt::Debug for Mutex<T, INT>
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        match self.try_lock()
        {
            Some(guard) => write!(f, "Mutex {{ data: ")
				.and_then(|()| (&*guard).fmt(f))
				.and_then(|()| write!(f, "}}")),
            None => write!(f, "Mutex {{ <locked> }}"),
        }
    }
}

impl<'a, T: ?Sized, const INT:bool> Deref for MutexGuard<'a, T, INT> {
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
        if INT == true { crate::wrappers::_sti(); }
	}
}
