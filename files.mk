RUST_SRCS		=	main.rs \
					mod.rs \
					io.rs \
					keyboard.rs \
					cursor.rs \
					color.rs \
					gdt.rs \
					cli.rs \
					page_directory.rs \
					page_table.rs

KERNELSRCS		=	$(foreach file, $(RUST_SRCS), $(shell find $(DIR_SRCS) -name $(file) -type f))

BOOTSRCS		=	header.s \
					boot.s \
					gdt.s

BOOTOBJS		=	$(BOOTSRCS:%.s=$(DIR_OBJS)/%.o)


