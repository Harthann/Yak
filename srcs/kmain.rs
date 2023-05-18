use crate::vga_buffer::change_color;
use crate::vga_buffer::color::Color;
use crate::wrappers::{cli, hlt, sti};
use crate::{kprint, kprintln, string};

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
use crate::proc::process::MASTER_PROCESS;

use crate::syscalls::mmap;
use crate::memory;
fn test_mmap() {
    let _mz =  mmap::sys_mmap(0x0, 0x1000, 0, memory::WRITABLE, 0, 0).expect("Couldn't map memory");
    let _mz2 = mmap::sys_mmap(0x0, 0x1000, 0, memory::WRITABLE, 0, 0).expect("Couldn't map memory");
    let _mz3 = mmap::sys_mmap(0x0, 0x1000, 0, 0, 0, 0).expect("Couldn't map memory");
    loop {}
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
    unsafe { crate::dprintln!("{}", MASTER_PROCESS.stack); }
    unsafe { crate::dprintln!("{}", MASTER_PROCESS.heap); }
    unsafe { crate::dprintln!("{}", MASTER_PROCESS.kernel_stack); }
    unsafe { crate::exec_fn!(test_mmap); }
	kprintln!("{}", workspace_msg);
	change_color!(Color::White, Color::Black);
	kprint!("$> ");
	loop {}
}
