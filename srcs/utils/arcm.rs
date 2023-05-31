use crate::spin::Mutex;
use alloc::sync::Arc;
use core::ops::{Deref, DerefMut};

/// Wrap the given type into an Arc and a Mutex.
/// Arc allow multiple reference on the same data between threads
/// Mutex allow any type to be `Send` and ensure safe access to the underlying data
#[derive(Default)]
pub struct Arcm<T: ?Sized> {
	arc: Arc<Mutex<T>>
}

impl<T> Clone for Arcm<T> {
	fn clone(&self) -> Self {
		Self { arc: self.arc.clone() }
	}
}
impl<T> Arcm<T> {
	/// Create a new Arcm by copying the data
	///
	/// # Examples
	/// ```
	/// let arcm: Arcm<u32> = Arcm::new(5);
	/// ```
	pub fn new(data: T) -> Self {
		Self { arc: Arc::new(Mutex::new(data)) }
	}

	/// Clone the current Arc and send it to the function pass in paramters
	///
	/// # Examples
	/// ```
	/// let arcm: Arcm<u32> = Arcm::new(5);
	/// arcm.execute(|cloned| {
	///     *cloned.lock() = 10;
	/// });
	/// assert_eq!(*arcm, 10);
	/// ```
	pub fn execute<R>(
		&self,
		mut callback: impl FnMut(Arc<Mutex<T>>) -> R
	) -> R {
		callback(self.arc.clone())
	}
}

impl<T: ?Sized> Deref for Arcm<T> {
	type Target = Arc<Mutex<T>>;
	fn deref(&self) -> &Self::Target {
		&self.arc
	}
}

impl<T: ?Sized> DerefMut for Arcm<T> {
	fn deref_mut(&mut self) -> &mut Arc<Mutex<T>> {
		&mut self.arc
	}
}

/// Allow type coercion from Arcm\<T\> to Arcm\<U\> where U is Unsized and coercion exist from T to U
/// This is used to store `dyn Trait` from a struct that implement the trait for example
///
/// # Examples
/// ```
/// trait Foo {
///     fn bar(&self) -> u32;
/// }
/// struct Foobar {}
/// impl Foo for Foobar {
///     fn bar(&self) -> u32 { 42 }
/// }
/// fn exmaple() {
///     let foo: Foobar = Foobar{};
///     let arcm: Arcm<dyn Foo> = Arcm::new(foo);
///     assert_eq!(arcm.lock().bar(), 42);
/// }
/// ```
impl<T, U> core::ops::CoerceUnsized<Arcm<U>> for Arcm<T>
where
	T: core::marker::Unsize<U> + ?Sized,
	U: ?Sized
{
}

#[cfg(test)]
mod test {
	use super::Arcm;
	use crate::boxed::Box;

	#[sys_macros::test_case]
	fn test_arcm_closure() {
		let arcm: Arcm<u32> = Arcm::new(5);
		arcm.execute(|cloned| {
			let mut guard = cloned.lock();
			*guard = 6;
		});
		assert_eq!(*arcm.lock(), 6);
	}

	#[sys_macros::test_case]
	fn test_arcm_with_box() {
		let arcm: Arcm<Box<u32>> = Arcm::new(Box::new(5));
		arcm.execute(|cloned| {
			let mut guard = cloned.lock();
			**guard = 6;
		});
		assert_eq!(**arcm.lock(), 6);
	}

	trait Foo {
		fn bar(&self) -> u32;
	}
	struct Foobar {}
	impl Foo for Foobar {
		fn bar(&self) -> u32 {
			42
		}
	}

	#[sys_macros::test_case]
	fn test_arcm_unsized_coercion() {
		let foo: Foobar = Foobar {};
		let arcm: Arcm<dyn Foo> = Arcm::new(foo);
		assert_eq!(arcm.lock().bar(), 42);
	}
}
