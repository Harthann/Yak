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
NAME			?=	kfs_$(VERSION)


################################################################################
# GDB SETUP
################################################################################
GDB             ?=  0

# Check if custom gdb startup script exist and use it if present
ifneq (, $(wildcard ./start.gdb))
GDB_STARTUP=--command=./start.gdb
else
# Default gdb startup command
GDB_STARTUP=-ex "target remote localhost:1234" -ex "break kinit" -ex "c";
endif

################################################################################
# QEMU additional arguments
################################################################################
QEMU_ARGS := 
# Add qemu gdb option if gdb env variable is set to 1
ifeq ($(GDB), 1)
	QEMU_ARGS += -s -S
endif

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
				$(BUILD_PREFIX) cargo run $(ARGS_CARGO) -- $(NAME) $@ "$(QEMU_ARGS)" $(BUILD_SUFFIX)

test:			$(NAME) $(DIR_LOGS)
				$(BUILD_PREFIX) cargo test $(ARGS_CARGO) -- $(NAME) $@ "$(QEMU_ARGS)" $(BUILD_SUFFIX)

# This rule will run qemu with flags to wait gdb to connect to it
debug:			$(NAME)
				gdb $(DIR_ISO)/boot/$(NAME) $(GDB_STARTUP)
				pkill qemu $(RUN_SUFFIX) # When exiting gdb kill qemu

# Rule to create iso file which can be run with qemu
$(NAME):		 build $(DIR_GRUB)/$(GRUB_CFG) Makefile;

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
