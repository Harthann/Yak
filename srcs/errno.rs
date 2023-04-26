#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ErrNo {
	EPERM           = 1,   // Operation not permitted
	ENOENT          = 2,   // No such file or directory
	ESRCH           = 3,   // No such process
	EINTR           = 4,   // Interrupted system call
	EIO             = 5,   // I/O error
	ENXIO           = 6,   // No such device or address
	E2BIG           = 7,   // Arg list too long
	ENOEXEC         = 8,   // Exec format error
	EBADF           = 9,   // Bad file number
	ECHILD          = 10,  // No child processes
	EAGAIN          = 11,  // Try again
	ENOMEM          = 12,  // Out of memory
	EACCES          = 13,  // Permission denied
	EFAULT          = 14,  // Bad address
	ENOTBLK         = 15,  // Block device required
	EBUSY           = 16,  // Device or resource busy
	EEXIST          = 17,  // File exists
	EXDEV           = 18,  // Cross-device link
	ENODEV          = 19,  // No such device
	ENOTDIR         = 20,  // Not a directory
	EISDIR          = 21,  // Is a directory
	EINVAL          = 22,  // Invalid argument
	ENFILE          = 23,  // File table overflow
	EMFILE          = 24,  // Too many open files
	ENOTTY          = 25,  // Not a typewriter
	ETXTBSY         = 26,  // Text file busy
	EFBIG           = 27,  // File too large
	ENOSPC          = 28,  // No space left on device
	ESPIPE          = 29,  // Illegal seek
	EROFS           = 30,  // Read-only file system
	EMLINK          = 31,  // Too many links
	EPIPE           = 32,  // Broken pipe
	EDOM            = 33,  // Math argument out of domain of func
	ERANGE          = 34,  // Math result not representable
	EDEADLK         = 35,  // Resource deadlock would occur
	ENAMETOOLONG    = 36,  // File name too long
	ENOLCK          = 37,  // No record locks available
	ENOSYS          = 38,  // Function not implemented
	ENOTEMPTY       = 39,  // Directory not empty
	ELOOP           = 40,  // Too many symbolic links encountered
	EWOULDBLOCK     = 41,  // Operation would block
	ENOMSG          = 42,  // No message of desired type
	EIDRM           = 43,  // Identifier removed
	ECHRNG          = 44,  // Channel number out of range
	EL2NSYNC        = 45,  // Level 2 not synchronized
	EL3HLT          = 46,  // Level 3 halted
	EL3RST          = 47,  // Level 3 reset
	ELNRNG          = 48,  // Link number out of range
	EUNATCH         = 49,  // Protocol driver not attached
	ENOCSI          = 50,  // No CSI structure available
	EL2HLT          = 51,  // Level 2 halted
	EBADE           = 52,  // Invalid exchange
	EBADR           = 53,  // Invalid request descriptor
	EXFULL          = 54,  // Exchange full
	ENOANO          = 55,  // No anode
	EBADRQC         = 56,  // Invalid request code
	EBADSLT         = 57,  // Invalid slot
	EDEADLOCK       = 58,  // File locking deadlock error
	EBFONT          = 59,  // Bad font file format
	ENOSTR          = 60,  // Device not a stream
	ENODATA         = 61,  // No data available
	ETIME           = 62,  // Timer expired
	ENOSR           = 63,  // Out of streams resources
	ENONET          = 64,  // Machine is not on the network
	ENOPKG          = 65,  // Package not installed
	EREMOTE         = 66,  // Object is remote
	ENOLINK         = 67,  // Link has been severed
	EADV            = 68,  // Advertise error
	ESRMNT          = 69,  // Srmount error
	ECOMM           = 70,  // Communication error on send
	EPROTO          = 71,  // Protocol error
	EMULTIHOP       = 72,  // Multihop attempted
	EDOTDOT         = 73,  // RFS specific error
	EBADMSG         = 74,  // Not a data message
	EOVERFLOW       = 75,  // Value too large for defined data type
	ENOTUNIQ        = 76,  // Name not unique on network
	EBADFD          = 77,  // File descriptor in bad state
	EREMCHG         = 78,  // Remote address changed
	ELIBACC         = 79,  // Can not access a needed shared library
	ELIBBAD         = 80,  // Accessing a corrupted shared library
	ELIBSCN         = 81,  // .lib section in a.out corrupted
	ELIBMAX         = 82,  /* Attempting to link in too many shared libraries */
	ELIBEXEC        = 83,  // Cannot exec a shared library directly
	EILSEQ          = 84,  // Illegal byte sequence
	ERESTART        = 85,  // Interrupted system call should be restarted
	ESTRPIPE        = 86,  // Streams pipe error
	EUSERS          = 87,  // Too many users
	ENOTSOCK        = 88,  // Socket operation on non-socket
	EDESTADDRREQ    = 89,  // Destination address required
	EMSGSIZE        = 90,  // Message too long
	EPROTOTYPE      = 91,  // Protocol wrong type for socket
	ENOPROTOOPT     = 92,  // Protocol not available
	EPROTONOSUPPORT = 93,  // Protocol not supported
	ESOCKTNOSUPPORT = 94,  // Socket type not supported
	EOPNOTSUPP      = 95,  // Operation not supported on transport endpoint
	EPFNOSUPPORT    = 96,  // Protocol family not supported
	EAFNOSUPPORT    = 97,  // Address family not supported by protocol
	EADDRINUSE      = 98,  // Address already in use
	EADDRNOTAVAIL   = 99,  // Cannot assign requested address
	ENETDOWN        = 100, // Network is down
	ENETUNREACH     = 101, // Network is unreachable
	ENETRESET       = 102, // Network dropped connection because of reset
	ECONNABORTED    = 103, // Software caused connection abort
	ECONNRESET      = 104, // Connection reset by peer
	ENOBUFS         = 105, // No buffer space available
	EISCONN         = 106, // Transport endpoint is already connected
	ENOTCONN        = 107, // Transport endpoint is not connected
	ESHUTDOWN       = 108, // Cannot send after transport endpoint shutdown
	ETOOMANYREFS    = 109, // Too many references: cannot splice
	ETIMEDOUT       = 110, // Connection timed out
	ECONNREFUSED    = 111, // Connection refused
	EHOSTDOWN       = 112, // Host is down
	EHOSTUNREACH    = 113, // No route to host
	EALREADY        = 114, // Operation already in progress
	EINPROGRESS     = 115, // Operation now in progress
	ESTALE          = 116, // Stale NFS file handle
	EUCLEAN         = 117, // Structure needs cleaning
	ENOTNAM         = 118, // Not a XENIX named type file
	ENAVAIL         = 119, // No XENIX semaphores available
	EISNAM          = 120, // Is a named type file
	EREMOTEIO       = 121, // Remote I/O error
	// Should never be seen by user programs
	ERESTARTSYS     = 512,
	ERESTARTNOINTR  = 513
}

