use crate::fs;
use crate::string::String;
use crate::boxed::Box;

// Test buffer to implement FileOperation
struct Buffer {
    buffer: [u8; 1024]
}
impl Buffer {
    pub const fn new() -> Self {
        Self {
            buffer: [0; 1024]
        }
    }
}

// Actual FileOperation impl
impl fs::FileOperation for Buffer {
    fn read(&self, dst: &mut [u8], length: usize) -> usize {
        for i in 0..length {
            if i >= self.buffer.len() || i >= dst.len() {
                return i;
            }
            dst[i] = self.buffer[i];
        }
        length
    }

    fn write(&mut self, src: &[u8], length: usize) -> usize {
        for i in 0..length {
            if i >= self.buffer.len() || i >= src.len() {
                return i;
            }
            self.buffer[i] = src[i];
        }
        length
    }
}

#[sys_macros::test]
fn test_file() {
    let mut buffer: Buffer = Buffer::new();
    let mut buffer2: Buffer = Buffer::new();
    let fd = fs::create_from_raw("test_file", buffer).expect("Failed to create file");
    let fd2 = fs::create_from_raw("test_file2", buffer2).expect("Failed to create file");
}

#[sys_macros::test]
fn test_file_op() {
    let mut buffer: Buffer = Buffer::new();
    let fd = fs::create_from_raw("test_file", buffer).expect("Failed to create file");

    

}

