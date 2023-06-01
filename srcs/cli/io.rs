use crate::fs::FileOperation;
use crate::fs::FileError;
use crate::cli::INPUT_BUFFER;
use crate::vec::Vec;
use core::cell::RefCell;

pub struct Stdin {
    buffer: RefCell<Vec<u8>>
}

impl Stdin {
    fn read_from_buffer(&self, dst: &mut [u8], length: usize) -> Result<usize, FileError> {
        for i in 0..length {
            dst[i] = self.buffer.borrow_mut().pop().ok_or(FileError::Unknown())?;
        }
        Ok(length)
    }
}

impl Default for Stdin {
    fn default() -> Self {
        Self {
            buffer: RefCell::new(Vec::new())
        }
    }
}

impl FileOperation for Stdin {

    fn read(&self, dst: &mut [u8], length: usize) -> Result<usize, FileError> {
        let mut readed: usize = 0;

        if self.buffer.borrow().len() >= length {
            return self.read_from_buffer(dst, length);
        }
        
        while readed < length {
            if INPUT_BUFFER.lock().as_ref().unwrap().is_empty() {
                unsafe { crate::wrappers::hlt!(); }
            } else {
                dst[readed] = INPUT_BUFFER.lock().as_mut().unwrap().pop();
                readed += 1;
            }
        }
        Ok(readed)
    }

    fn write(&mut self, src: &[u8], lenth: usize) -> Result<usize, FileError> {
        todo!()
    }
}
