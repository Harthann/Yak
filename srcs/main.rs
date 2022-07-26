#![feature(const_mut_refs)]
#![feature(rustc_attrs)]
#![feature(box_syntax)]
#![feature(ptr_internals)]
#![feature(fundamental)]
#![feature(lang_items)]
#![no_std]
#![allow(dead_code)]
#![no_main]


/*  Custom test framwork    */
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

#[cfg(test)]
mod test;

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
	kprintln!("Running {} tests", tests.len());
	for test in tests {
		test.run();
	}
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
		kprint!("{}... ", core::any::type_name::<T>());
		self();
		change_color!(Color::Green, Color::Black);
		kprintln!("[ok]");
		change_color!(Color::White, Color::Black);
	}
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn eh_personality() {}


const GLOBAL_ALIGN: usize = 8;

/*  Modules import  */
mod io;
mod keyboard;
mod vga_buffer;
mod gdt;
mod cli;
mod paging;
mod interrupts;
mod kmemory;
mod multiboot;
mod allocator;
mod string;
mod vec;

/*  Modules used function and variable  */
use paging::{init_paging, alloc_pages_at_addr, page_directory, VirtAddr};
use allocator::{linked_list::LinkedListAllocator, /*init_heap,*/ init_kheap};
use vga_buffer::color::Color;
use cli::Command;

#[global_allocator]
static mut ALLOCATOR: LinkedListAllocator = LinkedListAllocator::new();

//#[global_allocator]
//static mut ALLOCATOR: BumpAllocator = BumpAllocator::new();


/*  Code from boot section  */
#[allow(dead_code)]
extern "C" {
	static gdt_desc: u16;
	fn stack_bottom();
	fn stack_top();
	fn heap();
}

use crate::paging::{PAGE_WRITABLE};

pub fn init_kstack(stack_top: VirtAddr, stack_size: usize) -> VirtAddr {
	let mut nb_page: usize = stack_size / 4096;
	let stack_bottom: VirtAddr = stack_top - (stack_size - 1) as u32;
	if stack_size % 4096 != 0 {
		nb_page += 1;
	}
	alloc_pages_at_addr(stack_bottom, nb_page, PAGE_WRITABLE).expect("unable to allocate pages for stack");
	stack_top
}

/*  Kernel initialisation   */
#[no_mangle]
pub extern "C" fn kinit() {
	multiboot::read_tags();
	init_paging();
	unsafe {init_kheap(heap as u32, 100 * 4096 , &mut ALLOCATOR)};
	let kstack_addr: VirtAddr = 0xffbfffff;
	init_kstack(kstack_addr, 8192);
	/* Reserve some spaces to push things before main */
	unsafe{core::arch::asm!("mov esp, eax", in("eax") kstack_addr - 32)};

	#[cfg(test)]
	test_main();

	#[cfg(not(test))]
	kmain();
}

#[no_mangle]
pub extern "C" fn kmain() -> ! {

	kprintln!("Hello World of {}!", 42);

	change_color!(Color::Red, Color::White);
	kprintln!("Press Ctrl-{} to navigate to the second workspace", '2');
	change_color!(Color::White, Color::Black);

	#[cfg(not(test))]
	test();

	let x = allocator::boxed::Box::new(5 as u64);
	kprintln!("New value: {}", x);
	kprint!("$> ");
	loop {
		if keyboard::keyboard_event() {
			let charcode = keyboard::handle_event();
			clihandle!(charcode);
		}
	}
}


/*  Function to put all tests and keep main clean */
#[cfg(not(test))]
fn test() {
	vec::test();
}

