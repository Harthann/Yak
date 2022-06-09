VERSION			=	1

QEMU			=	qemu-system-x86_64

LINKER			=	ld
LINKERFLAGS		=	-melf_i386 --oformat binary

NASM			=	nasm
ASMFLAGS		=	-MP -MD ${basename $@}.d

DIR_HEADERS		=	./includes/
DIR_SRCS		=	./srcs/
DIR_OBJS		=	./compiled_srcs/

vpath %.s $(foreach dir, ${shell find $(DIR_SRCS) -type d}, $(dir))

BOOTSRCS		=	boot.s
BOOTBIN			=	$(BOOTSRCS:%.s=$(DIR_OBJS)%.bin)
NAME			=	kfs_$(VERSION)

all:			$(NAME)

boot:			$(NAME)
				$(QEMU) -drive format=raw,file=$(NAME)

$(NAME):		$(BOOTBIN) 
				cat $^ > $@

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

fclean:			clean
ifneq (,$(wildcard $(NAME)))
				rm -rf $(NAME)
endif

re:				fclean
				@$(MAKE) --no-print-directory

.PHONY:			all clean fclean re
