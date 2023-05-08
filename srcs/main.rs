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

mod poc {
	use sys_macros::Poc;
	#[derive(Poc)]
	struct Poc;

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
		Poc::poc();
		crate::kprintln!("Test attribute macro:");
		insertion_poc();
		crate::kprintln!("Test attribute macro with argument:");
		sys_mmap();
		crate::kprintln!("Test basic proc macro:");
		proc_macro_poc();
	}
}

fn add_lol(nb1: i32, nb2: u32, string: &str) -> i32 {
	crate::kprintln!("in add_lol: {}", string.len() as i32 + nb1 + nb2 as i32);
	string.len() as i32 + nb1 + nb2 as i32
}

fn test_exec_fn_diff_args() {
	let string = "salut";
	unsafe {
		let pid = crate::exec_fn!(add_lol, 8, 9, string);
		crate::kprintln!("pid: {}", pid);
		let mut wstatus: i32 = 0;
		let ret = crate::syscalls::exit::sys_waitpid(pid, &mut wstatus, 0);
		crate::kprintln!("ret: {}", ret);
		crate::kprintln!("wexited: {}", crate::syscalls::exit::__WIFEXITED!(wstatus));
		crate::kprintln!("wexitstatus: {}", crate::syscalls::exit::__WEXITSTATUS!(wstatus));
	}
}

#[no_mangle]
pub extern "C" fn kmain() -> ! {
	crate::user::test_user_page();

	test_exec_fn_diff_args();

	poc::test_macros();
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
