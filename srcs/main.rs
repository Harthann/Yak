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
use crate::syscalls::exit::sys_waitpid;

mod poc {
	use sys_macros::Poc;
	#[derive(Poc)]
	struct poc;

	#[sys_macros::poc_insertion]
	fn insertion_poc() {
		crate::kprintln!("Insertion poc core");
	}

	#[sys_macros::poc_insertion(mmap)]
	fn sys_mmap() {
		crate::kprintln!("Mmap syscall");
	}

	sys_macros::proc_macro_poc!();

	pub fn test_macros() {
		crate::kprintln!("Test drive macro:");
		poc::poc();
		crate::kprintln!("Test attribute macro:");
		insertion_poc();
		crate::kprintln!("Test attribute macro with argument:");
		sys_mmap();
		crate::kprintln!("Test basic proc macro:");
		proc_macro_poc();
	}
}

#[no_mangle]
pub extern "C" fn kmain() -> ! {
	crate::user::test_user_page();

	poc::test_macros();
	kprintln!("Hello World of {}!", 42);

	change_color!(Color::Red, Color::White);
	let workspace_msg = string::String::from(
		"Press Ctrl-2 to navigate to the second workspace"
	);
	kprintln!("{}", workspace_msg);
	change_color!(Color::White, Color::Black);
	loop {
	    kprint!("$> ");
        let pid = unsafe {
            crate::exec_fn!(crate::cli::cli)
        };
        unsafe { crate::dprintln!("Term pid: {:?}", pid) };
        let mut status = 0;
        sys_waitpid(pid, &mut status, 0);
        unsafe { crate::dprintln!("Term has been killed") };
    }
}
