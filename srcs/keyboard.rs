use crate::print;
use crate::io;
use crate::vga_buffer;

static KEYBOARD: [char; 58] = ['\0', '\0', '1', '2', '3', '4', '5', '6', '7',
'8', '9', '0', '-', '=', '\x08', ' ', 'q', 'w', 'e', 'r', 't', 'y',
'u', 'i', 'o', 'p', '[', ']', '\n', '\0', 'a', 's', 'd', 'f', 'g',
'h', 'j', 'k', 'l', ';', '\'', '`', '\0', '\\', 'z', 'x', 'c', 'v',
'b', 'n', 'm', ',', '.', '/', '\0', '\0', '\0', ' '];

static KEYBOARD_CAPS: [char; 58] = ['\0', '\0', '!', '@', '#', '$', '%', '^', '&',
'*', '(', ')', '_', '+', '\x08', ' ', 'Q', 'W', 'E', 'R', 'T', 'Y',
'U', 'I', 'O', 'P', '{', '}', '\n', '\0', 'A', 'S', 'D', 'F', 'G',
'H', 'J', 'K', 'L', ':', '"', '~', '\0', '|', 'Z', 'X', 'C', 'V',
'B', 'N', 'M', '<', '>', '?', '\0', '\0', '\0', ' '];

const SHIFTRPRESSED:	u8	= 54;
const SHIFTLPRESSED:	u8	= 42;
const SHIFTRRELEASED:	u8	= 182;
const SHIFTLRELEASED:	u8	= 170;
const CTRLPRESSED:		u8	= 29;
const CTRLRELEASED:		u8	= 157;
const CMDRPRESSED:		u8	= 92;
const CMDLPRESSED:		u8	= 91;
const CMDRRELEASED:		u8	= 220;
const CMDLRELEASED:		u8	= 219;
const OPTPRESSED:		u8	= 56;
const OPTRELEASED:		u8	= 184;
const CAPSPRESSED:		u8	= 58;
// const CAPSRELEASED:		u8	= 186; // -> useless

#[repr(u8)]
enum SpecialKeyFlag {
	ShiftLeft	=	0,
	ShiftRight	=	1,
	Ctrl		=	2,
	Opt			=	3,
	CmdLeft		=	4,
	CmdRight	=	5,
	Caps		=	6
}

macro_rules! setflag {
	($a:expr) => {
		unsafe{ SPECIAL_KEYS |= (1 << $a as u8)}
	}
}

macro_rules! getflag {
	($a:expr) => {
		unsafe{ SPECIAL_KEYS & (1 << $a as u8) != 0}
	}
}

macro_rules! unsetflag {
	($a:expr) => {
		unsafe {SPECIAL_KEYS ^= (1 << $a as u8)}
	}
}

static mut SPECIAL_KEYS: u8 = 0;

fn keyboard_to_ascii(key: u8) -> char {
	if key >= 58
		{return '\0'; }
	let mut charcode: u8 = KEYBOARD[key as usize] as u8;

	if charcode >= b'a' && charcode <= b'z' {
		if (getflag!(SpecialKeyFlag::ShiftLeft) || getflag!(SpecialKeyFlag::ShiftRight)) ^ getflag!(SpecialKeyFlag::Caps) {
				charcode = KEYBOARD_CAPS[key as usize] as u8;
		}
	} else if getflag!(SpecialKeyFlag::ShiftLeft) || getflag!(SpecialKeyFlag::ShiftRight) {
		{charcode = KEYBOARD_CAPS[key as usize] as u8;}
	}
	return charcode as char;
}

pub fn keyboard_event() -> bool {
	io::inb(0x64) & 1 != 0
}

pub fn handle_event() {
	let mut keycode: u8 = io::inb(0x60);
	
	let charcode = keyboard_to_ascii(keycode);
	if (charcode == '1' || charcode == '2') && getflag!(SpecialKeyFlag::Ctrl) {
		vga_buffer::change_screen((charcode as usize - '0' as usize - 1) as usize)
	}
	else if charcode != '\0' {
		print!("{}", charcode);
	}
	else {
		if keycode == 224
			{ keycode = io::inb(0x60);}
		match keycode {
		/*	Key mapping for special keys press (This will set a flag) */
			SHIFTLPRESSED		=> setflag!(SpecialKeyFlag::ShiftLeft),
			SHIFTRPRESSED		=> setflag!(SpecialKeyFlag::ShiftRight),
			CTRLPRESSED			=> setflag!(SpecialKeyFlag::Ctrl),
			OPTPRESSED			=> setflag!(SpecialKeyFlag::Opt),
			CMDLPRESSED			=> setflag!(SpecialKeyFlag::CmdLeft),
			CMDRPRESSED			=> setflag!(SpecialKeyFlag::CmdRight),
			CAPSPRESSED			=> unsetflag!(SpecialKeyFlag::Caps),

		/*	Key mapping for special keysased (This will ununset a flag) */
			SHIFTLRELEASED		=> unsetflag!(SpecialKeyFlag::ShiftLeft),
			SHIFTRRELEASED		=> unsetflag!(SpecialKeyFlag::ShiftRight),
			CTRLRELEASED		=> unsetflag!(SpecialKeyFlag::Ctrl),
			OPTRELEASED			=> unsetflag!(SpecialKeyFlag::Opt),
			CMDLRELEASED		=> unsetflag!(SpecialKeyFlag::CmdLeft),
			CMDRRELEASED		=> unsetflag!(SpecialKeyFlag::CmdRight),
			_					=> return ,
		};
	}
}
