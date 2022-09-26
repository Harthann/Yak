use crate::io;
use crate::vga_buffer::color::Color;

#[cfg(test)]
#[macro_export]
macro_rules! function {
	() => {{
		fn f() {}
		fn type_name_of<T>(_: T) -> &'static str {
			core::any::type_name::<T>()
		}
		let mut name = type_name_of(f);
		name = &name[..name.len() - 3];
		let split = name.split("::");
		split.last().unwrap()
	}}
}

#[cfg(test)]
#[macro_export]
macro_rules! print_fn {
	() => {
		crate::kprint!("{:40}{}", crate::function!(), "");
	}
}


pub fn leaks() -> bool {
	unsafe {
		crate::KTRACKER.allocation != crate::KTRACKER.freed ||
		crate::KTRACKER.allocated_bytes != crate::KTRACKER.freed_bytes
	}
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
	crate::kprintln!("Running {} tests", tests.len());
	for test in tests {
		test.run();
		if leaks() == true {
			crate::memory_state();
			panic!("Memory leaks test failed");
		}
	}
	crate::memory_state();
	io::outb(0xf4, 0x10);
}

#[cfg(test)]
pub trait Testable {
	fn run(&self) -> ();
}

#[cfg(test)]
impl<T> Testable for T
where T: Fn(),
{
	fn run(&self) {
		self();
		crate::change_color!(Color::Green, Color::Black);
		crate::kprintln!("[ok]");
		crate::change_color!(Color::White, Color::Black);
	}
}
