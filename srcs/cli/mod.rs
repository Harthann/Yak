use crate::alloc::boxed::Box;
use crate::proc::signal::SignalType;
use crate::spin::KMutex;
use crate::syscalls::signal::sys_signal;
use crate::utils::queue::Queue;
use crate::vga_buffer::WRITER;

mod input;
pub use input::{Input, Termcaps};

mod commands;
pub use commands::Command;

use crate::keyboard::SpecialKeyFlag;
use crate::vga_buffer::Screen;

pub const NB_SCREEN: u8 = 3;
pub static INPUT_BUFFER: KMutex<Option<Queue<(Input, u8)>>> = KMutex::new(None);

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
				self.new_char(value, spkey);
			},
			Input::Tcaps(value) => {
				self.new_tcaps(value, spkey);
			},
			_ => {}
		}
	}

	fn new_char(&mut self, value: char, spkey: u8) {
		if spkey & (1 << SpecialKeyFlag::Ctrl as u8) != 0 {
			if value.is_ascii_digit() {
				self.change_screen(value.to_digit(10).unwrap() as i8 - 1);
			}
		} else {
			self.screens[self.current_screen as usize]
				.get_command()
				.handle(value);
		}
	}

	fn new_tcaps(&mut self, event: Termcaps, _spkey: u8) {
		match event {
			Termcaps::ArrowLEFT => {
				if self.screens[self.current_screen as usize]
					.get_command()
					.index != 0
				{
					self.screens[self.current_screen as usize]
						.get_command()
						.index -= 1;
					WRITER.lock().move_cursor(-1);
				}
			},
			Termcaps::ArrowRIGHT => {
				if self.screens[self.current_screen as usize]
					.get_command()
					.index != self.screens[self.current_screen as usize]
					.get_command()
					.len()
				{
					self.screens[self.current_screen as usize]
						.get_command()
						.index += 1;
					WRITER.lock().move_cursor(1);
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

pub static mut LOCK_CMD: bool = false;

// signal handler for unlocking cmd after init
fn unlock_cmd(_no: i32) {
	unsafe { LOCK_CMD = false };
}

pub fn cli() {
	let mut emulator: Box<TermEmu> = Box::default();
	*INPUT_BUFFER.lock() = Some(Queue::new());

	unsafe { LOCK_CMD = false };
	sys_signal(SignalType::SIGHUP as i32, unlock_cmd);
	loop {
		if INPUT_BUFFER.lock().as_ref().unwrap().is_empty() {
			unsafe {
				crate::wrappers::hlt!();
			}
		} else {
			let event = INPUT_BUFFER.lock().as_mut().unwrap().pop();
			emulator.handle_event(event.0, event.1);
		}
	}
}
