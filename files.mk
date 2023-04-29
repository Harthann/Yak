RUST_SRCS		=	main.rs \
					boot.rs \
					kinit.rs \
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
					errno.rs \
					debug.rs\
					cmos.rs\
					$(SYSCALL_SRCS) \
					$(CLI_SRCS) \
					$(SOUNDS) \
					$(MACROS_SRCS) \
					$(FILE_SYSTEM) \
					bitmap.rs

MACROS_SRCS = lib.rs

CLI_SRCS = mod.rs input.rs commands.rs screen.rs

SYSCALL_SRCS	=	exit.rs \
					signal.rs \
					timer.rs

FILE_SYSTEM = mod.rs

SOUNDS = notes_frequencies.rs \
		 note.rs \
		 overworld.rs \
		 mii.rs

KERNELSRCS		=	$(foreach file, $(RUST_SRCS), $(shell find $(DIR_SRCS) -name $(file) -type f))
