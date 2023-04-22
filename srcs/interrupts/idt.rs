use crate::boot::KERNEL_BASE;
use crate::pic::handlers::irq_0;

#[naked]
#[no_mangle]
unsafe fn isr_common_stub() {
	core::arch::asm!(
		"pusha",

		"mov eax, cr3",
		"push eax",

		"xor eax, eax",
		"mov ax, ds",
		"push eax",

		"mov eax, offset page_directory - {}",
		"mov ebx, cr3",
		"cmp eax, ebx",
		"je 2f",

		"mov cr3, eax", // if cr3 is kernel don't swap

		"2:",
		"mov eax, 0x10",
		"mov ds, ax",
		"mov es, ax",
		"mov fs, ax",
		"mov gs, ax",

		"mov eax, esp",

		// (regs: &mut Registers)
		"push eax",
		"call exception_handler",
		"pop eax",

		"call schedule_task",
		const KERNEL_BASE,
		options(noreturn)
	);
}

#[no_mangle]
static isr_stub_syscall: unsafe fn() = isr_stub_128;
#[no_mangle]
static irq_stub_0: unsafe fn() = irq_0;

#[no_mangle]
static isr_stub_table: [unsafe fn(); 48] = [
	isr_stub_0,
	isr_stub_1,
	isr_stub_2,
	isr_stub_3,
	isr_stub_4,
	isr_stub_5,
	isr_stub_6,
	isr_stub_7,
	isr_stub_8,
	isr_stub_9,
	isr_stub_10,
	isr_stub_11,
	isr_stub_12,
	isr_stub_13,
	isr_stub_14,
	isr_stub_15,
	isr_stub_16,
	isr_stub_17,
	isr_stub_18,
	isr_stub_19,
	isr_stub_20,
	isr_stub_21,
	isr_stub_22,
	isr_stub_23,
	isr_stub_24,
	isr_stub_25,
	isr_stub_26,
	isr_stub_27,
	isr_stub_28,
	isr_stub_29,
	isr_stub_30,
	isr_stub_31,
	isr_stub_32,
	isr_stub_33,
	isr_stub_34,
	isr_stub_35,
	isr_stub_36,
	isr_stub_37,
	isr_stub_38,
	isr_stub_39,
	isr_stub_40,
	isr_stub_41,
	isr_stub_42,
	isr_stub_43,
	isr_stub_44,
	isr_stub_45,
	isr_stub_46,
	isr_stub_47
];

isr_no_err_stub!(isr_stub_0, 0);
isr_no_err_stub!(isr_stub_1, 1);
isr_no_err_stub!(isr_stub_2, 2);
isr_no_err_stub!(isr_stub_3, 3);
isr_no_err_stub!(isr_stub_4, 4);
isr_no_err_stub!(isr_stub_5, 5);
isr_no_err_stub!(isr_stub_6, 6);
isr_no_err_stub!(isr_stub_7, 7);
isr_err_stub!(isr_stub_8, 8);
isr_no_err_stub!(isr_stub_9, 9);
isr_err_stub!(isr_stub_10, 10);
isr_err_stub!(isr_stub_11, 11);
isr_err_stub!(isr_stub_12, 12);
isr_err_stub!(isr_stub_13, 13);
isr_err_stub!(isr_stub_14, 14);
isr_no_err_stub!(isr_stub_15, 15);
isr_no_err_stub!(isr_stub_16, 16);
isr_err_stub!(isr_stub_17, 17);
isr_no_err_stub!(isr_stub_18, 18);
isr_no_err_stub!(isr_stub_19, 19);
isr_no_err_stub!(isr_stub_20, 20);
isr_no_err_stub!(isr_stub_21, 21);
isr_no_err_stub!(isr_stub_22, 22);
isr_no_err_stub!(isr_stub_23, 23);
isr_no_err_stub!(isr_stub_24, 24);
isr_no_err_stub!(isr_stub_25, 25);
isr_no_err_stub!(isr_stub_26, 26);
isr_no_err_stub!(isr_stub_27, 27);
isr_no_err_stub!(isr_stub_28, 28);
isr_no_err_stub!(isr_stub_29, 29);
isr_err_stub!(isr_stub_30, 30);
isr_no_err_stub!(isr_stub_31, 31);
isr_no_err_stub!(isr_stub_32, 32);
isr_no_err_stub!(isr_stub_33, 33);
isr_no_err_stub!(isr_stub_34, 34);
isr_no_err_stub!(isr_stub_35, 35);
isr_no_err_stub!(isr_stub_36, 36);
isr_no_err_stub!(isr_stub_37, 37);
isr_no_err_stub!(isr_stub_38, 38);
isr_no_err_stub!(isr_stub_39, 39);
isr_no_err_stub!(isr_stub_40, 40);
isr_no_err_stub!(isr_stub_41, 41);
isr_no_err_stub!(isr_stub_42, 42);
isr_no_err_stub!(isr_stub_43, 43);
isr_no_err_stub!(isr_stub_44, 44);
isr_no_err_stub!(isr_stub_45, 45);
isr_no_err_stub!(isr_stub_46, 46);
isr_no_err_stub!(isr_stub_47, 47);

isr_no_err_stub!(isr_stub_128, 128);

macro_rules! isr_err_stub {
	($func: ident, $nb: expr) => {
		#[naked]
		#[no_mangle]
		unsafe fn $func() {
			core::arch::asm!(
				"cli",
				"push {}",
				"jmp isr_common_stub",
				const $nb,
				options(noreturn)
			);
		}
	};
}

macro_rules! isr_no_err_stub {
	($func: ident, $nb: expr) => {
		#[naked]
		#[no_mangle]
		unsafe fn $func() {
			core::arch::asm!(
				"cli",
				"push 0",
				"push {}",
				"jmp isr_common_stub",
				const $nb,
				options(noreturn)
			);
		}
	};
}

use {isr_err_stub, isr_no_err_stub};
