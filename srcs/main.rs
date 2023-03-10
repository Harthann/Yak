use crate::string;
use crate::vga_buffer::{change_color, color::Color};
use crate::{kprint, kprintln};

#[no_mangle]
pub extern "C" fn kmain() -> ! {
	crate::user::test_user_page();

	kprintln!("Hello World of {}!", 42);

	change_color!(Color::Red, Color::White);
	let workspace_msg = string::String::from("Press Ctrl-2 to navigate to the second workspace");
	kprintln!("{}", workspace_msg);
	change_color!(Color::White, Color::Black);
	kprint!("$> ");
	loop {
    }
}
