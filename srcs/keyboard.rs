//! Keyboard handler and key mapping

use crate::io;

const PRESSED: usize = 0;
const RELEASED: usize = 1;

pub struct SpecialKeys {
	shift_r:   [u8; 2],
	shift_l:   [u8; 2],
	ctrl:      [u8; 2],
	special_r: [u8; 2], // cmd or windows
	special_l: [u8; 2], // cmd or windows
	alt:       [u8; 2],
	caps:      [u8; 2],
	// Special keys that register using 2 input, first is always 224 and second one is the keycode
	up:        [u8; 2], // arrow up
	down:      [u8; 2], // arrow down
	left:      [u8; 2], // arrow left
	right:     [u8; 2], // arrow right
	insert:    [u8; 2],
	delete:    [u8; 2],
	pgup:      [u8; 2],
	pgdn:      [u8; 2],
	home:      [u8; 2],
	end:       [u8; 2]
}

pub struct Keymap {
	keys:         [char; 58],
	caps_keys:    [char; 58],
	special_keys: SpecialKeys
}

pub const KEYMAP_US: Keymap = Keymap {
	keys:         [
		'\0', '\0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '-', '=',
		'\x08', ' ', 'q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p', '[',
		']', '\n', '\0', 'a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l', ';',
		'\'', '`', '\0', '\\', 'z', 'x', 'c', 'v', 'b', 'n', 'm', ',', '.',
		'/', '\0', '\0', '\0', ' '
	],
	caps_keys:    [
		'\0', '\0', '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '_', '+',
		'\x08', ' ', 'Q', 'W', 'E', 'R', 'T', 'Y', 'U', 'I', 'O', 'P', '{',
		'}', '\n', '\0', 'A', 'S', 'D', 'F', 'G', 'H', 'J', 'K', 'L', ':', '"',
		'~', '\0', '|', 'Z', 'X', 'C', 'V', 'B', 'N', 'M', '<', '>', '?', '\0',
		'\0', '\0', ' '
	],
	special_keys: SpecialKeys {
		shift_r:   [54, 182],
		shift_l:   [42, 170],
		ctrl:      [29, 157],
		special_r: [92, 220],
		special_l: [91, 219],
		alt:       [56, 184],
		caps:      [58, 186],
		// Two input keys, first one always 224
		up:        [72, 200],
		down:      [80, 208],
		left:      [75, 203],
		right:     [77, 205],
		insert:    [82, 210],
		delete:    [83, 211],
		pgup:      [73, 201],
		pgdn:      [81, 209],
		home:      [71, 199],
		end:       [79, 207]
	}
};

pub const KEYMAP_FR: Keymap = Keymap {
	keys:         [
		'\0', '\0', '&', '\0', '"', '\'', '(', '-', '\0', '_', '\0', '\0', ')',
		'=', '\x08', ' ', 'a', 'z', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p',
		'^', '$', '\n', '\0', 'q', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm',
		'\0', '*', '\0', '\\', 'w', 'x', 'c', 'v', 'b', 'n', ',', ';', ':',
		'!', '\0', '\0', '\0', ' '
	],
	caps_keys:    [
		'\0', '\0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '\0',
		'+', '\x08', ' ', 'A', 'Z', 'E', 'R', 'T', 'Y', 'U', 'I', 'O', 'P',
		'"', '\0', '\n', '\0', 'Q', 'S', 'D', 'F', 'G', 'H', 'J', 'K', 'L',
		'M', '%', '\0', '\0', '\0', 'W', 'X', 'C', 'V', 'B', 'N', '?', '.',
		'/', '\0', '\0', '\0', '\0', ' '
	],
	special_keys: SpecialKeys {
		shift_r:   [54, 182],
		shift_l:   [42, 170],
		ctrl:      [29, 157],
		special_r: [92, 220],
		special_l: [91, 219],
		alt:       [56, 184],
		caps:      [58, 186],
		// Two input keys, first one always 224
		up:        [72, 200],
		down:      [80, 208],
		left:      [75, 203],
		right:     [77, 205],
		insert:    [82, 210],
		delete:    [83, 211],
		pgup:      [73, 201],
		pgdn:      [81, 209],
		home:      [71, 199],
		end:       [79, 207]
	}
};

pub static mut KEYMAP: &Keymap = &KEYMAP_US;

#[repr(u8)]
pub enum SpecialKeyFlag {
	ShiftLeft  = 0,
	ShiftRight = 1,
	Ctrl       = 2,
	Opt        = 3,
	CmdLeft    = 4,
	CmdRight   = 5,
	Caps       = 6
}

use core::sync::atomic::{AtomicU8, Ordering};
use crate::utils::flags::{Flags, FlagOp};
static SPECIAL_KEYS: Flags<AtomicU8> = Flags::new(0);

fn is_special(key: u8) -> bool {
	key == 54
		|| key == 182
		|| key == 42
		|| key == 170
		|| key == 29
		|| key == 157
		|| key == 92
		|| key == 220
		|| key == 91
		|| key == 219
		|| key == 56
		|| key == 184
		|| key == 58
		|| key == 186
		|| key == 72
		|| key == 200
		|| key == 80
		|| key == 208
		|| key == 75
		|| key == 203
		|| key == 77
		|| key == 205
		|| key == 82
		|| key == 210
		|| key == 83
		|| key == 211
		|| key == 73
		|| key == 201
		|| key == 81
		|| key == 209
		|| key == 71
		|| key == 199
		|| key == 79
		|| key == 207
}

