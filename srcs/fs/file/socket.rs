use crate::vec::Vec;
use super::FileOperation;
use crate::errno::ErrNo;
use crate::spin::Mutex;
use alloc::sync::Arc;

#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
pub enum SocketDomain {
    AF_UNIX,
    AF_INET
}
#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
pub enum SocketType {
    SOCK_RAW,
    SOCK_DGRAM,
    SOCK_STREAM
}
#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
pub enum SocketProtocol {
    DEFAULT
}

pub struct Socket {
    domain:   SocketDomain,
    stype:    SocketType,
    protocol: SocketProtocol,
    buffer: Option<Arc<Mutex<[Vec<u8>; 2], false>>>,
    endpoint: usize
}

impl Socket {
    pub const fn new(domain: SocketDomain, stype: SocketType, protocol: SocketProtocol) -> Self {
        Self {
            domain,
            stype,
            protocol,
            buffer: None,
            endpoint: 0
        }
    }
}

impl FileOperation for Socket {
    fn read(&self, dst: &mut [u8], length: usize) -> Result<usize, ErrNo> {
        match self.stype {
            SocketType::SOCK_RAW    => self.raw_read(dst, length),
            SocketType::SOCK_STREAM => self.stream_read(dst, length),
            SocketType::SOCK_DGRAM  => self.dgram_read(dst, length)
        }
    }

    fn write(&mut self, src: &[u8], length: usize) -> Result<usize, ErrNo> {
        match self.stype {
            SocketType::SOCK_RAW    => self.raw_write(src, length),
            SocketType::SOCK_STREAM => self.stream_write(src, length),
            SocketType::SOCK_DGRAM  => self.dgram_write(src, length)
        }
    }
}

/// FileOperations for DGRAM sockets
impl Socket {
    fn dgram_read(&self, dst: &mut [u8], length: usize) -> Result<usize, ErrNo> {
        let _size = core::cmp::min(dst.len(), length);
        todo!()
    }

    fn dgram_write(&mut self, src: &[u8], length: usize) -> Result<usize, ErrNo> {
        match &self.buffer {
            Some(buffer) => {
                let _guard = &buffer.lock()[self.endpoint];
                let _writing = core::cmp::min(length, src.len());
                todo!()
            },
            None => {todo!()}
        }
    }
}

/// FileOperations for STREAM sockets
impl Socket {
    fn stream_read(&self, _dst: &mut [u8], _length: usize) -> Result<usize, ErrNo> {
        todo!()
    }

    fn stream_write(&mut self, _src: &[u8], _length: usize) -> Result<usize, ErrNo> {
        todo!()
    }
}

/// FileOperations for RAW sockets
impl Socket {
    fn raw_read(&self, _dst: &mut [u8], _length: usize) -> Result<usize, ErrNo> {
        todo!()
    }

    fn raw_write(&mut self, _src: &[u8], _length: usize) -> Result<usize, ErrNo> {
        todo!()
    }
}

pub fn create_socket_pair(
    domain: SocketDomain,
    stype: SocketType,
    protocol: SocketProtocol)
    -> Result<(Socket, Socket), ErrNo>
{
    let mut first_socket = Socket::new(domain, stype, protocol);
    let mut second_socket = Socket::new(domain, stype, protocol);
    let buffer: Arc<Mutex<[Vec<u8>; 2], false>> = Arc::new(Mutex::default());
    // Clone the reference to our buffer and assign our endpoint index to 1
    second_socket.buffer = Some(Arc::clone(&buffer));
    second_socket.endpoint = 1;
    // Move the reference to our buffer and assign our endpoint index to 0
    first_socket.buffer = Some(buffer);
    first_socket.endpoint = 0;
    Ok((first_socket, second_socket))

}
