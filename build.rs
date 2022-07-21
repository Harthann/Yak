use std::process::Command;

fn main() {
	// Linking the C/asm library
	println!("cargo:rustc-link-search=native=./");
	println!("cargo:rustc-link-lib=static=boot");
	println!("cargo:rerun-if-changed=libboot.a");

	// Adding the linker script
	println!("cargo:rustc-link-arg=-Tarch/i386/linker.ld");
	println!("cargo:rerun-if-changed=arch/i386/linker.ld");
}
