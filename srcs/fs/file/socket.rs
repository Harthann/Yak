use crate::vec::Vec;
use super::FileOperation;
use crate::errno::ErrNo;
use crate::spin::Mutex;
use alloc::sync::Arc;
use core::cell::RefCell;

/// Represent the different domains of a socket.
/// AF_UNIX bind the socket to the system as file.
/// AF_INET bind the socket to a network connect. Not Yet Implemented
#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
pub enum SocketDomain {
    AF_UNIX,
    AF_INET
}
/// Represent the type of the socket.
/// SOCK_RAW: Not yet implemented
/// SOCK_DGRAM: Partially implemented
/// SOCK_STREAM: Not yet implemented
#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
pub enum SocketType {
    SOCK_RAW,
    SOCK_DGRAM,
    SOCK_STREAM
}
/// Represent the protocol bound to the socket.
/// Generally only one protocol is implemented for each socket type.
/// Currently no protocol are implemented
#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
pub enum SocketProtocol {
    DEFAULT
}

// Maybe replace Vec<u8> with reference on slice
// The slice could be obtain with mmap and will embed it's length.
// If we'll overflow the slice buffer, we could request a new page and etend our slice
// to wrap these function, the slice could be a MemoryZone structure

/// Socket structure representation. Socket alone can't do much. These need to be created by pair,
/// Each socket will be tide to both endpoint but we'll access only one by writing or reading.
/// UNIX domain will create 2 buffers for both endpoint of the socket.
/// INET domain is not implemented but could create an endpoint of the socket bound to the network
/// interface
/// TODO? Maybe embed the woffset inside the buffer to precisely know how much byte as been written
pub struct Socket {
    domain:   SocketDomain,
    stype:    SocketType,
    protocol: SocketProtocol,
    roffset:  RefCell<usize>, // needed for interior mutability in read
    woffset:  usize,
    buffer:   Option<[Arc<Mutex<Vec<u8>, false>>; 2]>,
    endpoint: usize
}

impl Socket {
    /// Create a socket given it's domain, type and protocol
    pub const fn new(domain: SocketDomain, stype: SocketType, protocol: SocketProtocol) -> Self {
        Self {
            domain,
            stype,
            protocol,
            roffset: RefCell::new(0),
            woffset: 0,
            buffer: None,
            endpoint: 0
        }
    }
}

impl FileOperation for Socket {
    /// Redirect to the read appropriate to the socket type
    fn read(&self, dst: &mut [u8], length: usize) -> Result<usize, ErrNo> {
        match self.stype {
            SocketType::SOCK_RAW    => self.raw_read(dst, length),
            SocketType::SOCK_STREAM => self.stream_read(dst, length),
            SocketType::SOCK_DGRAM  => self.dgram_read(dst, length)
        }
    }

    /// Redirect to the write appropriate to the socket type
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
        match &self.buffer {
            Some(buffer) => {
                let mut roffset = self.roffset.borrow_mut();
                let reading = core::cmp::min(dst.len(), length);
                // If nobody is writing to buffer this causes a deadlock
                // for later use woffset to know how much as been written and not lock waiting
                // input
                while buffer[0].lock().len() < *roffset+reading {
                    unsafe { hlt!()};
                }
                let guard = &mut buffer[0].lock();
                dst[0..reading].copy_from_slice(&guard.as_slice()[*roffset..*roffset+reading]);
                *roffset += reading;
                // panic if overflow?
                Ok(reading)
            },
            None => {todo!()}
        }
    }

    fn dgram_write(&mut self, src: &[u8], length: usize) -> Result<usize, ErrNo> {
        match &self.buffer {
            Some(buffer) => {
                let guard = &mut buffer[1].lock();
                let writing = core::cmp::min(length, src.len());
                // Need to store offset of writing if no vector are used
                // and access the array from this store offset
                for i in 0..writing {
                    guard.push(src[i]);
                }
                self.woffset += writing;
                Ok(writing)
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

/// Create two sockets that are bound together, and can be read and write
pub fn create_socket_pair(
    domain: SocketDomain,
    stype: SocketType,
    protocol: SocketProtocol)
    -> Result<(Socket, Socket), ErrNo>
{
    let mut first_socket = Socket::new(domain, stype, protocol);
    let mut second_socket = Socket::new(domain, stype, protocol);
    let buffer1: Arc<Mutex<Vec<u8>, false>> = Arc::new(Mutex::default());
    let buffer2: Arc<Mutex<Vec<u8>, false>> = Arc::new(Mutex::default());

    // Clone the reference to our buffers. Index 0 will be readed, index 1 will be writed to
    second_socket.buffer = Some([Arc::clone(&buffer1), Arc::clone(&buffer2)]);
    // Move the reference to our buffers. Index 0 will be readed, index 1 will be writed to
    first_socket.buffer = Some([buffer2, buffer1]);
    Ok((first_socket, second_socket))

}


#[cfg(test)]
mod test {
    use super::{Socket, SocketType, SocketDomain, SocketProtocol};
    use super::create_socket_pair;
    use super::FileOperation;

    #[sys_macros::test_case]
    fn test_socket() {
        let mut sockets: (Socket, Socket);
        let input = "Hellow World";
        let mut buffer: [u8; 255] = [0; 255];

        sockets = create_socket_pair(SocketDomain::AF_UNIX,
                                     SocketType::SOCK_DGRAM,
                                     SocketProtocol::DEFAULT)
                  .expect("Error creating sockets");
        sockets.1.write(input.as_bytes(), input.len()).expect("Failed writing to socket 1");
        sockets.0.read(&mut buffer, input.len()).expect("Failed reading socket 0");
        assert_eq!(input.as_bytes(), &buffer[0..input.len()]);
        
        let input2 = "This is a success";
        buffer = [0; 255];
        sockets.0.write(input2.as_bytes(), input2.len()).expect("Failed writing to socket 0");
        sockets.1.read(&mut buffer, input2.len()).expect("Failed reading socket 1");
        assert_eq!(input2.as_bytes(), &buffer[0..input2.len()]);
    }
}
