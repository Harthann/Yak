use crate::boxed::Box;
use crate::spin::Mutex;
use crate::string::String;
use crate::vec::Vec;
use alloc::sync::Arc;

#[cfg(test)]
mod test;

mod file;
pub use file::*;

// Contain all file system. This will be probably converted to a BST or something like that
// FileInfo will have to contain permission, file type information etc etc
static SYSFILES: Mutex<Vec<FileInfo>, true> = Mutex::new(Vec::new());

/// Take information on a file and add it to SYSFILES if it does not exist
/// FileError is return if file already found.
pub fn create(file: FileInfo) -> Result<(), FileError> {
	let mut guard = SYSFILES.lock();

	let found_file = guard.iter().find(|elem| elem.name == file.name);
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

/// Create a file given it's name and a predefined buffer. The buffer should implement
/// FileOperation trait.
/// WARNINGS: This does use String and Box to create FileInfo, no file can be created before Heap
/// creation
pub fn create_from_raw<T: FileOperation + 'static>(
	name: &str,
	buffer: T
) -> Result<(), FileError> {
	let file: FileInfo = FileInfo::new(String::from(name), Box::new(buffer));
	create(file)
}

/// Delete file from SYSFILE given its name
/// Should be updated later to check permission on the file
pub fn delete(name: &str) {
	let mut guard = SYSFILES.lock();
	if let Some(index) = guard.iter().position(|elem| elem.name == name) {
		guard.remove(index);
	}
	// If this is not done, SYSFILES will still have memory allocated
	// And so test will fail for memory leaks
	// This shrink can save memory as well on running kernel
	// But this can cost tiny bit of performance
	if guard.is_empty() {
		guard.shrink_to_fit();
	}
}

const DEFAULT: Option<File> = None;
// This static is temporary, file array should be bind to each process individually
static PROC_FILES: Mutex<[Option<File>; 32], false> = Mutex::new([DEFAULT; 32]);

/// Look for a file given it's name in SYSFILES and open it.
/// Open files list is common between processses, this will change in later version
pub fn open(name: &str) -> Result<usize, FileError> {
	let guard = SYSFILES.lock();

	let found_file = guard
		.iter()
		.find(|elem| elem.name == name)
		.ok_or(FileError::Unknown())?;
	let file: File = File { fd: 0, op: Arc::clone(&found_file.op) };
	let mut guard = PROC_FILES.lock();

	let index = guard
		.iter()
		.position(|elem| elem.is_none())
		.ok_or(FileError::Unknown())?;
	guard[index] = Some(file);
	return Ok(index);
}

/// Close a file given it's file descriptor. This does not delete the file from the system
pub fn close(fd: usize) {
	// TODO drop_in_place?
	PROC_FILES.lock()[fd] = None;
}

/// This function mimic the linux read syscall. Look for a file in file lists and call it's
/// FileOperation implementation. Mutex on PROC_FILES is acquire during all the read processus
/// which imply you can't r/w another file at the same time.
pub fn read(
	fd: usize,
	dst: &mut [u8],
	length: usize
) -> Result<usize, FileError> {
	let mut guard = PROC_FILES.lock();
	let file = guard[fd].as_mut().ok_or(FileError::Unknown())?;
	let guard2 = file.op.lock();
	guard2.read(dst, length)
}

/// This function mimic the linux write syscall. Look for a file in file lists and call it's
/// FileOperation implementation. Mutex on PROC_FILES is acquire during all the read processus
/// which imply you can't r/w another file at the same time.
pub fn write(fd: usize, src: &[u8], length: usize) -> Result<usize, FileError> {
	let mut guard = PROC_FILES.lock();
	let file = guard[fd].as_mut().ok_or(FileError::Unknown())?;
	let mut guard2 = file.op.lock();
	guard2.write(src, length)
}
