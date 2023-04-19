use crate::spin::Mutex;
use crate::utils::queue::Queue;
use crate::vga_buffer::WRITER;

mod input;
pub use input::{Input, Termcaps};

mod commands;
pub use commands::Command;

mod io;

use crate::keyboard::SpecialKeyFlag;
use crate::vga_buffer::Screen;

pub const NB_SCREEN: u8 = 3;
pub static INPUT_BUFFER: Mutex<Option<Queue<u8>>, true> =
	Mutex::new(None);

#[derive(Clone, Default)]
pub struct TermEmu {
	screens:        [Screen; NB_SCREEN as usize],
	current_screen: i8
}

const SCREEN: Screen = Screen::new();
impl TermEmu {
	pub const fn new() -> Self {
		TermEmu {
			screens:        [SCREEN; NB_SCREEN as usize],
			current_screen: 0
		}
	}

	pub fn handle_event(&mut self, event: Input, spkey: u8) {
		match event {
			Input::Ascii(value) => {
				if spkey & (1 << SpecialKeyFlag::Ctrl as u8) != 0 {
					if value.is_ascii_digit() {
						self.change_screen(
							value.to_digit(10).unwrap() as i8 - 1
						);
					}
				} else {
					self.screens[self.current_screen as usize]
						.get_command()
						.handle(value);
				}
			},
			_ => {}
		}
	}

	pub fn change_screen(&mut self, id: i8) {
		if id >= 0 && id < NB_SCREEN as i8 {
			let mut guard = WRITER.lock();
			guard.save(&mut self.screens[self.current_screen as usize]);
			guard.render(&mut self.screens[id as usize]);
			self.current_screen = id;
		}
	}
}

use crate::boxed::Box;
use crate::fs;
pub fn cli() {
	let mut emulator: Box<TermEmu> = Box::default();
	*INPUT_BUFFER.lock() = Some(Queue::new());
    let stdin: io::Stdin = io::Stdin::default();
    crate::fs::create_from_raw("tty0/stdin", stdin).expect("Failed to create file");
    let stdin  = crate::fs::open("tty0/stdin").expect("Failed to open stdin");

    let mut event: [u8; 1] = [0];
	loop {
        fs::read(stdin, &mut event, 1).expect("Failed to read stdin");
        crate::kprint!("{}", event[0] as char);
	}
}
