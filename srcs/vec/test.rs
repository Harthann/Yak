use crate::kprintln;
use crate::vec::Vec;


#[test_case]
fn test_creation() {
	let x: Vec<u32> = Vec::new();
	let y: Vec<u32> = Vec::with_capacity(10);

	assert_eq!(x.capacity(), 0);
	assert_eq!(y.capacity(), 10);
}
