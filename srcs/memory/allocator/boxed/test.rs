use crate::memory::allocator::boxed::Box;

#[test_case]
fn basic_allocation() {
	let x = Box::new(5);
	assert_eq!(*x, 5);
	let y = Box::new(10);
	assert_eq!(*y, 10);
	let z = Box::try_new(10);
	assert_eq!(z.is_ok(), true);
}

#[test_case]
fn free_test() {
	let ptr: u32;
	{
		let x = Box::new(5);
		ptr = (x.as_ref() as *const _) as u32;
	}
	let x = Box::new(5);
	assert_eq!(ptr, (x.as_ref() as *const _) as u32);
}

#[test_case]
fn test_mut_ref() {
	let mut x = Box::new(5);
	assert_eq!(*x, 5);
	let y = x.as_mut();
	*y = 10;
	assert_eq!(*x, 10);
}

#[test_case]
fn test_array() {
	let x = Box::new([5; 10]);
	assert_eq!(*x, [5; 10]);
}

#[test_case]
fn test_write() {
	let mut x = Box::new(5);
	assert_eq!(*x, 5);
	x = Box::write(x, 10);
	assert_eq!(*x, 10);
}
