use crate::print_fn;
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

#[test_case]
fn string_push_pop() {
	print_fn!();

	let mut x = String::new();

	assert_eq!(x.capacity(), 0);
	assert_eq!(x.len(), 0);
	x.push('a');
	assert!(x.capacity() > 0);
	assert_eq!(x.len(), 1);

	x.push('b');
	x.push('c');
	x.push('d');
	assert_eq!(&x[..], "abcd");
	assert_eq!(x.pop(), Some('d'));
	assert_eq!(x.pop(), Some('c'));
	assert_eq!(x.pop(), Some('b'));
	assert_eq!(x.pop(), Some('a'));
	assert_eq!(x.pop(), None);
}

#[test_case]
fn string_clear() {
	print_fn!();

	let mut x = String::from("abcdef");

	assert_eq!(x.len(), 6);
	assert!(x.capacity() > 0);

	x.clear();
	assert_eq!(x.len(), 0);
	assert_eq!(&x[..], "");
}

#[test_case]
fn string_deref() {
	print_fn!();

	let x = String::from("Ceci est un test");
	let mut iter = x.split(' ');

	assert_eq!(iter.next(), Some("Ceci"));
	assert_eq!(iter.next(), Some("est"));
	assert_eq!(iter.next(), Some("un"));
	assert_eq!(iter.next(), Some("test"));
	assert_eq!(iter.next(), None);
}

#[test_case]
fn string_push_str() {
	print_fn!();

	let mut x = String::from("Hello");

	assert_eq!(&x[..], "Hello");
	assert_eq!(x.len(), 5);

	x.push_str(" world!");
	assert_eq!(&x[..], "Hello world!");
	assert_eq!(x.len(), 12);
}

#[test_case]
fn string_insert() {
	print_fn!();

	let mut x = String::from("Hello");

	x.insert(3, 'k');
	assert_eq!(&x[..], "Helklo");
	x.insert(0, 'z');
	x.insert(6, '{');
	x.insert(x.len(), '6');
	assert_eq!(&x[..], "zHelkl{o6");
	x.insert_str(4, " World! ");
	assert_eq!(&x[..], "zHel World! kl{o6");
}

#[test_case]
fn string_remove() {
	print_fn!();

	let mut x = String::from("Hello World!");

	x.remove(0);
	assert_eq!(&x[..], "ello World!");
	x.remove(4);
	assert_eq!(&x[..], "elloWorld!");
	x.remove(2);
	assert_eq!(&x[..], "eloWorld!");
	x.remove(x.len());
	assert_eq!(&x[..], "eloWorld");
}

use crate::string::ToString;
#[test_case]
fn to_string() {
	print_fn!();

	let x = String::from("Hello World");
	let y: u8 = 42;

	assert_eq!(x, "Hello World".to_string());
	assert_eq!(y.to_string(), "42");
}
