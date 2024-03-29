use crate::alloc::sync::Arc;
use crate::errno::ErrNo;
use crate::fs::FileOperation;
use crate::proc::process::MAX_FD;
use crate::spin::KMutex;
use crate::string::String;
use crate::utils::arcm::Arcm;
use crate::vec::Vec;

/// TODO! Allow each syscalls that open an fd to return an object that implement close on drop to
/// avoid leaks due to unused close. This will make also use of full rust capabilities and lifetime

#[cfg(test)]
mod test;

pub mod ext2;
mod file;
pub use file::*;

// Contain all file system. This will be probably converted to a BST or something like that
// FileInfo will have to contain permission, file type information etc etc
static SYSFILES: KMutex<Vec<Arc<FileInfo>>> = KMutex::new(Vec::new());

/// Take information on a file and add it to SYSFILES if it does not exist
/// ErrNo is return if file already found.
pub fn create(file: FileInfo) -> Result<(), ErrNo> {
	let mut guard = SYSFILES.lock();

	let found_file = guard.iter().find(|elem| elem.name == file.name);
	match found_file {
		None => {
			guard.push(Arc::new(file));
			Ok(())
		},
		Some(file) => {
			crate::kprintln!("Found file {}", file.name);
			Err(ErrNo::EEXIST)
		}
	}
}

/// Create a file given its name and a predefined buffer. The buffer should implement
/// FileOperation trait.
/// WARNINGS: This does use String and Box to create FileInfo, no file can be created before Heap
/// creation
pub fn create_from_raw<T: FileOperation + 'static>(
	name: &str,
	buffer: T
) -> Result<(), ErrNo> {
	let file: FileInfo = FileInfo::new(String::from(name), Arcm::new(buffer));
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

/// Look for a file given its name in SYSFILES and open it.
/// Open files list is common between processses, this will change in later version
pub fn open(name: &str) -> Result<usize, ErrNo> {
	let guard = SYSFILES.lock();

	// Error if file does not exist
	let found_file = guard
		.iter()
		.find(|elem| elem.name == name)
		.ok_or(ErrNo::ENOENT)?;
	let binding = Process::get_running_process();
	let mut curr_process = binding.lock();

	// Error if file table already full
	let index = curr_process
		.fds
		.iter()
		.position(|elem| elem.is_none())
		.ok_or(ErrNo::EMFILE)?;
	curr_process.fds[index] = Some(Arc::clone(&found_file));
	return Ok(index);
}

/// Close a file given its file descriptor. This does not delete the file from the system
pub fn close(fd: usize) {
	// TODO drop_in_place?
	if fd < MAX_FD {
		let binding = Process::get_running_process();
		let mut curr_process = binding.lock();
		curr_process.fds[fd] = None;
	}
}

/// This function mimic the linux read syscall. Look for a file in file lists and call it's
/// FileOperation implementation. Mutex on PROC_FILES is acquire during all the read processus
/// which imply you can't r/w another file at the same time.
pub fn read(fd: usize, dst: &mut [u8], length: usize) -> Result<usize, ErrNo> {
	if fd >= MAX_FD {
		return Err(ErrNo::EBADF);
	}

	let binding = Process::get_running_process();
	let guard2 = binding.execute(|guard| {
		let mut curr_process = guard.lock();
		let file = curr_process.fds[fd].as_mut().ok_or(ErrNo::EBADF)?;
		Ok(file.op.clone())
	})?;
	let fileop = guard2.lock();
	fileop.read(dst, length)
}

use crate::proc::process::Process;
/// This function mimic the linux write syscall. Look for a file in file lists and call it's
/// FileOperation implementation. Mutex on PROC_FILES is acquire during all the read processus
/// which imply you can't r/w another file at the same time.
pub fn write(fd: usize, src: &[u8], length: usize) -> Result<usize, ErrNo> {
	if fd >= MAX_FD {
		return Err(ErrNo::EBADF);
	}

	let binding = Process::get_running_process();
	let guard2 = binding.execute(|guard| {
		let mut curr_process = guard.lock();
		let file = curr_process.fds[fd].as_mut().ok_or(ErrNo::EBADF)?;
		Ok(file.op.clone())
	})?;
	let mut fileop = guard2.lock();
	fileop.write(src, length)
}

// SOCKET HELPERS
use file::socket::{SocketDomain, SocketProtocol, SocketType};
/// Create and open a pair of socket given it's domain, type and protocol.
/// Fd are written to sockets array. Prototype is made to match linux syscall
pub fn socket_pair(
	domain: SocketDomain,
	stype: SocketType,
	protocol: SocketProtocol,
	sockets: &mut [usize; 2]
) -> Result<usize, ErrNo> {
	let socket = file::socket::create_socket_pair(domain, stype, protocol)?;
	let socket1: FileInfo =
		FileInfo::new(String::from("socketfs"), Arcm::new(socket.0));
	let socket2: FileInfo =
		FileInfo::new(String::from("socketfs"), Arcm::new(socket.1));

	let binding = Process::get_running_process();
	let mut curr_process = binding.lock();

	// Open first socket
	let index = curr_process
		.fds
		.iter()
		.position(|elem| elem.is_none())
		.ok_or(ErrNo::EMFILE)?;
	curr_process.fds[index] = Some(Arc::new(socket1));

	// Open second socket
	let index2 = curr_process
		.fds
		.iter()
		.position(|elem| elem.is_none())
		.ok_or(ErrNo::EMFILE)?;
	curr_process.fds[index2] = Some(Arc::new(socket2));

	sockets[0] = index;
	sockets[1] = index2;
	Ok(0)
}
