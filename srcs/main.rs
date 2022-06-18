#![feature(const_mut_refs)]
#![feature(lang_items)]
#![no_std]

//use core::arch::asm;
mod vga_buffer;
mod io;
mod keyboard;

extern "C" {
	fn stack_bottom();
	fn stack_top();
}

use vga_buffer::color::Color;

#[lang = "eh_personality"] #[no_mangle] pub extern fn eh_personality() {}

#[no_mangle]
pub extern fn rust_main() -> ! {
	println!("Hello World of {}!", 42);
	change_color!(Color::Red, Color::White);
	println!("Press Ctrl-{} to navigate to the second workspace", '2');
	change_color!(Color::White, Color::Black);
	let stack_size = stack_top as usize - stack_bottom as usize;
	let offset = unsafe{(stack_bottom as *const u8).offset((stack_size - 256) as isize)};
	hexdump!(offset, 256);
/*
	let mut x: u32 = 4;
	unsafe {
		asm!(
				"mov {tmp}, {x}",
				"shl {tmp}, 1",
				"shl {x}, 2",
				"add {x}, {tmp}",
				x = inout(reg) x,
				tmp = out(reg) _,
			);
	}
	assert!(x == 6);
*/

	let mut cmd: [char; 256] = ['\0'; 256];
	let mut i = 0;
	print!("$> ");
	loop {
		if keyboard::keyboard_event() {
			let charcode = keyboard::handle_event();
			if charcode >= ' ' && charcode <= '~' {
				if i < 256 {
					cmd[i] = charcode;
				}
				i += 1;
//				cmd = concat!(cmd, charcode);
			}
			else if charcode == '\n' {
				let known_cmd = ["reboot", "halt", "hexdump"];
				let mut j = 0;
				while j < known_cmd.len() {
					let len = known_cmd[j].chars().count();
					if (cmd[len] == '\0' || cmd[len] == ' ') && known_cmd[j].chars().zip(cmd.iter()).position(|(a, b)| a != *b) == None {
						println!("[!!!]");
						todo!();
						break ;
					}
					j += 1;
				}
				if j == known_cmd.len() {
					println!("Unknown command");
				}
				print!("$> ");
				i = 0;
				cmd = ['\0'; 256];
			}
		}
	}
}

