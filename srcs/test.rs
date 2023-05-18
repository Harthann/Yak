use crate::vga_buffer::color::Color;
use crate::{io, vga_buffer, KTRACKER};

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
	}};
}

#[cfg(test)]
#[macro_export]
macro_rules! print_fn {
	() => {
		crate::kprint!("{:40}{}", crate::function!(), "");
	};
}

pub fn leaks() -> bool {
	unsafe {
		KTRACKER.allocation != KTRACKER.freed
			|| KTRACKER.allocated_bytes != KTRACKER.freed_bytes
	}
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
	use crate::memory::paging::bitmap::physmap_as_mut;
	crate::kprintln!("Running {} tests", tests.len());
	let used_pages = physmap_as_mut().used;
	crate::kprintln!("Kernel as mapped {} pages", used_pages);
	for test in tests {
		let pages_before_test = physmap_as_mut().used;
		test.run();
		let pages_after_test = physmap_as_mut().used;
		if leaks() == true {
			crate::memory_state();
			panic!("Memory leaks test failed");
		}
		if pages_before_test != pages_after_test {
			crate::kprintln!(
				"Before {} pages\n After {} pages",
				pages_before_test,
				pages_after_test
			);
		}
	}
	crate::kprintln!(
		"Kernel uses {} more pages",
		physmap_as_mut().used - used_pages
	);
	crate::memory_state();
	io::outb(0xf4, 0x10);
}

#[cfg(test)]
pub trait Testable {
	fn run(&self) -> ();
}

#[cfg(test)]
impl<T> Testable for T
where
	T: Fn()
{
	fn run(&self) {
		self();
		vga_buffer::change_color!(Color::Green, Color::Black);
		crate::kprintln!("[ok]");
		vga_buffer::change_color!(Color::White, Color::Black);
	}
}
