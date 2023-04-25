use super::{FileOperation, FileError};
use crate::alloc::vec::Vec;
use crate::alloc::string::String;
use crate::alloc::boxed::Box;
use crate::spin::Mutex;
use alloc::sync::Arc;
use super::FileInfo;

#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
enum SocketDomain {
    AF_UNIX,
    AF_INET
}
#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
enum SocketType {
    SOCK_STREAM,
    SOCK_DGRAM
}
#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
enum SocketProtocol{
    NONE
}

struct Socket {
    buffer:   Arc<Mutex<Vec<u8>, false>>,
    domain:   SocketDomain,  // TODO AF_INET
    stype:    SocketType,    // TODO
    protocol: SocketProtocol // TODO
}

impl FileOperation for Socket {
    fn read(&self, dst: &mut [u8], length: usize) -> Result<usize, FileError> {
        todo!()
    }

    fn write(&mut self, dst: &[u8], length: usize) -> Result<usize, FileError> {
        todo!()
    }
}

impl Socket {
    pub fn new(domain: SocketDomain, stype: SocketType, protocol: SocketProtocol) -> Self {
        Self {
            buffer: Arc::new(Mutex::new(Vec::new())),
            domain,
            stype,
            protocol
        }
    }

    pub fn clone(&self) -> Self {
        Self {
            buffer:   Arc::clone(&self.buffer),
            domain:   self.domain,
            stype:    self.stype,
            protocol: self.protocol
        }
    }
}

fn create_pair(domain: SocketDomain, stype: SocketType, protocol: SocketProtocol) -> Result<(), FileError> {
    let first_socket = Socket::new(domain, stype, protocol);
    let second_socket = first_socket.clone();
	super::create(FileInfo::new(String::from(""), super::FileType::Socket, Box::new(first_socket)))?;
    // If fail occur in the second, we should delete the first_one
	super::create(FileInfo::new(String::from(""), super::FileType::Socket, Box::new(second_socket)))?;
    Ok(())
}
