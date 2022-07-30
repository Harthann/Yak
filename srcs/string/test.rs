use crate::{kprintln, print_fn};
use crate::string::String;

#[test_case]
fn string_basics() {
	print_fn!();

	let x = String::new();
	let y = String::from("abcdefgh");
	let z = String::from("42");

	assert_eq!(x.as_str(), "");
	assert_eq!(y.as_str(), "abcdefgh");
	assert_eq!(z.as_bytes(), [52, 50]);
}

#[test_case]
fn string_capacity() {
	print_fn!();

	let x = String::with_capacity(15);
	let mut y = String::new();

	assert!(x.capacity() >= 15);

	assert_eq!(y.capacity(), 0);
	y.reserve(15);
	assert!(y.capacity() >= 15);

}
