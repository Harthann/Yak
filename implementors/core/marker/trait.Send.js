(function() {var implementors = {
"kernel":[["impl Send for <a class=\"struct\" href=\"kernel/boot/struct.MultibootHeader.html\" title=\"struct kernel::boot::MultibootHeader\">MultibootHeader</a>",1,["kernel::boot::MultibootHeader"]],["impl Send for <a class=\"struct\" href=\"kernel/boot/struct.Stack.html\" title=\"struct kernel::boot::Stack\">Stack</a>",1,["kernel::boot::Stack"]],["impl Send for <a class=\"enum\" href=\"kernel/cli/input/enum.Input.html\" title=\"enum kernel::cli::input::Input\">Input</a>",1,["kernel::cli::input::Input"]],["impl Send for <a class=\"enum\" href=\"kernel/cli/input/enum.Termcaps.html\" title=\"enum kernel::cli::input::Termcaps\">Termcaps</a>",1,["kernel::cli::input::Termcaps"]],["impl Send for <a class=\"struct\" href=\"kernel/cli/commands/struct.Command.html\" title=\"struct kernel::cli::commands::Command\">Command</a>",1,["kernel::cli::commands::Command"]],["impl Send for <a class=\"struct\" href=\"kernel/cli/struct.TermEmu.html\" title=\"struct kernel::cli::TermEmu\">TermEmu</a>",1,["kernel::cli::TermEmu"]],["impl Send for <a class=\"struct\" href=\"kernel/gdt/tss/struct.Tss.html\" title=\"struct kernel::gdt::tss::Tss\">Tss</a>",1,["kernel::gdt::tss::Tss"]],["impl !Send for <a class=\"struct\" href=\"kernel/gdt/struct.GDTR.html\" title=\"struct kernel::gdt::GDTR\">GDTR</a>",1,["kernel::gdt::GDTR"]],["impl Send for <a class=\"struct\" href=\"kernel/gdt/struct.SegmentDescriptor.html\" title=\"struct kernel::gdt::SegmentDescriptor\">SegmentDescriptor</a>",1,["kernel::gdt::SegmentDescriptor"]],["impl Send for <a class=\"struct\" href=\"kernel/keyboard/struct.SpecialKeys.html\" title=\"struct kernel::keyboard::SpecialKeys\">SpecialKeys</a>",1,["kernel::keyboard::SpecialKeys"]],["impl Send for <a class=\"struct\" href=\"kernel/keyboard/struct.Keymap.html\" title=\"struct kernel::keyboard::Keymap\">Keymap</a>",1,["kernel::keyboard::Keymap"]],["impl Send for <a class=\"enum\" href=\"kernel/keyboard/enum.SpecialKeyFlag.html\" title=\"enum kernel::keyboard::SpecialKeyFlag\">SpecialKeyFlag</a>",1,["kernel::keyboard::SpecialKeyFlag"]],["impl Send for <a class=\"struct\" href=\"kernel/kmain/poc/struct.Poc.html\" title=\"struct kernel::kmain::poc::Poc\">Poc</a>",1,["kernel::kmain::poc::Poc"]],["impl Send for <a class=\"struct\" href=\"kernel/memory/allocator/bump/struct.BumpAllocator.html\" title=\"struct kernel::memory::allocator::bump::BumpAllocator\">BumpAllocator</a>",1,["kernel::memory::allocator::bump::BumpAllocator"]],["impl Send for <a class=\"struct\" href=\"kernel/memory/allocator/linked_list/struct.ListNode.html\" title=\"struct kernel::memory::allocator::linked_list::ListNode\">ListNode</a>",1,["kernel::memory::allocator::linked_list::ListNode"]],["impl Send for <a class=\"struct\" href=\"kernel/memory/allocator/linked_list/struct.LinkedListAllocator.html\" title=\"struct kernel::memory::allocator::linked_list::LinkedListAllocator\">LinkedListAllocator</a>",1,["kernel::memory::allocator::linked_list::LinkedListAllocator"]],["impl Send for <a class=\"struct\" href=\"kernel/memory/allocator/kglobal/struct.KGlobal.html\" title=\"struct kernel::memory::allocator::kglobal::KGlobal\">KGlobal</a>",1,["kernel::memory::allocator::kglobal::KGlobal"]],["impl Send for <a class=\"struct\" href=\"kernel/memory/allocator/struct.AllocError.html\" title=\"struct kernel::memory::allocator::AllocError\">AllocError</a>",1,["kernel::memory::allocator::AllocError"]],["impl Send for <a class=\"struct\" href=\"kernel/memory/paging/bitmap/struct.Bitmaps.html\" title=\"struct kernel::memory::paging::bitmap::Bitmaps\">Bitmaps</a>",1,["kernel::memory::paging::bitmap::Bitmaps"]],["impl Send for <a class=\"struct\" href=\"kernel/memory/paging/page_directory/struct.PageDirectory.html\" title=\"struct kernel::memory::paging::page_directory::PageDirectory\">PageDirectory</a>",1,["kernel::memory::paging::page_directory::PageDirectory"]],["impl Send for <a class=\"struct\" href=\"kernel/memory/paging/page_directory/struct.PageDirectoryEntry.html\" title=\"struct kernel::memory::paging::page_directory::PageDirectoryEntry\">PageDirectoryEntry</a>",1,["kernel::memory::paging::page_directory::PageDirectoryEntry"]],["impl Send for <a class=\"struct\" href=\"kernel/memory/paging/page_table/struct.PageTable.html\" title=\"struct kernel::memory::paging::page_table::PageTable\">PageTable</a>",1,["kernel::memory::paging::page_table::PageTable"]],["impl Send for <a class=\"struct\" href=\"kernel/memory/paging/page_table/struct.PageTableEntry.html\" title=\"struct kernel::memory::paging::page_table::PageTableEntry\">PageTableEntry</a>",1,["kernel::memory::paging::page_table::PageTableEntry"]],["impl Send for <a class=\"enum\" href=\"kernel/memory/enum.TypeZone.html\" title=\"enum kernel::memory::TypeZone\">TypeZone</a>",1,["kernel::memory::TypeZone"]],["impl Send for <a class=\"struct\" href=\"kernel/memory/struct.MemoryZone.html\" title=\"struct kernel::memory::MemoryZone\">MemoryZone</a>",1,["kernel::memory::MemoryZone"]],["impl Send for <a class=\"struct\" href=\"kernel/interrupts/struct.Registers.html\" title=\"struct kernel::interrupts::Registers\">Registers</a>",1,["kernel::interrupts::Registers"]],["impl Send for <a class=\"struct\" href=\"kernel/interrupts/struct.IDT.html\" title=\"struct kernel::interrupts::IDT\">IDT</a>",1,["kernel::interrupts::IDT"]],["impl Send for <a class=\"struct\" href=\"kernel/interrupts/struct.IDTR.html\" title=\"struct kernel::interrupts::IDTR\">IDTR</a>",1,["kernel::interrupts::IDTR"]],["impl Send for <a class=\"struct\" href=\"kernel/interrupts/struct.InterruptDescriptor.html\" title=\"struct kernel::interrupts::InterruptDescriptor\">InterruptDescriptor</a>",1,["kernel::interrupts::InterruptDescriptor"]],["impl Send for <a class=\"enum\" href=\"kernel/multiboot/enum.TagType.html\" title=\"enum kernel::multiboot::TagType\">TagType</a>",1,["kernel::multiboot::TagType"]],["impl Send for <a class=\"struct\" href=\"kernel/multiboot/struct.TagHeader.html\" title=\"struct kernel::multiboot::TagHeader\">TagHeader</a>",1,["kernel::multiboot::TagHeader"]],["impl Send for <a class=\"struct\" href=\"kernel/multiboot/struct.MemInfo.html\" title=\"struct kernel::multiboot::MemInfo\">MemInfo</a>",1,["kernel::multiboot::MemInfo"]],["impl Send for <a class=\"struct\" href=\"kernel/multiboot/struct.BootDev.html\" title=\"struct kernel::multiboot::BootDev\">BootDev</a>",1,["kernel::multiboot::BootDev"]],["impl Send for <a class=\"struct\" href=\"kernel/multiboot/struct.MemMapEntry.html\" title=\"struct kernel::multiboot::MemMapEntry\">MemMapEntry</a>",1,["kernel::multiboot::MemMapEntry"]],["impl Send for <a class=\"struct\" href=\"kernel/multiboot/struct.MemMap.html\" title=\"struct kernel::multiboot::MemMap\">MemMap</a>",1,["kernel::multiboot::MemMap"]],["impl Send for <a class=\"struct\" href=\"kernel/multiboot/struct.FrameBufferInfo.html\" title=\"struct kernel::multiboot::FrameBufferInfo\">FrameBufferInfo</a>",1,["kernel::multiboot::FrameBufferInfo"]],["impl Send for <a class=\"struct\" href=\"kernel/multiboot/struct.ElfSymbols.html\" title=\"struct kernel::multiboot::ElfSymbols\">ElfSymbols</a>",1,["kernel::multiboot::ElfSymbols"]],["impl Send for <a class=\"struct\" href=\"kernel/multiboot/struct.ApmTable.html\" title=\"struct kernel::multiboot::ApmTable\">ApmTable</a>",1,["kernel::multiboot::ApmTable"]],["impl Send for <a class=\"struct\" href=\"kernel/multiboot/struct.LoadBasePhys.html\" title=\"struct kernel::multiboot::LoadBasePhys\">LoadBasePhys</a>",1,["kernel::multiboot::LoadBasePhys"]],["impl Send for <a class=\"struct\" href=\"kernel/syscalls/exit/struct.Timeval.html\" title=\"struct kernel::syscalls::exit::Timeval\">Timeval</a>",1,["kernel::syscalls::exit::Timeval"]],["impl Send for <a class=\"struct\" href=\"kernel/syscalls/exit/struct.RUsage.html\" title=\"struct kernel::syscalls::exit::RUsage\">RUsage</a>",1,["kernel::syscalls::exit::RUsage"]],["impl Send for <a class=\"enum\" href=\"kernel/syscalls/enum.Syscall.html\" title=\"enum kernel::syscalls::Syscall\">Syscall</a>",1,["kernel::syscalls::Syscall"]],["impl Send for <a class=\"enum\" href=\"kernel/pic/enum.Pic1.html\" title=\"enum kernel::pic::Pic1\">Pic1</a>",1,["kernel::pic::Pic1"]],["impl Send for <a class=\"enum\" href=\"kernel/pic/enum.Pic2.html\" title=\"enum kernel::pic::Pic2\">Pic2</a>",1,["kernel::pic::Pic2"]],["impl Send for <a class=\"enum\" href=\"kernel/proc/process/enum.Status.html\" title=\"enum kernel::proc::process::Status\">Status</a>",1,["kernel::proc::process::Status"]],["impl !Send for <a class=\"struct\" href=\"kernel/proc/process/struct.Process.html\" title=\"struct kernel::proc::process::Process\">Process</a>",1,["kernel::proc::process::Process"]],["impl Send for <a class=\"struct\" href=\"kernel/proc/signal/struct.SignalHandler.html\" title=\"struct kernel::proc::signal::SignalHandler\">SignalHandler</a>",1,["kernel::proc::signal::SignalHandler"]],["impl Send for <a class=\"enum\" href=\"kernel/proc/signal/enum.SignalType.html\" title=\"enum kernel::proc::signal::SignalType\">SignalType</a>",1,["kernel::proc::signal::SignalType"]],["impl Send for <a class=\"struct\" href=\"kernel/proc/signal/struct.Signal.html\" title=\"struct kernel::proc::signal::Signal\">Signal</a>",1,["kernel::proc::signal::Signal"]],["impl Send for <a class=\"enum\" href=\"kernel/proc/task/enum.TaskStatus.html\" title=\"enum kernel::proc::task::TaskStatus\">TaskStatus</a>",1,["kernel::proc::task::TaskStatus"]],["impl !Send for <a class=\"struct\" href=\"kernel/proc/task/struct.Task.html\" title=\"struct kernel::proc::task::Task\">Task</a>",1,["kernel::proc::task::Task"]],["impl Send for <a class=\"struct\" href=\"kernel/proc/task/struct.TaskStack.html\" title=\"struct kernel::proc::task::TaskStack\">TaskStack</a>",1,["kernel::proc::task::TaskStack"]],["impl Send for <a class=\"struct\" href=\"kernel/time/struct.Time.html\" title=\"struct kernel::time::Time\">Time</a>",1,["kernel::time::Time"]],["impl Send for <a class=\"enum\" href=\"kernel/vga_buffer/color/enum.Color.html\" title=\"enum kernel::vga_buffer::color::Color\">Color</a>",1,["kernel::vga_buffer::color::Color"]],["impl Send for <a class=\"struct\" href=\"kernel/vga_buffer/color/struct.ColorCode.html\" title=\"struct kernel::vga_buffer::color::ColorCode\">ColorCode</a>",1,["kernel::vga_buffer::color::ColorCode"]],["impl Send for <a class=\"struct\" href=\"kernel/vga_buffer/cursor/struct.Cursor.html\" title=\"struct kernel::vga_buffer::cursor::Cursor\">Cursor</a>",1,["kernel::vga_buffer::cursor::Cursor"]],["impl Send for <a class=\"struct\" href=\"kernel/vga_buffer/struct.ScreenChar.html\" title=\"struct kernel::vga_buffer::ScreenChar\">ScreenChar</a>",1,["kernel::vga_buffer::ScreenChar"]],["impl Send for <a class=\"struct\" href=\"kernel/cmos/struct.Time.html\" title=\"struct kernel::cmos::Time\">Time</a>",1,["kernel::cmos::Time"]],["impl Send for <a class=\"enum\" href=\"kernel/errno/enum.ErrNo.html\" title=\"enum kernel::errno::ErrNo\">ErrNo</a>",1,["kernel::errno::ErrNo"]],["impl Send for <a class=\"enum\" href=\"kernel/sound/note/enum.NoteTempo.html\" title=\"enum kernel::sound::note::NoteTempo\">NoteTempo</a>",1,["kernel::sound::note::NoteTempo"]],["impl Send for <a class=\"enum\" href=\"kernel/sound/note/enum.NoteType.html\" title=\"enum kernel::sound::note::NoteType\">NoteType</a>",1,["kernel::sound::note::NoteType"]],["impl Send for <a class=\"struct\" href=\"kernel/sound/note/struct.Note.html\" title=\"struct kernel::sound::note::Note\">Note</a>",1,["kernel::sound::note::Note"]],["impl Send for <a class=\"enum\" href=\"kernel/sound/enum.BeatType.html\" title=\"enum kernel::sound::BeatType\">BeatType</a>",1,["kernel::sound::BeatType"]],["impl Send for <a class=\"struct\" href=\"kernel/sound/struct.Partition.html\" title=\"struct kernel::sound::Partition\">Partition</a>",1,["kernel::sound::Partition"]],["impl&lt;'a, T: ?Sized, const INT: bool&gt; Send for <a class=\"struct\" href=\"kernel/spin/struct.MutexGuard.html\" title=\"struct kernel::spin::MutexGuard\">MutexGuard</a>&lt;'a, T, INT&gt;<span class=\"where fmt-newline\">where\n    T: Send,</span>",1,["kernel::spin::MutexGuard"]],["impl&lt;T: ?Sized&gt; Send for <a class=\"struct\" href=\"kernel/utils/arcm/struct.Arcm.html\" title=\"struct kernel::utils::arcm::Arcm\">Arcm</a>&lt;T&gt;<span class=\"where fmt-newline\">where\n    T: Send,</span>",1,["kernel::utils::arcm::Arcm"]],["impl&lt;T&gt; Send for <a class=\"struct\" href=\"kernel/utils/flags/struct.Flags.html\" title=\"struct kernel::utils::flags::Flags\">Flags</a>&lt;T&gt;<span class=\"where fmt-newline\">where\n    T: Send,</span>",1,["kernel::utils::flags::Flags"]],["impl&lt;T&gt; Send for <a class=\"struct\" href=\"kernel/utils/queue/struct.Queue.html\" title=\"struct kernel::utils::queue::Queue\">Queue</a>&lt;T&gt;<span class=\"where fmt-newline\">where\n    T: Send,</span>",1,["kernel::utils::queue::Queue"]],["impl Send for <a class=\"struct\" href=\"kernel/debug/struct.DWriter.html\" title=\"struct kernel::debug::DWriter\">DWriter</a>",1,["kernel::debug::DWriter"]],["impl Send for <a class=\"struct\" href=\"kernel/fs/file/raw/struct.RawFileMemory.html\" title=\"struct kernel::fs::file::raw::RawFileMemory\">RawFileMemory</a>",1,["kernel::fs::file::raw::RawFileMemory"]],["impl Send for <a class=\"enum\" href=\"kernel/fs/file/socket/enum.SocketDomain.html\" title=\"enum kernel::fs::file::socket::SocketDomain\">SocketDomain</a>",1,["kernel::fs::file::socket::SocketDomain"]],["impl Send for <a class=\"enum\" href=\"kernel/fs/file/socket/enum.SocketType.html\" title=\"enum kernel::fs::file::socket::SocketType\">SocketType</a>",1,["kernel::fs::file::socket::SocketType"]],["impl Send for <a class=\"enum\" href=\"kernel/fs/file/socket/enum.SocketProtocol.html\" title=\"enum kernel::fs::file::socket::SocketProtocol\">SocketProtocol</a>",1,["kernel::fs::file::socket::SocketProtocol"]],["impl Send for <a class=\"struct\" href=\"kernel/fs/file/socket/struct.Socket.html\" title=\"struct kernel::fs::file::socket::Socket\">Socket</a>",1,["kernel::fs::file::socket::Socket"]],["impl Send for <a class=\"struct\" href=\"kernel/struct.Tracker.html\" title=\"struct kernel::Tracker\">Tracker</a>",1,["kernel::Tracker"]],["impl Send for <a class=\"struct\" href=\"kernel/fs/file/struct.FileInfo.html\" title=\"struct kernel::fs::file::FileInfo\">FileInfo</a>"],["impl Send for <a class=\"struct\" href=\"kernel/vga_buffer/struct.Writer.html\" title=\"struct kernel::vga_buffer::Writer\">Writer</a>"],["impl Send for <a class=\"struct\" href=\"kernel/vga_buffer/struct.Screen.html\" title=\"struct kernel::vga_buffer::Screen\">Screen</a>"],["impl&lt;T: ?Sized + Send, const INT: bool&gt; Send for <a class=\"struct\" href=\"kernel/spin/struct.RawMutex.html\" title=\"struct kernel::spin::RawMutex\">RawMutex</a>&lt;T, INT&gt;"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()