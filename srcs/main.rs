#![feature(const_mut_refs)]
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

/*  Modules used function and variable  */
use paging::{init_paging,
alloc_page,
alloc_pages,
free_page,
page_directory
};
use vga_buffer::color::Color;
use cli::Command;

/*  Code from boot section  */
#[allow(dead_code)]
extern "C" {
	static gdt_desc: u16;
	fn stack_bottom();
	fn stack_top();
	fn heap();
}

/*  Kernel initialisation   */
#[no_mangle]
pub extern "C" fn kinit() {
//    multiboot::read_tags();
	init_paging();
    
    #[cfg(test)]
    test_main();

    #[cfg(not(test))]
	kmain();

    io::outb(0xf4, 0x10);
}

#[no_mangle]
pub extern "C" fn kmain() -> ! {

	kprintln!("Hello World of {}!", 42);

	change_color!(Color::Red, Color::White);
	kprintln!("Press Ctrl-{} to navigate to the second workspace", '2');
	change_color!(Color::White, Color::Black);

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
	unsafe {
//		let ptr: kmemory::PhysAddr = kmemory::physmap_as_mut().get_page();
//		kprintln!("Get this {:#x}", ptr);
//		kprintln!("Get this {:#x}", kmemory::physmap_as_mut().get_page());
//		kmemory::physmap_as_mut().free_page(ptr);
//		kprintln!("Get this {:#x}", kmemory::physmap_as_mut().get_page());

		/* TESTS PAGES */
//		page_directory.new_page_table();
//		kprintln!("page[1]: {}", page_directory.entries[0]);
		let mut res = alloc_page();
		if !res.is_ok() {
			kprintln!("ko");
			core::arch::asm!("hlt");
		}
		let mut virt_addr: u32 = res.unwrap();
		kprintln!("virt_addr: {:#x}", virt_addr);
		kprintln!("paddr: {:#x}", get_paddr!(virt_addr as usize));
		let mut nb: *mut usize = &mut *(virt_addr as *mut usize);
		kprintln!("init value of nb: {:#x}", *nb);
		*nb = 8;
		kprintln!("next value of nb: {:#x}", *nb);
		res = alloc_pages(50);
		if !res.is_ok() {
			kprintln!("ko");
			core::arch::asm!("hlt");
		}
		virt_addr = res.unwrap();
		let mut i: usize = 0; 
		while i < (8 * 0x1000) - 4 {
			virt_addr += 4;
			nb = &mut *(virt_addr as *mut usize);
			kprintln!("{:#x}", virt_addr);
			*nb = 8;
			i += 4;
		}
		/*
		let mut i = 0;
		while i < 0x100000 {
			res = alloc_page();
			if !res.is_ok() {
				kprintln!("ko");
				core::arch::asm!("hlt");
			}
			virt_addr = res.unwrap();
			kprintln!("{}: {:#010x}", i, virt_addr);
			if i % 2 == 0 {
				free_page(virt_addr);
			}
			i += 1;
		}
		*/
//		page_directory.remove_page_frame(virt_addr);
//		*nb = 0x1000;
//		kprintln!("next value of nb: {:#x}", *nb);
//		kprintln!("paddr: {:#x}", get_paddr!(virt_addr as usize));
	}
}

