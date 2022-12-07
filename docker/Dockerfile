FROM alpine:3.16

# Install base packages
RUN apk update
RUN apk add curl

# Install compiler packages
RUN apk add nasm gcc gdb

# Create needed i386 ld and ar
RUN echo -e '#!/bin/sh\nld -m elf_i386 $@' > /usr/bin/i386-elf-ld
RUN echo -e '#!/bin/sh\nar $@ --target=elf32-i386' > /usr/bin/i386-elf-ar
RUN chmod +x /usr/bin/i386-elf-ld
RUN chmod +x /usr/bin/i386-elf-ar

# Install grub packages
RUN apk add grub-bios xz xorriso

# Install rust
RUN curl --proto '=https' --tlsv1.2 -sSf -o rustup-init.sh https://sh.rustup.rs
RUN chmod +x rustup-init.sh
RUN RUSTUP_HOME=/opt/rustup CARGO_HOME=/opt/cargo ./rustup-init.sh -y --default-toolchain none

# Install qemu
RUN apk add qemu-system-i386 qemu-ui-curses

# Initial setup
WORKDIR /root
ENV RUST_TARGET_PATH=/root
ENV PATH="${PATH}:/opt/rustup/bin:/opt/cargo/bin"

ENTRYPOINT ["/bin/sh", "-c"]
