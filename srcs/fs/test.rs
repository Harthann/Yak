use crate::boxed::Box;
use crate::fs;
use crate::fs::ErrNo;

// Test buffer to implement FileOperation
struct Buffer {
	pub buffer: [u8; 1024]
}
impl Buffer {
	pub const fn new() -> Self {
		Self { buffer: [0; 1024] }
	}
}

// Actual FileOperation impl
impl fs::FileOperation for Buffer {
	fn read(&self, dst: &mut [u8], length: usize) -> Result<usize, ErrNo> {
		for i in 0..length {
			if i >= self.buffer.len() || i >= dst.len() {
				return Ok(i);
			}
			dst[i] = self.buffer[i];
		}
		Ok(length)
	}

	fn write(&mut self, src: &[u8], length: usize) -> Result<usize, ErrNo> {
		for i in 0..length {
			if i >= self.buffer.len() || i >= src.len() {
				return Ok(i);
			}
			self.buffer[i] = src[i];
		}
		Ok(length)
	}
}

#[sys_macros::test_case]
fn test_file() {
	let buffer: Buffer = Buffer::new();
	let buffer2: Buffer = Buffer::new();
	let _file = fs::create_from_raw("test_file", buffer)
		.expect("Failed to create file");
	let _file2 = fs::create_from_raw("test_file2", buffer2)
		.expect("Failed to create file");
	fs::delete("test_file");
	fs::delete("test_file2");
}

#[sys_macros::test_case]
fn test_file_op() {
	let buffer: Buffer = Buffer::new();
	fs::create_from_raw("test_file", buffer).expect("Failed to create file");
	let fd = fs::open("test_file").expect("Failed to open file");
	let src: &[u8] = b"hello world";
	let mut dst = Box::<[u8; 1024]>::new([0; 1024]);

	assert_ne!(src, &dst[0..src.len()]);
	assert_eq!(
		fs::write(fd, src, src.len()).expect("Writing failed"),
		src.len()
	);
	assert_eq!(
		fs::read(fd, &mut *dst, src.len()).expect("Reading failed"),
		src.len()
	);
	assert_eq!(src, &dst[0..src.len()]);
	fs::close(fd);
	fs::delete("test_file");
}

#[sys_macros::test_case]
fn test_file_op2() {
	let buffer: Buffer = Buffer::new();
	fs::create_from_raw("test_file", buffer).expect("Failed to create file");
	let fd = fs::open("test_file").expect("Failed to open file");
	let fd2 = fs::open("test_file").expect("Failed to open file");
	let src: &[u8] = b"hello world";
	let mut dst = Box::<[u8; 1024]>::new([0; 1024]);

	assert_ne!(src, &dst[0..src.len()]);
	assert_eq!(
		fs::write(fd, src, src.len()).expect("Writing failed"),
		src.len()
	);
	assert_eq!(
		fs::read(fd2, &mut *dst, src.len()).expect("Reading failed"),
		src.len()
	);
	assert_eq!(src, &dst[0..src.len()]);
	fs::close(fd);
	fs::close(fd2);
	fs::delete("test_file");
}

#[sys_macros::test_case]
fn test_file_thread() {
	let buffer: Buffer = Buffer::new();
	let src: &[u8] = b"hello thread";
	fs::create_from_raw("test_file", buffer).expect("Failed to create file");

	let pid = unsafe { crate::exec_fn!(threaded_file, src) };
	// wait thread to write on buffer
	let mut status = 0;
	use crate::syscalls::exit::sys_waitpid;
	sys_waitpid(pid, &mut status, 0);

	let fd = fs::open("test_file").expect("Failed to open file");
	let mut dst = Box::<[u8; 1024]>::new([0; 1024]);

	assert_ne!(src, &dst[0..src.len()]);
	assert_eq!(
		fs::read(fd, &mut *dst, src.len()).expect("Reading failed"),
		src.len()
	);
	assert_eq!(src, &dst[0..src.len()]);
	fs::close(fd);
	fs::delete("test_file");
}

fn threaded_file(src: &[u8]) {
	let fd = fs::open("test_file").expect("Failed to open file");
	assert_eq!(
		fs::write(fd, src, src.len()).expect("Writing failed"),
		src.len()
	);
	fs::close(fd);
}

#[sys_macros::test_case]
fn test_socket_pair() {
	use super::socket::{SocketDomain, SocketProtocol, SocketType};
	use super::socket_pair;
	let mut sockets: [usize; 2] = [0; 2];
	socket_pair(
		SocketDomain::AF_UNIX,
		SocketType::SOCK_DGRAM,
		SocketProtocol::DEFAULT,
		&mut sockets
	)
	.expect("Failed to create socket pair");
	assert_eq!(sockets[0], 0);
	assert_eq!(sockets[1], 1);

	let src: &[u8] = b"hello world";
	let mut dst = Box::<[u8; 1024]>::new([0; 1024]);

	assert_ne!(src, &dst[0..src.len()]);
	assert_eq!(
		fs::write(sockets[1], src, src.len())
			.expect("Writing to socket 1 failed"),
		src.len()
	);
	assert_eq!(
		fs::read(sockets[0], &mut *dst, src.len())
			.expect("Reading socket 0 failed"),
		src.len()
	);
	assert_eq!(src, &dst[0..src.len()]);

	// Test reading/writing the other way
	let src2 = "This is not a drill!";
	*dst = [0; 1024];
	assert_ne!(src, &dst[0..src2.len()]);
	assert_eq!(
		fs::write(sockets[0], src2.as_bytes(), src2.len())
			.expect("Writing to socket 0 failed"),
		src2.len()
	);
	assert_eq!(
		fs::read(sockets[1], &mut *dst, src2.len())
			.expect("Reading socket 1 failed"),
		src2.len()
	);
	assert_eq!(src2.as_bytes(), &dst[0..src2.len()]);

	super::close(sockets[0]);
	super::close(sockets[1]);
}
