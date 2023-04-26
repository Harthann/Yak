use crate::boxed::Box;
use crate::spin::Mutex;
use crate::string::String;
use alloc::sync::Arc;
use crate::errno::ErrNo;

pub trait FileOperation {
	fn read(&self, dst: &mut [u8], length: usize) -> Result<usize, ErrNo>;
	fn write(&mut self, src: &[u8], length: usize) -> Result<usize, ErrNo>;
}

/// Contains all file information.
/// Current informatin are only it's name and it's FileOperation trait object
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

