use crate::boxed::Box;
use crate::string::String;
use crate::vec::Vec;
use crate::spin::Mutex;
use alloc::sync::Arc;

#[cfg(test)]
mod test;

mod file;
pub use file::*;

static SYSFILES: Mutex<Vec<FileInfo>, true> = Mutex::new(Vec::new());

pub fn create(file: FileInfo) -> Result<(), FileError> {
    let mut guard = SYSFILES.lock();

    let found_file = guard.iter()
                     .find(|elem| elem.name == file.name);
    match found_file {
        None => {
            guard.push(file);
            Ok(())
        },
        Some(file) => {
            crate::kprintln!("Found file {}", file.name);
            Err(FileError::Unknown())
        }
    }
}

pub fn create_from_raw<T: FileOperation + 'static>(name: &str, buffer: T) -> Result<(), FileError> {
    let file: FileInfo = FileInfo::new(String::from(name), Box::new(buffer));
    create(file)
}

pub fn delete(name: &str) {
    let mut guard = SYSFILES.lock();
    if let Some(index) = guard.iter().position(|elem| elem.name == name) {
        guard.remove(index);
    }
    // If this is not done, SYSFILES will still have memory allocated
    // And so test will fail for memory leaks
    // This shrinrk can save memory as well on running kernel
    // But this can cost tiny bit of performance
    if guard.is_empty() {
        guard.shrink_to_fit();
    }
}

pub fn open(name: &str) -> Result<usize, FileError> {
    let guard = SYSFILES.lock();

    let found_file = guard.iter()
                     .find(|elem| elem.name == name)
                     .ok_or(FileError::Unknown())?;
    let file: File = File {
        fd: 0,
        op: Arc::clone(&found_file.op)
    };
    let mut guard = PROC_FILES.lock();

    let index = guard.iter().position(|elem| elem.is_none()).ok_or(FileError::Unknown())?;
    guard[index] = Some(file);
    return Ok(index);
}

const DEFAULT: Option<File> = None;
static PROC_FILES: Mutex<[Option<File>; 32], false> = Mutex::new([DEFAULT; 32]);
pub fn close(fd: usize) {
    // TODO drop_in_place?
    PROC_FILES.lock()[fd] = None;
}

pub fn read(fd: usize, dst: &mut [u8], length: usize) -> Result<usize, FileError> {
    let mut guard = PROC_FILES.lock();
    let file = guard[fd].as_mut().ok_or(FileError::Unknown())?;
    let guard2 = file.op.lock();
    guard2.read(dst, length)
}

pub fn write(fd: usize, src: &[u8], length: usize) -> Result<usize, FileError> {
    let mut guard = PROC_FILES.lock();
    let file = guard[fd].as_mut().ok_or(FileError::Unknown())?;
    // This may cause panic if another thread write the same file
    // Wrapping a mutex inside the Arc may solve issue
    let mut guard2 = file.op.lock();
    guard2.write(src, length)
}
