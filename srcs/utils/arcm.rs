use crate::spin::RawMutex;
use alloc::sync::Arc;
use core::ops::{Deref, DerefMut};

pub type Arcm<T> = RawArcm<T, false>;
pub type KArcm<T> = RawArcm<T, true>;

/// Wrap the given type into an Arc and a Mutex.
/// Arc allow multiple reference on the same data between threads
/// Mutex allow any type to be `Send` and ensure safe access to the underlying data
#[derive(Default)]
pub struct RawArcm<T: ?Sized, const INT: bool> {
	arc: Arc<RawMutex<T, INT>>
}

impl<T, const INT: bool> Clone for RawArcm<T, INT> {
	fn clone(&self) -> Self {
		Self { arc: self.arc.clone() }
	}
}
impl<T, const INT: bool> RawArcm<T, INT> {
	/// Create a new RawArcm by copying the data
	///
	/// # Examples
	/// ```
	/// let arcm: RawArcm<u32, false> = RawArcm::new(5);
	/// ```
	pub fn new(data: T) -> Self {
		Self { arc: Arc::new(RawMutex::new(data)) }
	}

	/// Clone the current Arc and send it to the function pass in paramters
	///
	/// # Examples
	/// ```
	/// let arcm: RawArcm<u32, false> = RawArcm::new(5);
	/// arcm.execute(|cloned| {
	///     *cloned.lock() = 10;
	/// });
	/// assert_eq!(*arcm, 10);
	/// ```
	pub fn execute<R>(
		&self,
		mut callback: impl FnMut(Arc<RawMutex<T, INT>>) -> R
	) -> R {
		callback(self.arc.clone())
	}
}

impl<T: ?Sized, const INT: bool> Deref for RawArcm<T, INT> {
	type Target = Arc<RawMutex<T, INT>>;
	fn deref(&self) -> &Self::Target {
		&self.arc
	}
}

impl<T: ?Sized, const INT: bool> DerefMut for RawArcm<T, INT> {
	fn deref_mut(&mut self) -> &mut Arc<RawMutex<T, INT>> {
		&mut self.arc
	}
}

/// Allow type coercion from RawArcm\<T\> to RawArcm\<U\> where U is Unsized and coercion exist from T to U
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
///     let arcm: RawArcm<dyn Foo, false> = RawArcm::new(foo);
///     assert_eq!(arcm.lock().bar(), 42);
/// }
/// ```
impl<T, U, const INT: bool> core::ops::CoerceUnsized<RawArcm<U, INT>>
	for RawArcm<T, INT>
where
	T: core::marker::Unsize<U> + ?Sized,
	U: ?Sized
{
}

#[cfg(test)]
mod test {
	use super::Arcm;
	use crate::alloc::boxed::Box;

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
