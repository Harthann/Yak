use crate::errno::ErrNo;
use crate::proc::process::Process;
use crate::proc::Id;

pub type SigHandlerFn = fn(i32);

#[derive(Clone)]
pub struct SignalHandler {
	pub signal:  i32,
	pub handler: SigHandlerFn
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum SignalType {
	SigHup    = 1,  // Hangup
	SigInt    = 2,  // Terminal interrupt signal
	SigQuit   = 3,  // Terminal quit signal
	SigIll    = 4,  // Illegal instruction
	SigTrap   = 5,  // Trace/breakpoint trap
	SigAbrt   = 6,  // Process abort signal
	SigBus    = 7,  // Access to an undefined portion of a memory object
	SigFpe    = 8,  // Erroneous arithmetic operation
	SigKill   = 9,  // Kill (cannot be caught or ignored)
	SigUsr1   = 10, // User-defined signal 1
	SigSegv   = 11, // Invalid memory reference
	SigUsr2   = 12, // User-defined signal 2
	SigPipe   = 13, // Write on a pipe with no one to read it
	SigAlrm   = 14, // Alarm clock
	SigTerm   = 15, // Termination signal
	SigStkFlt = 16, // The coprocessor experiences a stack fault
	SigChld   = 17, // Child process terminated, stopped, or continued
	SigCont   = 18, // Continue executing, if stopped
	SigStop   = 19, // Stop executing (cannot be caught or ignored)
	SigTStp   = 20, // Terminal stop signal
	SigTTIn   = 21, // Background process attempting read
	SigTTOu   = 22, // Background process attempting write
	SigUrg    = 23, // Out-of-band data is available at a socket
	SigXCpu   = 24, // CPU time limit exceeded
	SigXFSz   = 25, // File size limit exceeded
	SigVtAlrm = 26, // Virtual timer expired
	SigProf   = 27, // Profiling timer expired
	SigWinCh  = 28, // When its controlling terminal changes its size
	SigIO     = 29, // A file descriptor is ready to perform input or output
	SigPwr    = 30, // System experiences power failure
	SigSys    = 31, // Bad system call
	SigRTMin  = 32  // First real-time signal
}

pub fn get_signal_type(nb: i32) -> Result<SignalType, ErrNo> {
	match nb {
		_ if nb == SignalType::SigHup as i32 => Ok(SignalType::SigHup),
		_ if nb == SignalType::SigInt as i32 => Ok(SignalType::SigInt),
		_ if nb == SignalType::SigQuit as i32 => Ok(SignalType::SigQuit),
		_ if nb == SignalType::SigIll as i32 => Ok(SignalType::SigIll),
		_ if nb == SignalType::SigTrap as i32 => Ok(SignalType::SigTrap),
		_ if nb == SignalType::SigAbrt as i32 => Ok(SignalType::SigAbrt),
		_ if nb == SignalType::SigBus as i32 => Ok(SignalType::SigBus),
		_ if nb == SignalType::SigFpe as i32 => Ok(SignalType::SigFpe),
		_ if nb == SignalType::SigKill as i32 => Ok(SignalType::SigKill),
		_ if nb == SignalType::SigUsr1 as i32 => Ok(SignalType::SigUsr2),
		_ if nb == SignalType::SigSegv as i32 => Ok(SignalType::SigSegv),
		_ if nb == SignalType::SigUsr2 as i32 => Ok(SignalType::SigUsr1),
		_ if nb == SignalType::SigPipe as i32 => Ok(SignalType::SigPipe),
		_ if nb == SignalType::SigAlrm as i32 => Ok(SignalType::SigAlrm),
		_ if nb == SignalType::SigTerm as i32 => Ok(SignalType::SigTerm),
		_ if nb == SignalType::SigStkFlt as i32 => Ok(SignalType::SigStkFlt),
		_ if nb == SignalType::SigChld as i32 => Ok(SignalType::SigChld),
		_ if nb == SignalType::SigCont as i32 => Ok(SignalType::SigCont),
		_ if nb == SignalType::SigStop as i32 => Ok(SignalType::SigStop),
		_ if nb == SignalType::SigTStp as i32 => Ok(SignalType::SigTStp),
		_ if nb == SignalType::SigTTIn as i32 => Ok(SignalType::SigTTIn),
		_ if nb == SignalType::SigTTOu as i32 => Ok(SignalType::SigTTOu),
		_ if nb == SignalType::SigUrg as i32 => Ok(SignalType::SigUrg),
		_ if nb == SignalType::SigXCpu as i32 => Ok(SignalType::SigXCpu),
		_ if nb == SignalType::SigXFSz as i32 => Ok(SignalType::SigXFSz),
		_ if nb == SignalType::SigVtAlrm as i32 => Ok(SignalType::SigVtAlrm),
		_ if nb == SignalType::SigProf as i32 => Ok(SignalType::SigProf),
		_ if nb == SignalType::SigWinCh as i32 => Ok(SignalType::SigWinCh),
		_ if nb == SignalType::SigIO as i32 => Ok(SignalType::SigIO),
		_ if nb == SignalType::SigPwr as i32 => Ok(SignalType::SigPwr),
		_ if nb == SignalType::SigSys as i32 => Ok(SignalType::SigSys),
		_ if nb == SignalType::SigRTMin as i32 => Ok(SignalType::SigRTMin),
		_ => Err(ErrNo::Inval)
	}
}

#[derive(Copy, Clone, PartialEq)]
pub struct Signal {
	pub sender:  Id,
	pub sigtype: SignalType,
	pub wstatus: i32
}

impl Signal {
	pub const fn new(pid: Id, sigtype: SignalType, wstatus: i32) -> Self {
		Self { sender: pid, sigtype, wstatus }
	}

	pub fn send_to_pid(
		pid: Id,
		sender_pid: Id,
		sigtype: SignalType,
		wstatus: i32
	) -> Result<Id, ErrNo> {
		let binding = Process::search_from_pid(pid)?;
		let mut process = binding.lock();
		Self::send_to_process(&mut process, sender_pid, sigtype, wstatus);
		Ok(pid)
	}

	pub fn send_to_process(
		process: &mut Process,
		pid: Id,
		sigtype: SignalType,
		wstatus: i32
	) {
		let signal = Self::new(pid, sigtype, wstatus);
		process.signals.push(signal);
	}
}
