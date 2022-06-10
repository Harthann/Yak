VERSION			=	1

QEMU			=	qemu-system-x86_64

LINKER			=	ld
LINKERFLAGS		=	-melf_i386 --oformat binary

NASM			=	nasm
ASMFLAGS		=	-MP -MD ${basename $@}.d

DOCKER_NAME		=	grub-linker

DIR_HEADERS		=	./includes/
DIR_SRCS		=	./srcs/
DIR_OBJS		=	./compiled_srcs/

DIR_ISO			=	./iso/

vpath %.s $(foreach dir, ${shell find $(DIR_SRCS) -type d}, $(dir))

BOOTSRCS		=	boot.s
BOOTBIN			=	$(BOOTSRCS:%.s=$(DIR_OBJS)%.bin)
NAME			=	kfs_$(VERSION)

all:			$(NAME)

boot:			$(NAME)
				$(QEMU) -drive format=raw,file=$(NAME)

$(NAME):		$(BOOTBIN)
				mkdir -p $(DIR_ISO)
				cat $^ > $(DIR_ISO)$@
ifeq ($(shell docker images -q ${DOCKER_NAME} 2> /dev/null),)
				docker build . -t $(DOCKER_NAME)
endif
				docker run -it --rm -v $(PWD):/root $(DOCKER_NAME) grub-mkrescue -o $(NAME) $(DIR_ISO)

$(BOOTBIN):		| $(DIR_OBJS)
$(DIR_OBJS)%.bin: %.s
	$(NASM) -f bin $(ASMFLAGS) -I $(DIR_HEADERS) -o $@ $<
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
