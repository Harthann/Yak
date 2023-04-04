RUST_SRCS		=	main.rs \
					mod.rs \
					io.rs \
					keyboard.rs \
					cursor.rs \
					color.rs \
					gdt.rs \
					cli.rs \
					page_directory.rs \
					page_table.rs \
					linked_list.rs \
					bump.rs \
					global.rs \
					kglobal.rs \
					tss.rs \
					handlers.rs \
					pit.rs \
					wrappers.rs \
					process.rs \
					task.rs \
					signal.rs \
					queue.rs \
					kinit.rs\
					$(SYSCALL_SRCS)

SYSCALL_SRCS	=	exit.rs \
					signal.rs

KERNELSRCS		=	$(foreach file, $(RUST_SRCS), $(shell find $(DIR_SRCS) -name $(file) -type f))
INCLUDES	=		boot.h \
					idt.h \
					task.h

BOOTSRCS		=	boot.s \
					gdt.s \
					idt.s \
					int.s \
					userjump.s \
					task.s

BOOTOBJS		=	$(BOOTSRCS:%.s=$(DIR_OBJS)/%.o)
