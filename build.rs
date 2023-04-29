fn main() {
	// Adding the linker script
	println!("cargo:rustc-link-arg=-Tarch/x86/linker.ld");
	println!("cargo:rerun-if-changed=arch/x86/linker.ld");
}
