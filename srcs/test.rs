use crate::{
	kprintln,
	kprint
};


#[test_case]
fn trivial_assertion() {
	assert_eq!(1, 1);
}

#[test_case]
#[should_panic]
fn trivial_fail() {
	assert_eq!(2, 1);
}