fn keyboard_to_ascii(key: u8) -> Option<char> {
	if is_special(key) || key >= 58 {
		return None;
	}
	// Get actual key pressed
	let mut charcode: u8 = unsafe { KEYMAP.keys[key as usize] as u8 };

	// If key is alphabetic, check if shift or caps are on
	if charcode >= b'a' && charcode <= b'z' {
        if SPECIAL_KEYS.is_enable(SpecialKeyFlag::ShiftLeft as u8) ||
            SPECIAL_KEYS.is_enable(SpecialKeyFlag::ShiftRight as u8) ||
            SPECIAL_KEYS.is_enable(SpecialKeyFlag::Caps as u8)
		{
			charcode = unsafe { KEYMAP.caps_keys[key as usize] as u8 };
		}
	// If key is not alphabetic, switch to cap_keys only if shift is pressed
	} else if SPECIAL_KEYS.is_enable(SpecialKeyFlag::ShiftLeft as u8) ||
            SPECIAL_KEYS.is_enable(SpecialKeyFlag::ShiftRight as u8)
	{
		{
			charcode = unsafe { KEYMAP.caps_keys[key as usize] as u8 };
		}
	}
	return Some(charcode as char);
}

/// Check if keyboard event is ready
pub fn keyboard_event() -> bool {
	io::inb(0x64) & 1 != 0
}

use crate::cli::{Input, Termcaps};
pub fn handle_event() -> Option<(crate::cli::Input, u8)> {
	let keycode: u8 = io::inb(0x60);

	match keyboard_to_ascii(keycode) {
		Some(ascii) => {
			Some((Input::Ascii(ascii), SPECIAL_KEYS.0.load(Ordering::Relaxed)))
		},
		None => {
			let special_keys: &SpecialKeys = unsafe { &KEYMAP.special_keys };
			match keycode {
				_ if keycode == special_keys.shift_l[PRESSED] => {
					SPECIAL_KEYS.enable(SpecialKeyFlag::ShiftLeft as u8);
					None
				},
				_ if keycode == special_keys.shift_r[PRESSED] => {
					SPECIAL_KEYS.enable(SpecialKeyFlag::ShiftRight as u8);
					None
				},
				_ if keycode == special_keys.ctrl[PRESSED] => {
					SPECIAL_KEYS.enable(SpecialKeyFlag::Ctrl as u8);
					None
				},
				_ if keycode == special_keys.alt[PRESSED] => {
					SPECIAL_KEYS.enable(SpecialKeyFlag::Opt as u8);
					None
				},
				_ if keycode == special_keys.special_l[PRESSED] => {
					SPECIAL_KEYS.enable(SpecialKeyFlag::CmdLeft as u8);
					None
				},
				_ if keycode == special_keys.special_r[PRESSED] => {
					SPECIAL_KEYS.enable(SpecialKeyFlag::CmdRight as u8);
					None
				},
				_ if keycode == special_keys.caps[PRESSED] => {
					SPECIAL_KEYS.disable(SpecialKeyFlag::Caps as u8);
					None
				},

				_ if keycode == special_keys.shift_l[RELEASED] => {
					SPECIAL_KEYS.disable(SpecialKeyFlag::ShiftLeft as u8);
					None
				},
				_ if keycode == special_keys.shift_r[RELEASED] => {
					SPECIAL_KEYS.disable(SpecialKeyFlag::ShiftRight as u8);
					None
				},
				_ if keycode == special_keys.ctrl[RELEASED] => {
					SPECIAL_KEYS.disable(SpecialKeyFlag::Ctrl as u8);
					None
				},
				_ if keycode == special_keys.alt[RELEASED] => {
					SPECIAL_KEYS.disable(SpecialKeyFlag::Opt as u8);
					None
				},
				_ if keycode == special_keys.special_l[RELEASED] => {
					SPECIAL_KEYS.disable(SpecialKeyFlag::CmdLeft as u8);
					None
				},
				_ if keycode == special_keys.special_r[RELEASED] => {
					SPECIAL_KEYS.disable(SpecialKeyFlag::CmdRight as u8);
					None
				},
				224 => {
					let keycode: u8 = io::inb(0x60);
					let special_keys: &SpecialKeys =
						unsafe { &KEYMAP.special_keys };
					match keycode {
						_ if keycode == special_keys.up[PRESSED] => Some((
							Input::Tcaps(Termcaps::ArrowUP),
							SPECIAL_KEYS.0.load(Ordering::Relaxed)
						)),
						_ if keycode == special_keys.down[PRESSED] => Some((
							Input::Tcaps(Termcaps::ArrowDOWN),
							SPECIAL_KEYS.0.load(Ordering::Relaxed)
						)),
						_ if keycode == special_keys.left[PRESSED] => Some((
							Input::Tcaps(Termcaps::ArrowLEFT),
							SPECIAL_KEYS.0.load(Ordering::Relaxed)
						)),
						_ if keycode == special_keys.right[PRESSED] => Some((
							Input::Tcaps(Termcaps::ArrowRIGHT),
							SPECIAL_KEYS.0.load(Ordering::Relaxed)
						)),
						_ => None
					}
				},
				_ => None
			}
		}
	}
}
