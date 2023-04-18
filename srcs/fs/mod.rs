use crate::vec::Vec;
use crate::boxed::Box;
use crate::string::String;
use crate::spin::Mutex;

#[cfg(test)]
mod test;

mod file;
pub use file::*;

const DEFAULT_NONE: Option<File> = None;
static KFILES: Mutex<[Option<File>; 32], true> = Mutex::new([DEFAULT_NONE; 32]);

pub fn create(mut file: File) -> Result<Fd, ()> {
    let mut guard = KFILES.lock();

    let index = guard.iter().position(|elem| elem.is_none()).ok_or(())?;
    guard[index] = Some(file);
    return Ok(Fd::new(index));
}

pub fn create_from_raw<T: FileOperation + 'static>(name: &str, buffer: T) -> Result<Fd, ()> {
    let mut file: File = File::new(String::from(name), Box::new(buffer));
    create(file)
}

pub fn delete(fd: usize) {
    KFILES.lock()[fd] = None;
}

