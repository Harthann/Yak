use crate::{
	kprintln,
	kprint
};


#[test_case]
fn trivial_assertion() {
	assert_eq!(1, 1);
}

#[test_case]
fn trivial_fail() {
	assert_eq!(1, 1);
}
