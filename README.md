# kfs 'Jellyfish' - 1.0

Our simple Kernel From Scratch made out of asm and rust for i386 - x86.  
Our kernel image will be mostly built using docker to provides easier cross-platform compilation (tested on `MacOS` and `ubuntu`) and didn't need root access rights on host (Simplify the deployment at 42school). The kernel image is built with grub-mkrescue (dockerized) in order to have a GRUB that provide the kernel boot.

## Dependencies
To build the kernel image, the host need `nasm` to compile asm's files and `docker` to build both linker and cargo (compile rust files).
To boot, the host need `qemu` and `qemu-system-i386` executable.

## Compilation
Docker daemon must be running and you must have installed all the dependencies then run:
```
make
```
The bootable image is named `kfs_$VERSION`.

## Boot
To boot and emulate it we provide a simple Makefile command:
```
make boot
```
That will launch qemu on the `kfs_$VERSION` executable.  
A `kernel.log` file at root will store every output.
