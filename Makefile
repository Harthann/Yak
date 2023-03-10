SHELL			=	/bin/bash

ifeq ($(BUILD),release)
NAME			=	kfs
ARGS_CARGO		=	--no-default-features --release
endif

VERSION			=	5

QEMU			=	qemu-system-i386

HOST			=	$(shell uname)

TARGET_ARCH 	=	x86
TOOLCHAIN_ARCH	=	i386

GRUB_CFG		=	grub.cfg

NASM			=	nasm
ASMFLAGS		=	-felf32 -MP -MD ${basename $@}.d

AR				=	$(TOOLCHAIN_ARCH)-elf-ar
ARFLAGS			=	rc

LIBBOOT			=	libboot.a

DOCKER_DIR		=	docker
DOCKER			=	kfs/toolchain

DIR_ARCH		=	arch/$(TARGET_ARCH)
DIR_CONFIG		=	config
DIR_HEADERS		=	includes
DIR_SRCS		=	srcs
DIR_OBJS		=	compiled_srcs

MAKEFILE_PATH	=	$(dir $(abspath Makefile))

DIR_ISO			=	iso
DIR_GRUB		=	$(DIR_ISO)/boot/grub

vpath %.s $(foreach dir, ${shell find $(DIR_SRCS) -type d}, $(dir))
include files.mk

BUILD			?=	debug
RUST_KERNEL 	?=	target/i386/$(BUILD)/kernel
NAME			?=	kfs_$(VERSION)

################################################################################
# Prepare Docker toolchain if there is no local toolchain
################################################################################
ifeq ($(and $(shell which grub-mkrescue), $(shell which xorriso), $(shell which mformat), $(shell which $(AR)), $(shell which cargo)),)
ifeq ($(shell docker images -q ${DOCKER} 2> /dev/null),)
BUILD_DOCKER	:= $(shell docker build $(DOCKER_DIR) -t $(DOCKER) >&2)
endif
BUILD_PREFIX	= docker run --rm -v $(MAKEFILE_PATH):/root:Z $(DOCKER) '
BUILD_SUFFIX	= '
endif

################################################################################
# Prepare Docker env if there is no qemu
################################################################################
ifeq ($(and $(shell which $(QEMU))),)
ifeq ($(shell docker images -q ${DOCKER} 2> /dev/null),)
BUILD_DOCKER	:= $(shell docker build $(DOCKER_DIR) -t $(DOCKER) >&2)
endif
RUN_PREFIX	= docker run --rm -it -v $(MAKEFILE_PATH):/root:Z $(DOCKER) '
RUN_SUFFIX	= '
endif
################################################################################

all:			$(NAME)

doc:
				cargo doc $(ARGS_CARGO) --open

boot:			$(NAME)
				$(RUN_PREFIX) $(QEMU) -no-reboot -d int -drive format=raw,file=$(NAME) -serial file:kernel.log -device isa-debug-exit,iobase=0xf4,iosize=0x04 -display curses 2> qemu.log $(RUN_SUFFIX)

# This rule will run qemu with flags to wait gdb to connect to it
debug:			$(NAME)
				$(RUN_PREFIX) $(QEMU) -s -S -drive format=raw,file=$(NAME) -serial file:kernel.log &\
				gdb $(DIR_ISO)/boot/$(NAME) -ex "target remote localhost:1234" -ex "break kinit" -ex "c";\
				pkill qemu $(RUN_SUFFIX)

test:			$(LIBBOOT) $(DIR_GRUB) $(DIR_GRUB)/$(GRUB_CFG)
				$(BUILD_PREFIX) cargo test $(ARGS_CARGO) -- $(NAME) $(BUILD_SUFFIX)

# Rule to create iso file which can be run with qemu
$(NAME):		$(DIR_ISO)/boot/$(NAME) $(DIR_GRUB)/$(GRUB_CFG)
				$(BUILD_PREFIX) grub-mkrescue --compress=xz -o $(NAME) $(DIR_ISO) $(BUILD_SUFFIX)

$(LIBBOOT):		$(BOOTOBJS)
				$(BUILD_PREFIX) $(AR) $(ARFLAGS) $(LIBBOOT) $(BOOTOBJS) $(BUILD_SUFFIX)

# Link asm file with rust according to the linker script in arch directory
$(DIR_ISO)/boot/$(NAME):	$(LIBBOOT) $(RUST_KERNEL) $(DIR_ARCH)/$(LINKERFILE) | $(DIR_GRUB)
							cp -f $(RUST_KERNEL) iso/boot/$(NAME)

$(DIR_GRUB):
				mkdir -p $(DIR_GRUB)

# Build libkernel using cargo
$(RUST_KERNEL):	$(KERNELSRCS) $(BOOTOBJS) Makefile Cargo.toml $(addprefix $(DIR_HEADERS)/, $(INCLUDES))
				$(BUILD_PREFIX) cargo build $(ARGS_CARGO) $(BUILD_SUFFIX)

# Check if the rust can compile without actually compiling it
check:			$(KERNELSRCS)
				$(BUILD_PREFIX) cargo check $(ARGS_CARGO) $(BUILD_SUFFIX)

$(DIR_GRUB)/$(GRUB_CFG): $(DIR_CONFIG)/$(GRUB_CFG)
				cp -f $(DIR_CONFIG)/$(GRUB_CFG) $(DIR_GRUB)
ifeq ($(strip $(HOST)),Darwin)
				sed -i '' "s/__kfs__/$(NAME)/" $(DIR_GRUB)/$(GRUB_CFG)
else
				sed -i "s/__kfs__/$(NAME)/" $(DIR_GRUB)/$(GRUB_CFG)
endif

$(BOOTOBJS):	| $(DIR_OBJS)
$(DIR_OBJS)/%.o: %.s
				$(BUILD_PREFIX) $(NASM) $(ASMFLAGS) -I $(DIR_HEADERS) -o $@ $< $(BUILD_SUFFIX)
-include $(BOOTOBJS:.o=.d)

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
