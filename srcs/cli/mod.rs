use crate::utils::queue::Queue;
use crate::spin::Mutex;
use crate::vga_buffer::WRITER;

mod input;
pub use input::{Input, Termcaps};

mod commands;
pub use commands::Command;

use crate::vga_buffer::Screen;
use crate::keyboard::{SpecialKeyFlag};

pub const NB_SCREEN: u8 = 3;
pub static INPUT_BUFFER: Mutex<Queue<(Input, u8)>, true> = Mutex::new(Queue::new());

#[derive(Clone)]
pub struct TermEmu {
    screens:  [Screen; NB_SCREEN as usize],
    current_screen: u8
}

const SCREEN: Screen = Screen::new();
impl TermEmu {
     pub const fn new() -> Self {
        TermEmu {
            screens:         [SCREEN; NB_SCREEN as usize],
            current_screen: 0
        }
    }

    pub fn handle_event(&mut self, event: Input, spkey: u8) {
        match event {
            Input::Ascii(value) => {
                if spkey & (1 << SpecialKeyFlag::Ctrl as u8) != 0 {
                    if value.is_ascii_digit() {
                        self.change_screen(value.to_digit(10).unwrap() as u8 - 1);
                    }
                } else {
                    self.screens[self.current_screen as usize].get_command().handle(value);
                }
            },
            _ => {}

        }
    }

    pub fn change_screen(&mut self, id: u8) {
        if id >= 0 && id < NB_SCREEN {
            let mut guard = WRITER.lock();
            guard.save(&mut self.screens[self.current_screen as usize]);
            guard.render(&mut self.screens[id as usize]);
            self.current_screen = id;
        }
    }
}

pub fn cli() {
    let mut emulator = TermEmu::new();
    loop {
        if INPUT_BUFFER.lock().is_empty() {
            unsafe { crate::wrappers::hlt!(); }
        } else {
            let event = INPUT_BUFFER.lock().pop();
            emulator.handle_event(event.0, event.1);
        }
    }
}