pub fn strerror(errno: ErrNo) -> &'static str {
    STRERROR[errno as usize - 1]
}

static STRERROR: [&str; 121] = [
"Operation not permitted",
"No such file or directory",
"No such process",
"Interrupted system call",
"I/O error",
"No such device or address",
"Arg list too long",
"Exec format error",
"Bad file number",
"No child processes",
"Try again",
"Out of memory",
"Permission denied",
"Bad address",
"Block device required",
"Device or resource busy",
"File exists",
"Cross-device link",
"No such device",
"Not a directory",
"Is a directory",
"Invalid argument",
"File table overflow",
"Too many open files",
"Not a typewriter",
"Text file busy",
"File too large",
"No space left on device",
"Illegal seek",
"Read-only file system",
"Too many links",
"Broken pipe",
"Math argument out of domain of func",
"Math result not representable",
"Resource deadlock would occur",
"File name too long",
"No record locks available",
"Function not implemented",
"Directory not empty",
"Too many symbolic links encountered",
"Operation would block",
"No message of desired type",
"Identifier removed",
"Channel number out of range",
"Level 2 not synchronized",
"Level 3 halted",
"Level 3 reset",
"Link number out of range",
"Protocol driver not attached",
"No CSI structure available",
"Level 2 halted",
"Invalid exchange",
"Invalid request descriptor",
"Exchange full",
"No anode",
"Invalid request code",
"Invalid slot",
"File locking deadlock error",
"Bad font file format",
"Device not a stream",
"No data available",
"Timer expired",
"Out of streams resources",
"Machine is not on the network",
"Package not installed",
"Object is remote",
"Link has been severed",
"Advertise error",
"Srmount error",
"Communication error on send",
"Protocol error",
"Multihop attempted",
"RFS specific error",
"Not a data message",
"Value too large for defined data type",
"Name not unique on network",
"File descriptor in bad state",
"Remote address changed",
"Can not access a needed shared library",
"Accessing a corrupted shared library",
".lib section in a.out corrupted",
"Attempting to link in too many shared libraries */",
"Cannot exec a shared library directly",
"Illegal byte sequence",
"Interrupted system call should be restarted",
"Streams pipe error",
"Too many users",
"Socket operation on non-socket",
"Destination address required",
"Message too long",
"Protocol wrong type for socket",
"Protocol not available",
"Protocol not supported",
"Socket type not supported",
"Operation not supported on transport endpoint",
"Protocol family not supported",
"Address family not supported by protocol",
"Address already in use",
"Cannot assign requested address",
"Network is down",
"Network is unreachable",
"Network dropped connection because of reset",
"Software caused connection abort",
"Connection reset by peer",
"No buffer space available",
"Transport endpoint is already connected",
"Transport endpoint is not connected",
"Cannot send after transport endpoint shutdown",
"Too many references: cannot splice",
"Connection timed out",
"Connection refused",
"Host is down",
"No route to host",
"Operation already in progress",
"Operation now in progress",
"Stale NFS file handle",
"Structure needs cleaning",
"Not a XENIX named type file",
"No XENIX semaphores available",
"Is a named type file",
"Remote I/O error"
];
