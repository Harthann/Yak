var sourcesIndex = JSON.parse('{\
"kernel":["",[["boot",[],["mod.rs"]],["fs",[["file",[],["mod.rs","socket.rs"]]],["mod.rs"]],["gdt",[],["mod.rs","tss.rs"]],["interrupts",[],["idt.rs","int.rs","mod.rs"]],["memory",[["allocator",[],["bump.rs","global.rs","kglobal.rs","linked_list.rs","mod.rs"]],["paging",[],["bitmap.rs","mod.rs","page_directory.rs","page_table.rs"]]],["mod.rs"]],["multiboot",[],["mod.rs"]],["pic",[],["handlers.rs","mod.rs","pit.rs"]],["proc",[],["mod.rs","process.rs","signal.rs","task.rs"]],["sound",[],["mii.rs","mod.rs","note.rs","notes_frequencies.rs","overworld.rs"]],["spin",[],["mod.rs"]],["syscalls",[],["exit.rs","mmap.rs","mod.rs","process.rs","signal.rs","timer.rs"]],["user",[],["mod.rs"]],["utils",[],["arcm.rs","flags.rs","mod.rs","queue.rs"]],["vga_buffer",[],["color.rs","cursor.rs","mod.rs"]]],["cli.rs","cmos.rs","debug.rs","errno.rs","io.rs","keyboard.rs","kinit.rs","kmain.rs","time.rs","wrappers.rs"]]\
}');
createSourceSidebar();
