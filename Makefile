VERSION			=	1

QEMU			=	qemu-system-x86_64

TARGER_ARCH 	=	x86_64

LINKER			=	ld
LINKERFILE		=	linker.ld
LINKERFLAGS		=	-n -T $(DIR_ARCH)/$(LINKERFILE)

GRUB_CFG		=	grub.cfg

NASM			=	nasm
ASMFLAGS		=	-felf64 -MP -MD ${basename $@}.d


DOCKER_DIR		=	docker
DOCKER_GRUB		=	grub-linker
DOCKER_RUST		=	rust-compiler

DIR_ARCH		=	./arch/x86_64
DIR_CONFIG		=	./config
DIR_HEADERS		=	./includes
DIR_SRCS		=	./srcs
DIR_OBJS		=	./compiled_srcs

MAKEFILE_PATH	=	$(dir $(abspath $(lastword $(MAKEFILE_LIST))))

DIR_ISO			=	./iso
DIR_GRUB		=	$(DIR_ISO)/boot/grub

vpath %.s $(foreach dir, ${shell find $(DIR_SRCS) -type d}, $(dir))

BOOTSRCS		=	header.s \
					boot.s
BOOTOBJS		=	$(BOOTSRCS:%.s=$(DIR_OBJS)/%.o)

RUST_KERNEL 	=	target/x86_64-kfs/debug/libkernel.a
NAME			=	kfs_$(VERSION)

all:			$(NAME)

boot:			$(NAME)
				$(QEMU) -drive format=raw,file=$(NAME)

$(NAME):		$(DIR_ISO) $(DIR_GRUB)/$(GRUB_CFG) $(RUST_KERNEL)
ifeq ($(shell docker images -q ${DOCKER_GRUB} 2> /dev/null),)
				docker build $(DOCKER_DIR) -f $(DOCKER_DIR)/$(DOCKER_GRUB).dockerfile -t $(DOCKER_GRUB)
endif
				docker run -it --rm -v $(MAKEFILE_PATH):/root $(DOCKER_GRUB) -o $(NAME) $(DIR_ISO)

$(DIR_ISO):		$(BOOTOBJS) $(RUST_KERNEL)
				mkdir -p $(DIR_GRUB)
				$(LINKER) $(LINKERFLAGS) $^ -o $(DIR_ISO)/boot/$(NAME)

$(RUST_KERNEL): 
ifeq ($(shell docker images -q ${DOCKER_RUST} 2> /dev/null),)
				docker build $(DOCKER_DIR) -f $(DOCKER_DIR)/$(DOCKER_RUST).dockerfile -t $(DOCKER_RUST)
endif
				docker run -it --rm -v $(MAKEFILE_PATH):/root $(DOCKER_RUST) build --target=$(TARGER_ARCH)-kfs

$(DIR_GRUB)/$(GRUB_CFG): $(DIR_CONFIG)/$(GRUB_CFG)
				cp -f $(DIR_CONFIG)/$(GRUB_CFG) $(DIR_GRUB)
				sed -i "s/__kfs__/$(NAME)/" $(DIR_GRUB)/$(GRUB_CFG)

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
ifneq (,$(wildcard $(DIR_ISO)))
				rm -rf $(DIR_ISO)
endif

fclean:			clean
ifneq (,$(wildcard $(NAME)))
				rm -rf $(NAME)
endif

redocker:
				docker build $(DOCKER_DIR) -f $(DOCKER_DIR)/$(DOCKER_GRUB).dockerfile -t $(DOCKER_GRUB)
				docker build $(DOCKER_DIR) -f $(DOCKER_DIR)/$(DOCKER_RUST).dockerfile -t $(DOCKER_RUST)

re:				fclean
				@$(MAKE) --no-print-directory

.PHONY:			all boot clean fclean re
