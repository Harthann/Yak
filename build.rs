fn main() {
	// Linking the C/asm library
	println!("cargo:rustc-link-search=native=./");
	println!("cargo:rustc-link-lib=static=boot");
	println!("cargo:rerun-if-changed=libboot.a");

	// Adding the linker script
	println!("cargo:rustc-link-arg=-Tarch/x86/linker.ld");
	println!("cargo:rerun-if-changed=arch/x86/linker.ld");
}
