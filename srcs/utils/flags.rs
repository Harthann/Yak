use core::fmt;
use core::sync::atomic::{AtomicU8, Ordering};

#[derive(Default)]
pub struct Flags<T>(pub T);

/// To use this trait the type T has to allow interior mutability
/// This is ok in the example since AtomicU8 is thread safe and allow interior mutability
pub trait FlagOp<T> {
	/// Toggle bit.
	/// Safety: No check performed, will produce panic if trying to toggle bit outside of range
	/// e.g:
	///
	/// let flag = Flags::<AtomicU8>::default();
	/// flag.toggle(1); // Toggle the first bit of the inner AtomicU8
	/// flag.toggle(9); // Panic there is no 9th bit in an u8
	fn toggle(&self, bit: u8);

	/// enable x bit.
	/// Safety: No check performed, will produce panic if trying to enable bit outside of range
	/// e.g:
	///
	/// let flag = Flags::<AtomicU8>::default();
	/// flag.disable(1); // Enable the first bit of the inner AtomicU8
	/// flag.disable(9); // Panic there is no 9th bit in an u8
	fn enable(&self, bit: u8);

	/// disable x bit.
	/// Safety: No check performed, will produce panic if trying to disable bit outside of range
	/// e.g:
	///
	/// let flag = Flags::<AtomicU8>::default();
	/// flag.disable(1); // Disable the first bit of the inner AtomicU8
	/// flag.disable(9); // Panic there is no 9th bit in an u8
	fn disable(&self, bit: u8);

	/// check if the x bit is set.
	/// safety: no check performed, will produce panic if trying to check bit outside of range
	/// e.g:
	///
	/// let flag = flags::<atomicu8>::default();
	/// flag.is_enable(1); // return true/false depending of the firt bit state
	/// flag.is_enable(9); // panic there is no 9th bit in an u8
	fn is_enable(&self, bit: u8) -> bool;

	/// check if the x bit is unset.
	/// safety: no check performed, will produce panic if trying to check bit outside of range
	/// e.g:
	///
	/// let flag = flags::<atomicu8>::default();
	/// flag.is_disable(1); // return true/false depending of the firt bit state
	/// flag.is_disable(9); // panic there is no 9th bit in an u8
	fn is_disable(&self, bit: u8) -> bool;
}

impl FlagOp<AtomicU8> for Flags<AtomicU8> {
	fn toggle(&self, bit: u8) {
		self.0.fetch_xor(1 << bit, Ordering::Relaxed);
	}

	fn enable(&self, bit: u8) {
		self.0.fetch_or(1 << bit, Ordering::Relaxed);
	}

	fn disable(&self, bit: u8) {
		self.0.fetch_and(!(1 << bit), Ordering::Relaxed);
	}

	fn is_enable(&self, bit: u8) -> bool {
		(self.0.load(Ordering::Relaxed) & (1 << bit)) == (1 << bit)
	}

	fn is_disable(&self, bit: u8) -> bool {
		!self.is_enable(bit)
	}
}

impl Flags<AtomicU8> {
	pub const fn new(base: u8) -> Self {
		Self(AtomicU8::new(base))
	}
}

impl fmt::Display for Flags<AtomicU8> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:#010b}", self.0.load(Ordering::Relaxed))
	}
}

#[cfg(test)]
mod tests {
	use super::{FlagOp, Flags};
	use core::sync::atomic::{AtomicU8, Ordering};

	#[sys_macros::test]
	fn test_flag_toggle() {
		let mut flags = Flags::<AtomicU8>::default();
		assert_eq!(flags.0.load(Ordering::Relaxed), 0);
		for i in 0..8 {
			assert_eq!(flags.0.load(Ordering::Relaxed) & (1 << i), 0);
			flags.toggle(i);
			assert_eq!(flags.0.load(Ordering::Relaxed) & (1 << i), (1 << i));
			flags.toggle(i);
			assert_eq!(flags.0.load(Ordering::Relaxed) & (1 << i), 0);
		}
	}

	#[sys_macros::test]
	fn test_flag_enable() {
		let mut flags = Flags::<AtomicU8>::default();
		assert_eq!(flags.0.load(Ordering::Relaxed), 0);
		for i in 0..8 {
			assert_eq!(flags.0.load(Ordering::Relaxed) & (1 << i), 0);
			flags.enable(i);
			assert_eq!(flags.0.load(Ordering::Relaxed) & (1 << i), (1 << i));
			flags.enable(i);
			assert_eq!(flags.0.load(Ordering::Relaxed) & (1 << i), (1 << i));
		}
	}

	#[sys_macros::test]
	fn test_flag_disable() {
		let mut flags = Flags::<AtomicU8>::new(0b11111111);
		assert_eq!(flags.0.load(Ordering::Relaxed), 0xff);
		for i in 0..8 {
			assert_eq!(flags.0.load(Ordering::Relaxed) & (1 << i), (1 << i));
			flags.disable(i);
			assert_eq!(flags.0.load(Ordering::Relaxed) & (1 << i), 0);
			flags.disable(i);
			assert_eq!(flags.0.load(Ordering::Relaxed) & (1 << i), 0);
		}
	}

	#[sys_macros::test]
	fn test_flag_is() {
		let mut flags = Flags::<AtomicU8>::default();
		assert_eq!(flags.0.load(Ordering::Relaxed), 0);
		for i in 0..8 {
			assert_eq!(flags.is_enable(i), false);
			assert_eq!(flags.is_disable(i), true);
			flags.enable(i);
			assert_eq!(flags.is_enable(i), true);
			assert_eq!(flags.is_disable(i), false);
			flags.disable(i);
			assert_eq!(flags.is_enable(i), false);
			assert_eq!(flags.is_disable(i), true);
		}
	}
}
