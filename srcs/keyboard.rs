use crate::print;
use crate::io;

static KEYBOARD: [char; 58] = ['\0', '\0', '1', '2', '3', '4', '5', '6', '7',
'8', '9', '0', '-', '=', '\x08', '\0', 'q', 'w', 'e', 'r', 't', 'y',
'u', 'i', 'o', 'p', '[', ']', '\n', '\0', 'a', 's', 'd', 'f', 'g',
'h', 'j', 'k', 'l', ';', '\'', '`', '\0', '\\', 'z', 'x', 'c', 'v',
'b', 'n', 'm', ',', '.', '/', '\0', '\0', '\0', ' '];

const ShiftRPressed:	u8	= 54;
const ShiftLPressed:	u8	= 42;
const ShiftRReleased:	u8	= 182;
const ShiftLReleased:	u8	= 170;
const CtrlLPressed:		u8	= 0x0;
const CtrlRPressed:		u8	= 0x1;
const CtrlLReleased:	u8	= 0x2;
const CtrlRReleased:	u8	= 0x3;
const CmdRPressed:		u8	= 92;
const CmdLPressed:		u8	= 91;
const CmdRReleased:		u8	= 220;
const CmdLReleased:		u8	= 219;
const OptLPressed:		u8	= 0x4;
const OptRPressed:		u8	= 0x5;
const OptLReleased:		u8	= 0x6;
const OptRReleased:		u8	= 0x7;
const CapsPressed:		u8	= 58;
const CapsReleased:		u8	= 186;

#[repr(u8)]
enum SpecialKeyFlag {
	ShiftLeft	=	0,
	ShiftRight	=	1,
	CtrlLeft	=	2,
	CtrlRight	=	3,
	OptLeft		=	4,
	OptRight	=	5,
	CmdLeft		=	6,
	CmdRight	=	7,
	Caps		=	8
}

macro_rules! setflag {
	($a:expr) => {
		unsafe{ SPECIAL_KEYS |= (1 << $a as u16)}
	}
}


macro_rules! getflag {
	($a:expr) => {
		unsafe{ SPECIAL_KEYS & (1 << $a as u16) != 0}
	}
}

macro_rules! unsetflag {
	($a:expr) => {
		unsafe {SPECIAL_KEYS ^= (1 << $a as u16)}
	}
}

static mut SPECIAL_KEYS: u16 = 0;

fn keyboard_to_ascii(key: u8) -> char {
	if key >= 58
		{return '\0'; }
	let mut charcode: u8 = KEYBOARD[key as usize] as u8;
	
	if (charcode >= b'a' && charcode <= b'z' &&
		(getflag!(SpecialKeyFlag::ShiftLeft) || getflag!(SpecialKeyFlag::ShiftRight) || (getflag!(SpecialKeyFlag::Caps))))
		{ charcode = charcode - b'a' + b'A';}
	return charcode as char;
}

pub fn keyboard_event() -> bool {
	io::inb(0x64) & 1 != 0
}

pub fn handle_event() {
	let mut keycode: u8 = io::inb(0x60);
	
	let mut charcode = keyboard_to_ascii(keycode);
	if charcode != '\0' {
		print!("{}", charcode);
	}
	else {
		if keycode == 224
			{ keycode = io::inb(0x60);}
		match keycode {
		/*	Key mapping for special keys press (This will set a flag) */
			ShiftLPressed		=> setflag!(SpecialKeyFlag::ShiftLeft),
			ShiftRPressed		=> setflag!(SpecialKeyFlag::ShiftRight),
			CtrlLPressed		=> setflag!(SpecialKeyFlag::CtrlLeft),
			CtrlRPressed		=> setflag!(SpecialKeyFlag::CtrlRight),
			OptLPressed		=> setflag!(SpecialKeyFlag::OptLeft),
			OptRPressed		=> setflag!(SpecialKeyFlag::OptRight),
			CmdLPressed		=> setflag!(SpecialKeyFlag::CmdLeft),
			CmdRPressed		=> setflag!(SpecialKeyFlag::CmdRight),
			CapsPressed		=> setflag!(SpecialKeyFlag::Caps),

		/*	Key mapping for special keysased (This will ununset a flag) */
			ShiftLReleased		=> unsetflag!(SpecialKeyFlag::ShiftLeft),
			ShiftRReleased		=> unsetflag!(SpecialKeyFlag::ShiftRight),
			CtrlLReleased		=> unsetflag!(SpecialKeyFlag::CtrlLeft),
			CtrlRReleased		=> unsetflag!(SpecialKeyFlag::CtrlRight),
			OptLReleased		=> unsetflag!(SpecialKeyFlag::OptLeft),
			OptRReleased		=> unsetflag!(SpecialKeyFlag::OptRight),
			CmdLReleased		=> unsetflag!(SpecialKeyFlag::CmdLeft),
			CmdRReleased		=> unsetflag!(SpecialKeyFlag::CmdRight),
			CapsReleased		=> unsetflag!(SpecialKeyFlag::Caps),
			_					=> return,
		};
	}
}
