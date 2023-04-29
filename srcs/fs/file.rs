use crate::boxed::Box;
use crate::spin::Mutex;
use crate::string::String;
use alloc::sync::Arc;

pub trait FileOperation {
	fn read(&self, dst: &mut [u8], length: usize) -> Result<usize, FileError>;
	fn write(&mut self, src: &[u8], length: usize) -> Result<usize, FileError>;
}

#[derive(Debug)]
pub enum FileError {
	Unknown()
}

/// Contains all file information.
/// Current information are only name and it's FileOperation trait object
/// The op trait object can be store either by reference of Box. For the moment Box is choosen but
/// this may change in the future. To make the trait object ThreadSafe Mutex is used.
/// Arc is used to allow multiple reference on the object in a multithreaded environment
pub struct FileInfo {
	pub name: String,
	pub op:   Arc<Mutex<Box<dyn FileOperation>, false>>
}
// Sync/Send marker to indicate rust that FileInfo is thread safe
unsafe impl Sync for FileInfo {}
unsafe impl Send for FileInfo {}

impl FileInfo {
	pub fn new(name: String, op: Box<dyn FileOperation>) -> Self {
		Self { name: name, op: Arc::new(Mutex::new(op)) }
	}
}

/// Currently this structure is only used to store op in the PROC_FILES vector
/// fd field isn't used.
/// TODO: This could be the structure returned by open containing fd, file size etc.... and could
/// close fd and drop
pub struct File {
	pub fd: usize,
	pub op: Arc<Mutex<Box<dyn FileOperation>, false>>
}
unsafe impl Sync for File {}
unsafe impl Send for File {}

impl File {
	#[inline]
	pub fn read(
		&self,
		dst: &mut [u8],
		length: usize
	) -> Result<usize, FileError> {
		crate::fs::read(self.fd, dst, length)
	}

	#[inline]
	pub fn write(
		&mut self,
		src: &[u8],
		length: usize
	) -> Result<usize, FileError> {
		crate::fs::write(self.fd, src, length)
	}
}

impl Drop for File {
	fn drop(&mut self) {
		// TODO Delete something? Close fd?
	}
}
