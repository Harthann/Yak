#![feature(const_mut_refs)]
#![feature(lang_items)]
#![no_std]
#![allow(dead_code)]

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

use paging::init_paging;
use paging::alloc_page;
use paging::alloc_pages;
use paging::kalloc_pages;
use paging::free_page;
use paging::free_pages;
use paging::page_directory;

use paging::VirtAddr;

use allocator::linked_list::LinkedListAllocator;
use allocator::bump::BumpAllocator;
use allocator::init_heap;

#[global_allocator]
static mut ALLOCATOR: BumpAllocator = BumpAllocator::new();


#[allow(dead_code)]
extern "C" {
	static gdt_desc: u16;
	fn _start();
	fn stack_bottom();
	fn stack_top();
}

use vga_buffer::color::Color;
use cli::Command;

#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn eh_personality() {}

use crate::paging::alloc_pages_at_addr;

pub fn init_stack(stack_top: VirtAddr, stack_size: usize) -> VirtAddr {
	let mut nb_page: usize = stack_size / 4096;
	let stack_bottom: VirtAddr = stack_top - (stack_size - 1) as u32;
	if stack_size % 4096 != 0 {
		nb_page += 1;
	}
	kprintln!("alloc at {:#10x} - {} pages", stack_bottom, nb_page);
	alloc_pages_at_addr(stack_bottom, nb_page);
	stack_top
}

#[no_mangle]
pub extern "C" fn kinit() {
	kprintln!("kinit_start");
	kprintln!("multiboot:");
	multiboot::read_tags();
	kprintln!("init_paging");
	init_paging();
	kprintln!("init_heap");
	init_heap();
	kprintln!("init_stack");
	init_stack(0xffffffff, 8192);
	unsafe{core::arch::asm!("mov esp, eax", in("eax") 0xffffffff as u32)};
	kmain();
}

/*  Function to put all tests and keep main clean */
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
		let mut addr: u32;
		core::arch::asm!("mov eax, esp", out("eax") addr);
		kprintln!("stack_addr at: {:#010x}", addr);

		let mut res = alloc_page();

		if !res.is_ok() {
			kprintln!("ko");
			core::arch::asm!("hlt");
		}
		let mut virt_addr: u32 = res.unwrap();
		let mut saved_virt_addr: u32 = virt_addr;
		kprintln!("virt_addr: {:#x}", virt_addr);
		kprintln!("paddr: {:#x}", get_paddr!(virt_addr as usize));
		let mut nb: *mut usize = &mut *(virt_addr as *mut usize);
		kprintln!("init value of nb: {:#x}", *nb);
		*nb = 8;
		kprintln!("next value of nb: {:#x}", *nb);
		free_page(saved_virt_addr);
		res = alloc_pages(50);
		if !res.is_ok() {
			kprintln!("ko");
			core::arch::asm!("hlt");
		}
		virt_addr = res.unwrap();
		saved_virt_addr = virt_addr;
		kprintln!("virt_addr: {:#x}", virt_addr);
		let mut i: usize = 0; 
		while i < (50 * 0x1000) - 4 {
			virt_addr += 4;
			nb = &mut *(virt_addr as *mut usize);
			*nb = 8;
			i += 4;
		}
		free_pages(saved_virt_addr, 50);
		kprintln!("alloc one");
		res = alloc_pages(2000);
		kprintln!("abc");
		if !res.is_ok() {
			kprintln!("ko");
			core::arch::asm!("hlt");
		}
		virt_addr = res.unwrap();
		saved_virt_addr = virt_addr;
		i = 0;
		while i < (2000 * 0x1000) - 4 {
			virt_addr += 4;
			nb = &mut *(virt_addr as *mut usize);
//			kprintln!("{:#x}", virt_addr);
			*nb = 8;
			i += 4;
		}
		free_pages(saved_virt_addr, 2000);
		kprintln!("kalloc_pages");
		res = kalloc_pages(32);
		if !res.is_ok() {
			kprintln!("ko");
			core::arch::asm!("hlt");
		}
		virt_addr = res.unwrap();
		saved_virt_addr = virt_addr;
		i = 0;
		while i < (32 * 0x1000) - 4 {
			virt_addr += 4;
			nb = &mut *(virt_addr as *mut usize);
//			kprintln!("{:#x}", virt_addr);
			*nb = 8;
			i += 4;
		}
		free_pages(saved_virt_addr, 32);
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

#[no_mangle]
pub extern "C" fn kmain() -> ! {
	kprintln!("Hello World of {}!", 42);
	change_color!(Color::Red, Color::White);
	kprintln!("Press Ctrl-{} to navigate to the second workspace", '2');
	change_color!(Color::White, Color::Black);

	test();

	kprint!("$> ");
	loop {
		if keyboard::keyboard_event() {
			let charcode = keyboard::handle_event();
			clihandle!(charcode);
		}
	}
}
