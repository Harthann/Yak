pub mod segments;


extern "C" {
	fn gdt_desc();
	fn _start();
	fn stack_bottom();
	fn stack_top();
	fn loadgdt(addr: u32, code: u32, data: u16);
}

pub fn gdt_init() {
	segments::set_segment(segments::KERNEL_CODE, 0x00 as u32, 0x00ffffff, 0x9a, 0xcf);
	segments::set_segment(segments::KERNEL_CODE, 0x00 as u32, 0x00ffffff, 0x9a, 0xcf);
	segments::set_segment(segments::KERNEL_STACK, stack_bottom as u32, stack_top as u32 - stack_bottom as u32, 0x0c, 0x96);
	//unsafe { loadgdt(gdt_desc as u32, 0x08, 0x10) };
	segments::print_gdt();
}


