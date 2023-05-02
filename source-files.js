var sourcesIndex = {};
sourcesIndex["kernel"] = {"name":"","dirs":[{"name":"boot","files":["mod.rs"]},{"name":"gdt","files":["mod.rs","tss.rs"]},{"name":"interrupts","files":["idt.rs","int.rs","mod.rs"]},{"name":"memory","dirs":[{"name":"allocator","files":["bump.rs","global.rs","kglobal.rs","linked_list.rs","mod.rs"]},{"name":"paging","files":["bitmap.rs","mod.rs","page_directory.rs","page_table.rs"]}],"files":["mod.rs"]},{"name":"multiboot","files":["mod.rs"]},{"name":"pic","files":["handlers.rs","mod.rs","pit.rs"]},{"name":"proc","files":["mod.rs","process.rs","signal.rs","task.rs"]},{"name":"sound","files":["mii.rs","mod.rs","note.rs","notes_frequencies.rs","overworld.rs"]},{"name":"spin","files":["mod.rs"]},{"name":"syscalls","files":["exit.rs","mmap.rs","mod.rs","process.rs","signal.rs","timer.rs"]},{"name":"user","files":["mod.rs"]},{"name":"utils","files":["flags.rs","mod.rs","queue.rs"]},{"name":"vga_buffer","files":["color.rs","cursor.rs","mod.rs"]}],"files":["cli.rs","cmos.rs","debug.rs","errno.rs","io.rs","keyboard.rs","kinit.rs","main.rs","wrappers.rs"]};
createSourceSidebar();
