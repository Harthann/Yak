use crate::string::String;
use crate::boxed::Box;
use alloc::sync::Arc;
use crate::spin::Mutex;

pub trait FileOperation {
    fn read(&self, dst: &mut [u8], length: usize) -> Result<usize, FileError>;
    fn write(&mut self, src: &[u8], length: usize) -> Result<usize, FileError>;
}

#[derive(Debug)]
pub enum FileError {
    Unknown()
}

unsafe impl Sync for FileInfo {}
unsafe impl Send for FileInfo {}
unsafe impl Sync for File {}
unsafe impl Send for File {}
pub struct FileInfo {
    pub name: String,
    pub op: Arc<Mutex<Box<dyn FileOperation>, false>>
}

impl FileInfo {
    pub fn new(name: String, op: Box<dyn FileOperation>) -> Self {
        Self {
            name:   name,
            op: Arc::new(Mutex::new(op))
        }
    } 

}

pub struct File {
    pub fd: usize,
    pub op: Arc<Mutex<Box<dyn FileOperation>, false>>
}

impl File {
    //pub fn new(fd: usize) -> Self {
    //    Self {
    //        fd: fd
    //    }
    //}

    #[inline]
    pub fn read(&self, dst: &mut [u8], length: usize) -> Result<usize, FileError> {
        crate::fs::read(self.fd, dst, length)
    }

    #[inline]
    pub fn write(&mut self, src: &[u8], length: usize) -> Result<usize, FileError> {
        crate::fs::write(self.fd, src, length)
    }
}

impl Drop for File {
    fn drop(&mut self) {
    }
}


