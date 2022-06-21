#![feature(const_mut_refs)]
#![feature(lang_items)]
#![no_std]

//use core::arch::asm;
mod io;
mod keyboard;
mod vga_buffer;
mod cli;

use core::arch::asm;
use core::slice;

extern "C" {
	fn stack_bottom();
	fn stack_top();
}

use vga_buffer::color::Color;
use cli::Command;

#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn eh_personality() {}

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
	println!("Hello World of {}!", 42);
	change_color!(Color::Red, Color::White);
	println!("Press Ctrl-{} to navigate to the second workspace", '2');
	change_color!(Color::White, Color::Black);
	let stack_size = stack_top as usize - stack_bottom as usize;
	let offset = unsafe { (stack_bottom as *const u8).offset((stack_size - 256) as isize) };

	let mut command: Command = Command::new();
	print!("$> ");
	loop {
		if keyboard::keyboard_event() {
			let charcode = keyboard::handle_event();
			if charcode >= ' ' && charcode <= '~' {
				if command.append(charcode).is_err() {
					println!("Can't handle longer command, clearing buffer");
					command.clear();
				}
			} else if charcode == '\x08' {
				command.pop_back();
			} else if charcode == '\n' {
				match command.is_known() {
					Some(x) => cli::COMMANDS[x](&command),
					_		=> println!("Unknown command"),
				}
				command.clear();
				print!("$> ");
			}
		}
	}
}
