use crate::kprintln;
use crate::vec::Vec;


#[test_case]
fn test_basics() {
	let mut x: Vec<u32> = Vec::new();
	let mut y: Vec<u32> = Vec::with_capacity(10);

	assert_eq!(x.capacity(), 0);
	assert_eq!(y.capacity(), 10);

	y.push(5);
	y.push(10);
	y.push(15);
	assert_eq!(y.len(), 3);
	x.push(10);
	assert!(x.capacity() > 0);
}

#[test_case]
fn test_reserve() {
	let mut x: Vec<u32> = Vec::new();

	assert_eq!(x.capacity(), 0);
	x.reserve(10);
	assert_eq!(x.capacity(), 10);
}

/* Simply test if conversion is working */
#[test_case]
fn test_slices() {
	let mut x: Vec<u32> = Vec::with_capacity(10);

	x.push(5);
	x.push(10);
	x.push(15);
	assert_eq!(x.as_slice(), [5, 10, 15]);
}
