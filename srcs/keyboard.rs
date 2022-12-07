//! Keyboard handler and key mapping

use crate::kprint;
use crate::io;
use crate::vga_buffer;
use crate::vga_buffer::NB_SCREEN;

const PRESSED: usize = 0;
const RELEASED: usize = 1;

pub struct SpecialKeys {
	shift_r:	[u8; 2],
	shift_l:	[u8; 2],
	ctrl:		[u8; 2],
	special_r:	[u8; 2], /* cmd or windows */
	special_l:	[u8; 2], /* cmd or windows */
	alt:		[u8; 2],
	caps:		[u8; 2]
}

pub struct Keymap {
	keys:			[char; 58],
	caps_keys:		[char; 58],
	special_keys:	SpecialKeys
}

pub const KEYMAP_US: Keymap = Keymap {
	keys: ['\0', '\0', '1', '2', '3', '4', '5', '6', '7',
'8', '9', '0', '-', '=', '\x08', ' ', 'q', 'w', 'e', 'r', 't', 'y',
'u', 'i', 'o', 'p', '[', ']', '\n', '\0', 'a', 's', 'd', 'f', 'g',
'h', 'j', 'k', 'l', ';', '\'', '`', '\0', '\\', 'z', 'x', 'c', 'v',
'b', 'n', 'm', ',', '.', '/', '\0', '\0', '\0', ' '],
	caps_keys: ['\0', '\0', '!', '@', '#', '$', '%', '^', '&',
'*', '(', ')', '_', '+', '\x08', ' ', 'Q', 'W', 'E', 'R', 'T', 'Y',
'U', 'I', 'O', 'P', '{', '}', '\n', '\0', 'A', 'S', 'D', 'F', 'G',
'H', 'J', 'K', 'L', ':', '"', '~', '\0', '|', 'Z', 'X', 'C', 'V',
'B', 'N', 'M', '<', '>', '?', '\0', '\0', '\0', ' '],
	special_keys: SpecialKeys {
		shift_r:	[54, 182],
		shift_l:	[42, 170],
		ctrl:		[29, 157],
		special_r:	[92, 220],
		special_l:	[91, 219],
		alt:		[56, 184],
		caps:		[58, 186]
	}
};

pub const KEYMAP_FR: Keymap = Keymap {
	keys: ['\0', '\0', '&', '\0', '"', '\'', '(', '-', '\0',
'_', '\0', '\0', ')', '=', '\x08', ' ', 'a', 'z', 'e', 'r', 't', 'y',
'u', 'i', 'o', 'p', '^', '$', '\n', '\0', 'q', 's', 'd', 'f', 'g',
'h', 'j', 'k', 'l', 'm', '\0', '*', '\0', '\\', 'w', 'x', 'c', 'v',
'b', 'n', ',', ';', ':', '!', '\0', '\0', '\0', ' '],
	caps_keys: ['\0', '\0', '1', '2', '3', '4', '5', '6', '7',
'8', '9', '0', '\0', '+', '\x08', ' ', 'A', 'Z', 'E', 'R', 'T', 'Y',
'U', 'I', 'O', 'P', '"', '\0', '\n', '\0', 'Q', 'S', 'D', 'F', 'G',
'H', 'J', 'K', 'L', 'M', '%', '\0', '\0', '\0', 'W', 'X', 'C', 'V',
'B', 'N', '?', '.', '/', '\0', '\0', '\0', '\0', ' '],
	special_keys: SpecialKeys {
		shift_r:	[54, 182],
		shift_l:	[42, 170],
		ctrl:		[29, 157],
		special_r:	[92, 220],
		special_l:	[91, 219],
		alt:		[56, 184],
		caps:		[58, 186]
	}
};

pub static mut KEYMAP: &Keymap = &KEYMAP_US;

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
	let mut charcode: u8 = unsafe{KEYMAP.keys[key as usize] as u8};

	if charcode >= b'a' && charcode <= b'z' {
		if (getflag!(SpecialKeyFlag::ShiftLeft) || getflag!(SpecialKeyFlag::ShiftRight)) ^ getflag!(SpecialKeyFlag::Caps) {
				charcode = unsafe{KEYMAP.caps_keys[key as usize] as u8};
		}
	} else if getflag!(SpecialKeyFlag::ShiftLeft) || getflag!(SpecialKeyFlag::ShiftRight) {
		{charcode = unsafe{KEYMAP.caps_keys[key as usize] as u8};}
	}
	return charcode as char;
}

pub fn keyboard_event() -> bool {
	io::inb(0x64) & 1 != 0
}

pub fn handle_event() -> char {
	let keycode: u8 = io::inb(0x60);
	
	let charcode = keyboard_to_ascii(keycode);
	if charcode >= '1' && charcode <= ('0' as u8 + NB_SCREEN as u8) as char && getflag!(SpecialKeyFlag::Ctrl) {
		unsafe{vga_buffer::WRITER.change_screen((charcode as usize - '0' as usize - 1) as usize);}
		return '\0';
	}
	else if charcode != '\0' {
		kprint!("{}", charcode);
	}
	else {
		let special_keys: &SpecialKeys = unsafe{&KEYMAP.special_keys};
        match keycode {
            _ if keycode == special_keys.shift_l[PRESSED]       => setflag!(SpecialKeyFlag::ShiftLeft),
            _ if keycode == special_keys.shift_r[PRESSED]       => setflag!(SpecialKeyFlag::ShiftRight),
			_ if keycode == special_keys.ctrl[PRESSED]          => setflag!(SpecialKeyFlag::Ctrl),
			_ if keycode == special_keys.alt[PRESSED]           => setflag!(SpecialKeyFlag::Opt),
			_ if keycode == special_keys.special_l[PRESSED]     => setflag!(SpecialKeyFlag::CmdLeft),
			_ if keycode == special_keys.special_r[PRESSED]     => setflag!(SpecialKeyFlag::CmdRight),
			_ if keycode == special_keys.caps[PRESSED]          => unsetflag!(SpecialKeyFlag::Caps),

			_ if keycode == special_keys.shift_l[RELEASED]      => unsetflag!(SpecialKeyFlag::ShiftLeft),
			_ if keycode == special_keys.shift_r[RELEASED]      => unsetflag!(SpecialKeyFlag::ShiftRight),
			_ if keycode == special_keys.ctrl[RELEASED]         => unsetflag!(SpecialKeyFlag::Ctrl),
			_ if keycode == special_keys.alt[RELEASED]          => unsetflag!(SpecialKeyFlag::Opt),
			_ if keycode == special_keys.special_l[RELEASED]    => unsetflag!(SpecialKeyFlag::CmdLeft),
			_ if keycode == special_keys.special_r[RELEASED]    => unsetflag!(SpecialKeyFlag::CmdRight),

            _ => return '\0',
        };
	}
	return charcode;
}
