SHELL			=	/bin/bash

export RUSTUP_TOOLCHAIN=nightly

VERSION			=	4

QEMU			=	qemu-system-i386

HOST			=	$(shell uname)

TARGER_ARCH 	=	i386

GRUB_CFG		=	grub.cfg

NASM			=	nasm
ASMFLAGS		=	-felf32 -MP -MD ${basename $@}.d
LIBBOOT			=	libboot.a

XARGO_FLAGS		=	$(release) --target $(TARGER_ARCH)-kfs

DOCKER_DIR		=	docker
DOCKER_GRUB		=	grub-linker
DOCKER_RUST		=	rust-compiler

DIR_ARCH		=	arch/i386
DIR_CONFIG		=	config
DIR_HEADERS		=	includes
DIR_SRCS		=	srcs
DIR_OBJS		=	compiled_srcs

MAKEFILE_PATH	=	$(dir $(abspath $(lastword $(MAKEFILE_LIST))))

DIR_ISO			=	iso
DIR_GRUB		=	$(DIR_ISO)/boot/grub

vpath %.s $(foreach dir, ${shell find $(DIR_SRCS) -type d}, $(dir))
include files.mk

RUST_KERNEL 	?=	target/i386-kfs/debug/kernel
NAME			?=	kfs_$(VERSION)

all:			$(NAME)

boot:			$(NAME)
				$(QEMU) -no-reboot -d int -drive format=raw,file=$(NAME) -serial file:$(MAKEFILE_PATH)kernel.log -device isa-debug-exit,iobase=0xf4,iosize=0x04 2> qemu.log

# This rule will run qemu with flags to wait gdb to connect to it
debug:			$(NAME)
				$(QEMU) -s -S -daemonize -drive format=raw,file=$(NAME) -serial file:$(MAKEFILE_PATH)kernel.log
				gdb $(DIR_ISO)/boot/$(NAME) -ex "target remote localhost:1234" -ex "break kinit" -ex "c"
				pkill qemu

release:
	make clean all \
		release=--release \
		RUST_KERNEL=target/i386-kfs/release/kernel \
		NAME=kfs

test: $(BOOTOBJS) $(DIR_GRUB) $(DIR_GRUB)/$(GRUB_CFG)
				i386-elf-ar rc $(LIBBOOT) $(BOOTOBJS)
				xargo test $(XARGO_FLAGS) -- $(NAME)

# Rule to create iso file which can be run with qemu
$(NAME):		$(DIR_ISO)/boot/$(NAME) $(DIR_GRUB)/$(GRUB_CFG)
ifeq ($(and $(shell which grub-mkrescue), $(shell which xorriso), $(shell which mformat) ),) 
ifeq ($(shell docker images -q ${DOCKER_GRUB} 2> /dev/null),)
				docker build $(DOCKER_DIR) -f $(DOCKER_DIR)/$(DOCKER_GRUB).dockerfile -t $(DOCKER_GRUB)
endif
				docker run --rm -v $(MAKEFILE_PATH):/root:Z $(DOCKER_GRUB) -o $(NAME) $(DIR_ISO)
else
				grub-mkrescue --compress=xz -o $(NAME) $(DIR_ISO)
endif

# Link asm file with rust according to the linker script in arch directory
$(DIR_ISO)/boot/$(NAME):		$(BOOTOBJS) $(RUST_KERNEL) $(DIR_ARCH)/$(LINKERFILE)| $(DIR_GRUB)
				cp -f $(RUST_KERNEL) iso/boot/$(NAME)

$(DIR_GRUB):
				mkdir -p $(DIR_GRUB)

# Build libkernel using xargo
$(RUST_KERNEL):	$(KERNELSRCS) $(BOOTOBJS) Makefile
ifeq ($(or $(shell which xargo), $(shell which i386-elf-ar) ),)
ifeq ($(shell docker images -q ${DOCKER_RUST} 2> /dev/null),)
				docker build $(DOCKER_DIR) -f $(DOCKER_DIR)/$(DOCKER_RUST).dockerfile -t $(DOCKER_RUST)
endif
				docker run --rm -v $(MAKEFILE_PATH):/root:Z $(DOCKER_RUST) 'i386-elf-ar libboot.a $(BOOTOBJS) && xargo build $(XARGO_FLAGS)'
else
				i386-elf-ar rc libboot.a $(BOOTOBJS)
				xargo build $(XARGO_FLAGS)
endif

# Check if the rust can compile without actually compiling it
check: $(KERNELSRCS)
ifeq ($(shell which xargo),)
ifeq ($(shell docker images -q ${DOCKER_RUST} 2> /dev/null),)
				docker build $(DOCKER_DIR) -f $(DOCKER_DIR)/$(DOCKER_RUST).dockerfile -t $(DOCKER_RUST)
endif
				docker run -t --rm -v $(MAKEFILE_PATH):/root:Z $(DOCKER_RUST) check $(XARGO_FLAGS)
else
				xargo check $(XARGO_FLAGS)
endif

$(DIR_GRUB)/$(GRUB_CFG): $(DIR_CONFIG)/$(GRUB_CFG)
				cp -f $(DIR_CONFIG)/$(GRUB_CFG) $(DIR_GRUB)
ifeq ($(strip $(HOST)),Darwin)
				sed -i '' "s/__kfs__/$(NAME)/" $(DIR_GRUB)/$(GRUB_CFG)
else
				sed -i "s/__kfs__/$(NAME)/" $(DIR_GRUB)/$(GRUB_CFG)
endif

$(BOOTOBJS):	| $(DIR_OBJS)
$(DIR_OBJS)/%.o: %.s
	$(NASM) $(ASMFLAGS) -I $(DIR_HEADERS) -o $@ $<
-include $(ASMOBJS:.o=.d)

$(DIR_OBJS):
				mkdir -p $(DIR_OBJS)

clean:
				rm -rf $(DIR_OBJS)
				rm -rf $(LIBBOOT)
				rm -rf qemu.log kernel.log
				rm -rf target
				rm -rf Cargo.lock
				rm -rf $(DIR_ISO)

fclean:			clean
				rm -rf kfs*

re:				fclean
				@$(MAKE) --no-print-directory

.PHONY:			all boot clean fclean re
