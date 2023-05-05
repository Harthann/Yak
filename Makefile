SHELL			=	/bin/bash

ifeq ($(BUILD),release)
NAME			=	kfs
ARGS_CARGO		=	--no-default-features --release
endif

VERSION			=	5

HOST			=	$(shell uname)

TOOLCHAIN_ARCH	=	i386

QEMU			=	qemu-system-$(TOOLCHAIN_ARCH)

GRUB_CFG		=	grub.cfg

DOCKER_DIR		=	docker
DOCKER_TAG		=	kfs/toolchain

DIR_CONFIG		=	config
DIR_SRCS		=	srcs

MAKEFILE_PATH	=	$(dir $(abspath Makefile))

DIR_ISO			=	iso
DIR_GRUB		=	$(DIR_ISO)/boot/grub

DIR_LOGS        =   logs

BUILD			?=	debug
RUST_KERNEL 	?=	target/i386/$(BUILD)/kernel
NAME			?=	kfs_$(VERSION)

################################################################################
# Prepare Docker toolchain if there is no local toolchain
################################################################################
ifeq ($(and $(shell which grub-mkrescue), $(shell which xorriso), $(shell which mformat), $(shell which cargo)),)
ifeq ($(shell docker images -q ${DOCKER_TAG} 2> /dev/null),)
BUILD_DOCKER	:= $(shell docker build $(DOCKER_DIR) -t $(DOCKER_TAG) >&2)
endif
BUILD_PREFIX	= docker run --rm -v $(MAKEFILE_PATH):/root:Z $(DOCKER_TAG) '
BUILD_SUFFIX	= '
endif

################################################################################
# Prepare Docker env if there is no qemu
################################################################################
ifeq ($(and $(shell which $(QEMU))),)
ifeq ($(shell docker images -q ${DOCKER_TAG} 2> /dev/null),)
BUILD_DOCKER	:= $(shell docker build $(DOCKER_DIR) -t $(DOCKER_TAG) >&2)
endif
RUN_PREFIX	= docker run --rm -it -v $(MAKEFILE_PATH):/root:Z $(DOCKER_TAG) '
RUN_SUFFIX	= '
endif
################################################################################

all:			$(NAME)

doc:
				cargo doc $(ARGS_CARGO) --open

boot:			$(NAME) $(DIR_LOGS)
				$(RUN_PREFIX) $(QEMU) -audiodev pa,id=audio0 -machine pcspk-audiodev=audio0\
									  -rtc base=localtime\
									  -no-reboot\
									  -d int\
									  -drive format=raw,file=$(NAME)\
									  -serial file:$(DIR_LOGS)/kernel.log\
									  -serial file:$(DIR_LOGS)/debug_kernel.log\
									  -device isa-debug-exit,iobase=0xf4,iosize=0x04\
									  -display curses 2> $(DIR_LOGS)/qemu.log $(RUN_SUFFIX)

# This rule will run qemu with flags to wait gdb to connect to it
debug:			$(NAME)
				$(RUN_PREFIX) $(QEMU) -s -S -drive format=raw,file=$(NAME)\
										-serial file:$(DIR_LOGS)/kernel.log &\
				gdb $(DIR_ISO)/boot/$(NAME)\
					-ex "target remote localhost:1234"\
					-ex "break kinit"\
					-ex "c";\
				pkill qemu $(RUN_SUFFIX) # When exiting gdb kill qemu

test:			$(DIR_GRUB) $(DIR_GRUB)/$(GRUB_CFG)
				$(BUILD_PREFIX) cargo test $(ARGS_CARGO) -- $(NAME) $(BUILD_SUFFIX)

# Rule to create iso file which can be run with qemu
$(NAME):		$(DIR_ISO)/boot/$(NAME) $(DIR_GRUB)/$(GRUB_CFG) Makefile
				$(BUILD_PREFIX) grub-mkrescue -o $(NAME) $(DIR_ISO) $(BUILD_SUFFIX)

# Put kernel binary inside iso boot for grub-mkrescue
$(DIR_ISO)/boot/$(NAME):	$(RUST_KERNEL) | $(DIR_GRUB)
							cp -f $(RUST_KERNEL) $(DIR_ISO)/boot/$(NAME)

# Let cargo handle build depency - ';' to make empty target
$(RUST_KERNEL):		build;

# Build kernel using cargo
build:
				$(BUILD_PREFIX) cargo build $(ARGS_CARGO) $(BUILD_SUFFIX)

# Check if the rust can compile without actually compiling it
check:
				$(BUILD_PREFIX) cargo check $(ARGS_CARGO) $(BUILD_SUFFIX)

$(DIR_GRUB)/$(GRUB_CFG): $(DIR_CONFIG)/$(GRUB_CFG) | $(DIR_GRUB)
				cp -f $(DIR_CONFIG)/$(GRUB_CFG) $(DIR_GRUB)
ifeq ($(strip $(HOST)),Darwin) # sed on macOS doesn't work like GNU sed
				sed -i '' "s/__kfs__/$(NAME)/" $(DIR_GRUB)/$(GRUB_CFG)
else
				sed -i "s/__kfs__/$(NAME)/" $(DIR_GRUB)/$(GRUB_CFG)
endif

$(DIR_GRUB):
				mkdir -p $(DIR_GRUB)

$(DIR_LOGS):
				mkdir -p $(DIR_LOGS)


clean:
				rm -rf $(DIR_LOGS)
				rm -rf target
				rm -rf Cargo.lock
				rm -rf $(DIR_ISO)

fclean:			clean
				rm -rf kfs*

re:				fclean
				@$(MAKE) --no-print-directory

.PHONY:			all doc boot debug test build check clean fclean re
