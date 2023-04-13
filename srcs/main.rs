use crate::vga_buffer::change_color;
use crate::vga_buffer::color::Color;
use crate::wrappers::{cli, hlt, sti};
use crate::{kprint, kprintln, string};

// Temporary sleep function until a proper sleep is implemented and teste
pub fn sleep(microseconds: usize) {
	unsafe {
		let tmp = crate::pic::JIFFIES;
		while crate::pic::JIFFIES < tmp + microseconds {
			sti!();
			hlt!();
			cli!();
		}
	}
}

use sys_macros::Poc;
#[derive(Poc)]
struct poc;

#[sys_macros::poc_insertion]
fn insertion_poc() {
    crate::kprintln!("Insertion poc core");
}

#[no_mangle]
pub extern "C" fn kmain() -> ! {
	crate::user::test_user_page();

    poc::poc();
    insertion_poc();
	kprintln!("Hello World of {}!", 42);

	change_color!(Color::Red, Color::White);
	let workspace_msg = string::String::from(
		"Press Ctrl-2 to navigate to the second workspace"
	);
	kprintln!("{}", workspace_msg);
	change_color!(Color::White, Color::Black);
	kprint!("$> ");
	loop {}
}
