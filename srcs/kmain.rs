use crate::syscalls::exit::sys_waitpid;
use crate::vga_buffer::change_color;
use crate::vga_buffer::color::Color;
use crate::{kprint, kprintln, string};

mod poc {
	use sys_macros::Poc;
	#[derive(Poc)]
	struct Poc;

	#[sys_macros::poc_insertion]
	pub fn insertion_poc() {
		crate::kprintln!("Insertion poc core");
	}

	#[sys_macros::poc_insertion(mmap)]
	fn sys_mmap() {
		crate::kprintln!("Mmap syscall");
	}

	sys_macros::proc_macro_poc!();

	pub fn test_macros() {
		crate::kprintln!("Test drive macro:");
		Poc::poc();
		crate::kprintln!("Test attribute macro:");
		insertion_poc();
		crate::kprintln!("Test attribute macro with argument:");
		sys_mmap();
		crate::kprintln!("Test basic proc macro:");
		proc_macro_poc();
	}
}

use crate::fs::ext2;
use crate::cli::DISKNO;

#[no_mangle]
pub extern "C" fn kmain() -> ! {
	for i in 0..4 {
		if ext2::is_ext2(i) {
			*DISKNO.lock() = i as i8;
			crate::kprintln!("Found ext2 filesystem on disk {}.", i);
			break;
		}
	}
	if *DISKNO.lock() == -1 {
		todo!("No ext2 disk found.");
	}
	// 	poc::insertion_poc();
	kprintln!("Hello World of {}!", 42);
	change_color!(Color::Red, Color::White);
	let workspace_msg = string::String::from(
		"Press Ctrl-2 to navigate to the second workspace"
	);
	kprintln!("{}", workspace_msg);
	change_color!(Color::White, Color::Black);

	kprint!("$> ");
	let mut pid = unsafe { crate::exec_fn!(crate::cli::cli) };
	loop {
		// Auto-remove all zombies on pid 0 and relaunch cli if killed
		let mut status = 0;
		let ret = sys_waitpid(-1, &mut status, 0);
		if ret == pid {
			crate::dprintln!("Term has been killed");
			kprint!("$> ");
			pid = unsafe { crate::exec_fn!(crate::cli::cli) };
		}
	}
}
