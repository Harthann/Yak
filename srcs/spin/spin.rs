
pub struct Mutex<T> {
	locked: AtomicBool,
	data: T
};

impl<T> Mutext<T> {
	pub fn lock() -> MutexGuard<'_, T> {
	}
	
	pub fn unlock() {
	}
}

pub struct MutexGuard<'a, T: 'a> {
	mutex: &'a Mutex<T>
}

