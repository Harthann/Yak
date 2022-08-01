use crate::kprintln;
use crate::vec::Vec;


#[test_case]
fn vector_basics() {
	print_fn!();
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
fn vector_reserve() {
	print_fn!();
	let mut x: Vec<u32> = Vec::new();

	assert_eq!(x.capacity(), 0);
	x.reserve(10);
	assert_eq!(x.capacity(), 10);
}

#[test_case]
fn vector_free() {
	print_fn!();
	let x: Vec<u32> = Vec::with_capacity(1000);
	let ptr: u32;

	{
		let y: Vec<u32> = Vec::with_capacity(1000);
		ptr = y.as_ptr() as u32;
		assert!(x.as_ptr() != y.as_ptr());
	}
	let z: Vec<u32> = Vec::with_capacity(1000);
	assert_eq!(ptr, z.as_ptr() as u32);
}

#[test_case]
fn vector_big_alloc() {
	use crate::vec::{Global, AllocError};

	print_fn!();

/* Should send an error */
	{
		let x = Vec::<u32, Global>::try_alloc(200000, &Global);
		assert_eq!(x, Err(AllocError));
	}

/* Big chunk */
	{
		let size = 100000;
		let mut x: Vec<u32> = Vec::with_capacity(size);

		for i in 0..size {
			x.push(i as u32);
		}
		assert_eq!(x.capacity(), size);
	}
}

/* Simply test if conversion is working */
#[test_case]
fn vector_slices() {
	print_fn!();
	let mut x: Vec<u32> = Vec::with_capacity(10);

	x.push(5);
	x.push(10);
	x.push(15);
	assert_eq!(x.as_slice(), [5, 10, 15]);
}

#[test_case]
fn vector_deref() {
	print_fn!();

	let mut x: Vec<u32> = Vec::new();

	x.push(1);
	x.push(2);
	x.push(3);
	x.push(4);

	assert_eq!(x[..], [1, 2, 3, 4]);
	x.reverse();
	assert_eq!(x[..], [4, 3, 2, 1]);
}


#[test_case]
fn vector_insertion() {
	print_fn!();

	let mut x: Vec<u32> = Vec::new();

	for i in 0..5 {
		x.insert(i, i as u32);
	}
	assert_eq!(x[..], [0, 1, 2, 3, 4]);
	x.insert(3, 10);
	x.insert(3, 10);
	x.insert(3, 10);
	assert_eq!(x[..], [0, 1, 2, 10, 10, 10, 3, 4]);
	x.insert(x.len(), 15);
	assert_eq!(x[..], [0, 1, 2, 10, 10, 10, 3, 4, 15]);
}

#[test_case]
fn vector_extend_from_slice() {
	print_fn!();

	let mut x: Vec<u32> = Vec::new();
	x.extend_from_slice(&[0, 1, 2, 3, 4, 5, 6, 7]);
	assert_eq!(x[..], [0, 1, 2, 3, 4, 5, 6, 7]);
	x.extend_from_slice(&[0, 1, 2, 3, 4]);
	assert_eq!(x[..], [0, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4]);
}

use crate::vec;

/*
** vec![x..y] use specific function called "into_vec"
** vec!() simply calls Vec::new()
** vec![x; y] use specific function called from_elem
** Assuming function related to macros works if macros works
*/
#[test_case]
fn vector_macros() {
	print_fn!();

	let x: Vec<u32> = vec![0,1,2,3,4,5];
	let y: Vec<u32> = vec!();//[0,1,2,3,4,5];
	let z: Vec<u32> = vec![5; 10];//[0,1,2,3,4,5];

	assert_eq!(x[..], [0,1,2,3,4,5]);

	assert_eq!(y[..], []);
	assert_eq!(y.capacity(), 0);
	assert_eq!(y.len(), 0);

	assert_eq!(z[..], [5; 10]);
	assert_eq!(z.capacity(), 10);
	assert_eq!(z.len(), 10);
}

#[test_case]
fn vector_remove() {
	print_fn!();

	let mut x: Vec<u32> = vec![0,1,2,3,4,5];
	let base_len = x.len();

	assert_eq!(x.remove(1), Some(1));
	assert_eq!(x.len(), base_len - 1);
	assert_eq!(x.remove(3), Some(4));
	assert_eq!(x.len(), base_len - 2);
	assert_eq!(x.remove(50), None);

	assert_eq!(x.empty(), false);
	x.clear();
	assert_eq!(x.empty(), true);
}
