use crate::boxed::Box;
use crate::fs;
use crate::fs::FileError;

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
	fn read(&self, dst: &mut [u8], length: usize) -> Result<usize, FileError> {
		for i in 0..length {
			if i >= self.buffer.len() || i >= dst.len() {
				return Ok(i);
			}
			dst[i] = self.buffer[i];
		}
		Ok(length)
	}

	fn write(&mut self, src: &[u8], length: usize) -> Result<usize, FileError> {
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