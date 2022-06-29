VERSION			=	2

QEMU			=	qemu-system-i386

HOST			=	$(shell uname)

ifneq ($(shell which gnome-terminal),)
TERM_EMU	=	gnome-terminal --
else ifneq ($(shell which xterm),)
TERM_EMU	=	xterm -e
else ifneq ($(shell which konsole),)
TERM_EMU	=	konsole -e
else ifeq ($(shell which terminator),)
TERM_EMU	=	terminator -x
else
TERM_EMU	=	$(TERM_EMU)
endif

TARGER_ARCH 	=	i386

LINKERFILE		=	linker.ld
LINKERFLAGS		=	-m elf_i386 -n -T $(DIR_ARCH)/$(LINKERFILE)

GRUB_CFG		=	grub.cfg

NASM			=	nasm
ASMFLAGS		=	-felf32 -MP -MD ${basename $@}.d

DOCKER_DIR		=	docker
DOCKER_GRUB		=	grub-linker
DOCKER_RUST		=	rust-compiler
DOCKER_LINKER	=	linker

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

RUST_KERNEL 	=	target/i386-kfs/debug/libkernel.a
NAME			=	kfs_$(VERSION)

all:			$(NAME)

boot:			$(NAME)
				$(QEMU) -d int -drive format=raw,file=$(NAME) -serial file:$(MAKEFILE_PATH)kernel.log 2> qemu.log

# This rule will run qemu with flags to wait gdb to connect to it
debug:			$(NAME)
				$(QEMU) -s -S -daemonize -drive format=raw,file=$(NAME) -serial file:$(MAKEFILE_PATH)kernel.log
				$(TERM_EMU) bash -c "cd $(MAKEFILE_PATH); gdb $(DIR_ISO)/boot/$(NAME) -x gdbstart"

test:
	echo $(TEST)

# Rule to create iso file which can be run with qemu
$(NAME):		$(DIR_ISO)/boot/$(NAME) $(DIR_GRUB)/$(GRUB_CFG)
ifeq ($(shell docker images -q ${DOCKER_GRUB} 2> /dev/null),)
				docker build $(DOCKER_DIR) -f $(DOCKER_DIR)/$(DOCKER_GRUB).dockerfile -t $(DOCKER_GRUB)
endif
				docker run --rm -u $(shell id -u ${USER}):$(shell id -g ${USER}) -v $(MAKEFILE_PATH):/root:Z $(DOCKER_GRUB) -o $(NAME) $(DIR_ISO)

# Link asm file with rust according to the linker script in arch directory
$(DIR_ISO)/boot/$(NAME):		$(BOOTOBJS) $(RUST_KERNEL) | $(DIR_GRUB)
ifeq ($(shell docker images -q ${DOCKER_LINKER} 2> /dev/null),)
				docker build $(DOCKER_DIR) -f $(DOCKER_DIR)/$(DOCKER_LINKER).dockerfile -t $(DOCKER_LINKER)
endif
				docker run --rm -u $(shell id -u ${USER}):$(shell id -g ${USER}) -v $(MAKEFILE_PATH):/root:Z $(DOCKER_LINKER) $(LINKERFLAGS) $^ -o $(DIR_ISO)/boot/$(NAME)

$(DIR_GRUB):
				mkdir -p $(DIR_GRUB)

# Build libkernel using xargo
$(RUST_KERNEL):	$(KERNELSRCS)
ifeq ($(shell docker images -q ${DOCKER_RUST} 2> /dev/null),)
				docker build $(DOCKER_DIR) -f $(DOCKER_DIR)/$(DOCKER_RUST).dockerfile -t $(DOCKER_RUST)
endif
ifeq ($(shell which xargo),)
				docker run --rm -v $(MAKEFILE_PATH):/root:Z $(DOCKER_RUST) build --target=$(TARGER_ARCH)-kfs
else
				xargo build --target $(TARGER_ARCH)-kfs
endif

# Check if the rust can compile without actually compiling it
check: $(KERNELSRCS)
ifeq ($(shell docker images -q ${DOCKER_RUST} 2> /dev/null),)
				docker build $(DOCKER_DIR) -f $(DOCKER_DIR)/$(DOCKER_RUST).dockerfile -t $(DOCKER_RUST)
endif
				docker run -t --rm -v $(MAKEFILE_PATH):/root:Z $(DOCKER_RUST) check

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
ifneq (,$(wildcard $(DIR_OBJS)))
				rm -rf $(DIR_OBJS)
endif
ifneq (,$(wildcard target))
				rm -rf target
endif
ifneq (,$(wildcard Cargo.lock))
				rm -rf Cargo.lock
endif
ifneq (,$(wildcard $(DIR_ISO)))
				rm -rf $(DIR_ISO)
endif

fclean:			clean
ifneq (,$(wildcard $(NAME)))
				rm -rf $(NAME)
endif

re:				fclean
				@$(MAKE) --no-print-directory

.PHONY:			all boot clean fclean re
