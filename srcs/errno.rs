#[derive(Copy, Clone, PartialEq)]
pub enum ErrNo {
	Perm           = 1,   // Operation not permitted
	NoEnt          = 2,   // No such file or directory
	Srch           = 3,   // No such process
	Intr           = 4,   // Interrupted system call
	IO             = 5,   // I/O error
	NXio           = 6,   // No such device or address
	TooBig         = 7,   // Arg list too long
	NoExec         = 8,   // Exec format error
	BadF           = 9,   // Bad file number
	Child          = 10,  // No child processes
	Again          = 11,  // Try again
	NoMem          = 12,  // Out of memory
	Acces          = 13,  // Permission denied
	Fault          = 14,  // Bad address
	NotBlk         = 15,  // Block device required
	Busy           = 16,  // Device or resource busy
	Exist          = 17,  // File exists
	XDev           = 18,  // Cross-device link
	NoDev          = 19,  // No such device
	NotDir         = 20,  // Not a directory
	IsDir          = 21,  // Is a directory
	Inval          = 22,  // Invalid argument
	NFile          = 23,  // File table overflow
	MFile          = 24,  // Too many open files
	NoTTY          = 25,  // Not a typewriter
	TxtBsy         = 26,  // Text file busy
	FBig           = 27,  // File too large
	NoSpc          = 28,  // No space left on device
	SPipe          = 29,  // Illegal seek
	ROFs           = 30,  // Read-only file system
	MLink          = 31,  // Too many links
	Pipe           = 32,  // Broken pipe
	Dom            = 33,  // Math argument out of domain of func
	Range          = 34,  // Math result not representable
	DeadLk         = 35,  // Resource deadlock would occur
	NameTooLong    = 36,  // File name too long
	NoLck          = 37,  // No record locks available
	NoSys          = 38,  // Function not implemented
	NotEmpty       = 39,  // Directory not empty
	Loop           = 40,  // Too many symbolic links encountered
	WouldBlock     = 41,  // Operation would block
	NoMsg          = 42,  // No message of desired type
	IdRm           = 43,  // Identifier removed
	ChRng          = 44,  // Channel number out of range
	L2NSync        = 45,  // Level 2 not synchronized
	L3Hlt          = 46,  // Level 3 halted
	L3Rst          = 47,  // Level 3 reset
	LNRng          = 48,  // Link number out of range
	UnAtch         = 49,  // Protocol driver not attached
	NoCSI          = 50,  // No CSI structure available
	L2Hlt          = 51,  // Level 2 halted
	BadE           = 52,  // Invalid exchange
	BadR           = 53,  // Invalid request descriptor
	XFull          = 54,  // Exchange full
	NoAno          = 55,  // No anode
	BadRqC         = 56,  // Invalid request code
	BadSlt         = 57,  // Invalid slot
	Deadlock       = 58,  // File locking deadlock error
	BFont          = 59,  // Bad font file format
	NoStr          = 60,  // Device not a stream
	NoData         = 61,  // No data available
	Time           = 62,  // Timer expired
	NoSR           = 63,  // Out of streams resources
	NoNet          = 64,  // Machine is not on the network
	NoPkg          = 65,  // Package not installed
	Remote         = 66,  // Object is remote
	NoLink         = 67,  // Link has been severed
	Adv            = 68,  // Advertise error
	SrMnt          = 69,  // Srmount error
	Comm           = 70,  // Communication error on send
	Proto          = 71,  // Protocol error
	Multihop       = 72,  // Multihop attempted
	DotDot         = 73,  // RFS specific error
	BadMsg         = 74,  // Not a data message
	Overflow       = 75,  // Value too large for defined data type
	NotUniq        = 76,  // Name not unique on network
	BadFd          = 77,  // File descriptor in bad state
	RemChg         = 78,  // Remote address changed
	LibAcc         = 79,  // Can not access a needed shared library
	LibBad         = 80,  // Accessing a corrupted shared library
	LibScn         = 81,  // .lib section in a.out corrupted
	LibMax         = 82,  /* Attempting to link in too many shared libraries */
	LibExec        = 83,  // Cannot exec a shared library directly
	IlSeq          = 84,  // Illegal byte sequence
	Restart        = 85,  // Interrupted system call should be restarted
	StrPipe        = 86,  // Streams pipe error
	Users          = 87,  // Too many users
	NotSock        = 88,  // Socket operation on non-socket
	DestAddrReq    = 89,  // Destination address required
	MsgSize        = 90,  // Message too long
	Prototype      = 91,  // Protocol wrong type for socket
	NoProtoOpt     = 92,  // Protocol not available
	ProtoNoSupport = 93,  // Protocol not supported
	SocktNoSupport = 94,  // Socket type not supported
	OpNotSupp      = 95,  // Operation not supported on transport endpoint
	PFNoSupport    = 96,  // Protocol family not supported
	AFNoSupport    = 97,  // Address family not supported by protocol
	AddrInUse      = 98,  // Address already in use
	AddrNotAvail   = 99,  // Cannot assign requested address
	NetDown        = 100, // Network is down
	NetUnreach     = 101, // Network is unreachable
	NetReset       = 102, // Network dropped connection because of reset
	ConnAborted    = 103, // Software caused connection abort
	ConnReset      = 104, // Connection reset by peer
	NoBufs         = 105, // No buffer space available
	IsConn         = 106, // Transport endpoint is already connected
	NotConn        = 107, // Transport endpoint is not connected
	Shutdown       = 108, // Cannot send after transport endpoint shutdown
	TooManyRefs    = 109, // Too many references: cannot splice
	Timedout       = 110, // Connection timed out
	ConnRefused    = 111, // Connection refused
	HostDown       = 112, // Host is down
	HostUnreach    = 113, // No route to host
	Already        = 114, // Operation already in progress
	InProgress     = 115, // Operation now in progress
	Stale          = 116, // Stale NFS file handle
	UClean         = 117, // Structure needs cleaning
	NotNam         = 118, // Not a XENIX named type file
	NAvail         = 119, // No XENIX semaphores available
	IsNam          = 120, // Is a named type file
	RemoteIO       = 121, // Remote I/O error
	// Should never be seen by user programs
	RestartSys     = 512,
	RestartNoIntr  = 513
}

use core::fmt;
impl fmt::Debug for ErrNo {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "ErrNo: [{:?}] {}", *self as usize, strerror(*self))
	}
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
