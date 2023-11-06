use core::marker::Copy;
use core::ops::{Add, Rem, Sub};

pub fn roundup<
	T: Rem
		+ Sub<<T as Rem>::Output>
		+ Add<
			<<T as Sub<<T as Rem>::Output>>::Output as Rem<T>>::Output,
			Output = T
		> + Copy
>(
	number: T,
	multiple: T
) -> T
where
	<T as Sub<<T as Rem>::Output>>::Output: Rem<T>
{
	number + (multiple - (number % multiple)) % multiple
}
