use crate::string::String;
use crate::boxed::Box;

pub trait FileOperation {
    fn read(&self, dst: &mut [u8], length: usize) -> usize;
    fn write(&mut self, src: &[u8], length: usize) -> usize;
}

unsafe impl Sync for File {}
unsafe impl Send for File {}
pub struct FileInfo {
    name: String,
    op: Box<dyn FileOperation>
}

impl FileInfo {
    pub fn new(name: String, op: Box<dyn FileOperation>) -> Self {
        Self {
            name:   name,
            op: op
        }
    } 
}

pub struct File {
    pub fd: usize
}

impl File {
    pub fn new(fd: usize) -> Self {
        Self {
            fd: fd
        }
    }

    pub fn value(&self) -> usize {
        self.fd
    }
}

impl FileOperation for File {

}

impl Drop for Fd {
    fn drop(&mut self) {
        crate::fs::delete(self.value());
    }
}


