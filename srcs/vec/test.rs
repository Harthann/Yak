use crate::kprintln;
use crate::vec::Vec;


#[test_case]
fn test_basics() {
	let x: Vec<u32> = Vec::new();
	let mut y: Vec<u32> = Vec::with_capacity(10);

	assert_eq!(x.capacity(), 0);
	assert_eq!(y.capacity(), 10);

	y.push(5);
	y.push(10);
	y.push(15);
	kprintln!("Debug test: {}", y);
	kprintln!("Poped: {:?}", y.pop());
	kprintln!("Debug test: {}", y);
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
