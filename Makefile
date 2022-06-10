VERSION			=	1

QEMU			=	qemu-system-x86_64

LINKER			=	ld
LINKERFILE		=	linker.ld
LINKERFLAGS		=	-n -T $(DIR_ARCH)/$(LINKERFILE)

GRUB_CFG		=	grub.cfg

NASM			=	nasm
ASMFLAGS		=	-felf64 -MP -MD ${basename $@}.d


DOCKER_NAME		=	grub-linker

DIR_ARCH		=	./arch/x86_64
DIR_CONFIG		=	./config
DIR_HEADERS		=	./includes
DIR_SRCS		=	./srcs
DIR_OBJS		=	./compiled_srcs

MAKEFILE_PATH	=	$(dir $(abspath $(lastword $(MAKEFILE_LIST))))

DIR_ISO			=	./iso

vpath %.s $(foreach dir, ${shell find $(DIR_SRCS) -type d}, $(dir))

BOOTSRCS		=	header.s \
					boot.s
BOOTOBJS		=	$(BOOTSRCS:%.s=$(DIR_OBJS)/%.o)
NAME			=	kfs_$(VERSION)

all:			$(NAME)

boot:			$(NAME)
				$(QEMU) -drive format=raw,file=$(NAME)

$(NAME):		$(BOOTOBJS)
				mkdir -p $(DIR_ISO)/boot/grub/
				$(LINKER) $(LINKERFLAGS) $^ -o $(DIR_ISO)/boot/$@
				cp -f $(DIR_CONFIG)/$(GRUB_CFG) $(DIR_ISO)/boot/grub/
				sed -i "s/__kfs__/$(NAME)/" $(DIR_ISO)/boot/grub/$(GRUB_CFG)
ifeq ($(shell docker images -q ${DOCKER_NAME} 2> /dev/null),)
				docker build . -t $(DOCKER_NAME)
endif
				docker run -it --rm -v $(MAKEFILE_PATH):/root $(DOCKER_NAME) -o $(NAME) $(DIR_ISO)

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

re:				fclean
				@$(MAKE) --no-print-directory

.PHONY:			all boot clean fclean re
