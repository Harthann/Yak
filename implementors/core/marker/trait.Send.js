(function() {var implementors = {};
implementors["kernel"] = [{"text":"impl !Send for <a class=\"struct\" href=\"kernel/cli/struct.Command.html\" title=\"struct kernel::cli::Command\">Command</a>","synthetic":true,"types":["kernel::cli::Command"]},{"text":"impl Send for <a class=\"struct\" href=\"kernel/gdt/tss/struct.Tss.html\" title=\"struct kernel::gdt::tss::Tss\">Tss</a>","synthetic":true,"types":["kernel::gdt::tss::Tss"]},{"text":"impl Send for <a class=\"struct\" href=\"kernel/gdt/struct.GDTR.html\" title=\"struct kernel::gdt::GDTR\">GDTR</a>","synthetic":true,"types":["kernel::gdt::GDTR"]},{"text":"impl Send for <a class=\"struct\" href=\"kernel/gdt/struct.SegmentDescriptor.html\" title=\"struct kernel::gdt::SegmentDescriptor\">SegmentDescriptor</a>","synthetic":true,"types":["kernel::gdt::SegmentDescriptor"]},{"text":"impl Send for <a class=\"struct\" href=\"kernel/keyboard/struct.SpecialKeys.html\" title=\"struct kernel::keyboard::SpecialKeys\">SpecialKeys</a>","synthetic":true,"types":["kernel::keyboard::SpecialKeys"]},{"text":"impl Send for <a class=\"struct\" href=\"kernel/keyboard/struct.Keymap.html\" title=\"struct kernel::keyboard::Keymap\">Keymap</a>","synthetic":true,"types":["kernel::keyboard::Keymap"]},{"text":"impl Send for <a class=\"enum\" href=\"kernel/keyboard/enum.SpecialKeyFlag.html\" title=\"enum kernel::keyboard::SpecialKeyFlag\">SpecialKeyFlag</a>","synthetic":true,"types":["kernel::keyboard::SpecialKeyFlag"]},{"text":"impl Send for <a class=\"struct\" href=\"kernel/memory/allocator/linked_list/struct.ListNode.html\" title=\"struct kernel::memory::allocator::linked_list::ListNode\">ListNode</a>","synthetic":true,"types":["kernel::memory::allocator::linked_list::ListNode"]},{"text":"impl Send for <a class=\"struct\" href=\"kernel/memory/allocator/linked_list/struct.LinkedListAllocator.html\" title=\"struct kernel::memory::allocator::linked_list::LinkedListAllocator\">LinkedListAllocator</a>","synthetic":true,"types":["kernel::memory::allocator::linked_list::LinkedListAllocator"]},{"text":"impl Send for <a class=\"struct\" href=\"kernel/memory/allocator/bump/struct.BumpAllocator.html\" title=\"struct kernel::memory::allocator::bump::BumpAllocator\">BumpAllocator</a>","synthetic":true,"types":["kernel::memory::allocator::bump::BumpAllocator"]},{"text":"impl&lt;T, A&nbsp;=&nbsp;<a class=\"struct\" href=\"kernel/memory/allocator/kglobal/struct.KGlobal.html\" title=\"struct kernel::memory::allocator::kglobal::KGlobal\">KGlobal</a>&gt; !Send for <a class=\"struct\" href=\"kernel/memory/allocator/boxed/struct.Box.html\" title=\"struct kernel::memory::allocator::boxed::Box\">Box</a>&lt;T, A&gt;","synthetic":true,"types":["kernel::memory::allocator::boxed::Box"]},{"text":"impl Send for <a class=\"struct\" href=\"kernel/memory/allocator/global/struct.Global.html\" title=\"struct kernel::memory::allocator::global::Global\">Global</a>","synthetic":true,"types":["kernel::memory::allocator::global::Global"]},{"text":"impl Send for <a class=\"struct\" href=\"kernel/memory/allocator/kglobal/struct.KGlobal.html\" title=\"struct kernel::memory::allocator::kglobal::KGlobal\">KGlobal</a>","synthetic":true,"types":["kernel::memory::allocator::kglobal::KGlobal"]},{"text":"impl Send for <a class=\"struct\" href=\"kernel/memory/allocator/struct.AllocError.html\" title=\"struct kernel::memory::allocator::AllocError\">AllocError</a>","synthetic":true,"types":["kernel::memory::allocator::AllocError"]},{"text":"impl Send for <a class=\"struct\" href=\"kernel/memory/paging/page_directory/struct.PageDirectory.html\" title=\"struct kernel::memory::paging::page_directory::PageDirectory\">PageDirectory</a>","synthetic":true,"types":["kernel::memory::paging::page_directory::PageDirectory"]},{"text":"impl Send for <a class=\"struct\" href=\"kernel/memory/paging/page_directory/struct.PageDirectoryEntry.html\" title=\"struct kernel::memory::paging::page_directory::PageDirectoryEntry\">PageDirectoryEntry</a>","synthetic":true,"types":["kernel::memory::paging::page_directory::PageDirectoryEntry"]},{"text":"impl Send for <a class=\"struct\" href=\"kernel/memory/paging/page_table/struct.PageTable.html\" title=\"struct kernel::memory::paging::page_table::PageTable\">PageTable</a>","synthetic":true,"types":["kernel::memory::paging::page_table::PageTable"]},{"text":"impl Send for <a class=\"struct\" href=\"kernel/memory/paging/page_table/struct.PageTableEntry.html\" title=\"struct kernel::memory::paging::page_table::PageTableEntry\">PageTableEntry</a>","synthetic":true,"types":["kernel::memory::paging::page_table::PageTableEntry"]},{"text":"impl Send for <a class=\"struct\" href=\"kernel/memory/paging/bitmap/struct.Bitmaps.html\" title=\"struct kernel::memory::paging::bitmap::Bitmaps\">Bitmaps</a>","synthetic":true,"types":["kernel::memory::paging::bitmap::Bitmaps"]},{"text":"impl Send for <a class=\"enum\" href=\"kernel/memory/enum.TypeZone.html\" title=\"enum kernel::memory::TypeZone\">TypeZone</a>","synthetic":true,"types":["kernel::memory::TypeZone"]},{"text":"impl Send for <a class=\"struct\" href=\"kernel/memory/struct.MemoryZone.html\" title=\"struct kernel::memory::MemoryZone\">MemoryZone</a>","synthetic":true,"types":["kernel::memory::MemoryZone"]},{"text":"impl Send for <a class=\"struct\" href=\"kernel/multiboot/struct.TagHeader.html\" title=\"struct kernel::multiboot::TagHeader\">TagHeader</a>","synthetic":true,"types":["kernel::multiboot::TagHeader"]},{"text":"impl Send for <a class=\"struct\" href=\"kernel/multiboot/struct.MemInfo.html\" title=\"struct kernel::multiboot::MemInfo\">MemInfo</a>","synthetic":true,"types":["kernel::multiboot::MemInfo"]},{"text":"impl Send for <a class=\"struct\" href=\"kernel/multiboot/struct.MemMapEntry.html\" title=\"struct kernel::multiboot::MemMapEntry\">MemMapEntry</a>","synthetic":true,"types":["kernel::multiboot::MemMapEntry"]},{"text":"impl Send for <a class=\"struct\" href=\"kernel/multiboot/struct.MemMap.html\" title=\"struct kernel::multiboot::MemMap\">MemMap</a>","synthetic":true,"types":["kernel::multiboot::MemMap"]},{"text":"impl Send for <a class=\"struct\" href=\"kernel/multiboot/struct.AddrTag.html\" title=\"struct kernel::multiboot::AddrTag\">AddrTag</a>","synthetic":true,"types":["kernel::multiboot::AddrTag"]},{"text":"impl&lt;T, A&nbsp;=&nbsp;<a class=\"struct\" href=\"kernel/memory/allocator/kglobal/struct.KGlobal.html\" title=\"struct kernel::memory::allocator::kglobal::KGlobal\">KGlobal</a>&gt; !Send for <a class=\"struct\" href=\"kernel/vec/struct.Vec.html\" title=\"struct kernel::vec::Vec\">Vec</a>&lt;T, A&gt;","synthetic":true,"types":["kernel::vec::Vec"]},{"text":"impl !Send for <a class=\"struct\" href=\"kernel/string/struct.String.html\" title=\"struct kernel::string::String\">String</a>","synthetic":true,"types":["kernel::string::String"]},{"text":"impl Send for <a class=\"struct\" href=\"kernel/interrupts/struct.Registers.html\" title=\"struct kernel::interrupts::Registers\">Registers</a>","synthetic":true,"types":["kernel::interrupts::Registers"]},{"text":"impl Send for <a class=\"struct\" href=\"kernel/interrupts/struct.IDT.html\" title=\"struct kernel::interrupts::IDT\">IDT</a>","synthetic":true,"types":["kernel::interrupts::IDT"]},{"text":"impl Send for <a class=\"struct\" href=\"kernel/interrupts/struct.IDTR.html\" title=\"struct kernel::interrupts::IDTR\">IDTR</a>","synthetic":true,"types":["kernel::interrupts::IDTR"]},{"text":"impl Send for <a class=\"struct\" href=\"kernel/interrupts/struct.InterruptDescriptor.html\" title=\"struct kernel::interrupts::InterruptDescriptor\">InterruptDescriptor</a>","synthetic":true,"types":["kernel::interrupts::InterruptDescriptor"]},{"text":"impl Send for <a class=\"struct\" href=\"kernel/syscalls/exit/struct.Timeval.html\" title=\"struct kernel::syscalls::exit::Timeval\">Timeval</a>","synthetic":true,"types":["kernel::syscalls::exit::Timeval"]},{"text":"impl Send for <a class=\"struct\" href=\"kernel/syscalls/exit/struct.RUsage.html\" title=\"struct kernel::syscalls::exit::RUsage\">RUsage</a>","synthetic":true,"types":["kernel::syscalls::exit::RUsage"]},{"text":"impl Send for <a class=\"enum\" href=\"kernel/syscalls/enum.Syscall.html\" title=\"enum kernel::syscalls::Syscall\">Syscall</a>","synthetic":true,"types":["kernel::syscalls::Syscall"]},{"text":"impl Send for <a class=\"enum\" href=\"kernel/vga_buffer/color/enum.Color.html\" title=\"enum kernel::vga_buffer::color::Color\">Color</a>","synthetic":true,"types":["kernel::vga_buffer::color::Color"]},{"text":"impl Send for <a class=\"struct\" href=\"kernel/vga_buffer/color/struct.ColorCode.html\" title=\"struct kernel::vga_buffer::color::ColorCode\">ColorCode</a>","synthetic":true,"types":["kernel::vga_buffer::color::ColorCode"]},{"text":"impl Send for <a class=\"struct\" href=\"kernel/vga_buffer/cursor/struct.Cursor.html\" title=\"struct kernel::vga_buffer::cursor::Cursor\">Cursor</a>","synthetic":true,"types":["kernel::vga_buffer::cursor::Cursor"]},{"text":"impl !Send for <a class=\"struct\" href=\"kernel/vga_buffer/struct.Screen.html\" title=\"struct kernel::vga_buffer::Screen\">Screen</a>","synthetic":true,"types":["kernel::vga_buffer::Screen"]},{"text":"impl Send for <a class=\"struct\" href=\"kernel/vga_buffer/struct.ScreenChar.html\" title=\"struct kernel::vga_buffer::ScreenChar\">ScreenChar</a>","synthetic":true,"types":["kernel::vga_buffer::ScreenChar"]},{"text":"impl Send for <a class=\"struct\" href=\"kernel/vga_buffer/struct.Buffer.html\" title=\"struct kernel::vga_buffer::Buffer\">Buffer</a>","synthetic":true,"types":["kernel::vga_buffer::Buffer"]},{"text":"impl !Send for <a class=\"struct\" href=\"kernel/vga_buffer/struct.Writer.html\" title=\"struct kernel::vga_buffer::Writer\">Writer</a>","synthetic":true,"types":["kernel::vga_buffer::Writer"]},{"text":"impl Send for <a class=\"enum\" href=\"kernel/pic/enum.Pic1.html\" title=\"enum kernel::pic::Pic1\">Pic1</a>","synthetic":true,"types":["kernel::pic::Pic1"]},{"text":"impl Send for <a class=\"enum\" href=\"kernel/pic/enum.Pic2.html\" title=\"enum kernel::pic::Pic2\">Pic2</a>","synthetic":true,"types":["kernel::pic::Pic2"]},{"text":"impl !Send for <a class=\"struct\" href=\"kernel/proc/task/struct.Task.html\" title=\"struct kernel::proc::task::Task\">Task</a>","synthetic":true,"types":["kernel::proc::task::Task"]},{"text":"impl Send for <a class=\"enum\" href=\"kernel/proc/process/enum.Status.html\" title=\"enum kernel::proc::process::Status\">Status</a>","synthetic":true,"types":["kernel::proc::process::Status"]},{"text":"impl !Send for <a class=\"struct\" href=\"kernel/proc/process/struct.Process.html\" title=\"struct kernel::proc::process::Process\">Process</a>","synthetic":true,"types":["kernel::proc::process::Process"]},{"text":"impl Send for <a class=\"enum\" href=\"kernel/proc/signal/enum.SignalType.html\" title=\"enum kernel::proc::signal::SignalType\">SignalType</a>","synthetic":true,"types":["kernel::proc::signal::SignalType"]},{"text":"impl Send for <a class=\"struct\" href=\"kernel/proc/signal/struct.Signal.html\" title=\"struct kernel::proc::signal::Signal\">Signal</a>","synthetic":true,"types":["kernel::proc::signal::Signal"]},{"text":"impl&lt;T:&nbsp;?Sized, const INT:&nbsp;bool&gt; Send for <a class=\"struct\" href=\"kernel/spin/struct.Mutex.html\" title=\"struct kernel::spin::Mutex\">Mutex</a>&lt;T, INT&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Send,&nbsp;</span>","synthetic":true,"types":["kernel::spin::Mutex"]},{"text":"impl&lt;'a, T:&nbsp;?Sized, const INT:&nbsp;bool&gt; Send for <a class=\"struct\" href=\"kernel/spin/struct.MutexGuard.html\" title=\"struct kernel::spin::MutexGuard\">MutexGuard</a>&lt;'a, T, INT&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Send,&nbsp;</span>","synthetic":true,"types":["kernel::spin::MutexGuard"]},{"text":"impl&lt;T&gt; !Send for <a class=\"struct\" href=\"kernel/utils/queue/struct.Queue.html\" title=\"struct kernel::utils::queue::Queue\">Queue</a>&lt;T&gt;","synthetic":true,"types":["kernel::utils::queue::Queue"]},{"text":"impl Send for <a class=\"struct\" href=\"kernel/struct.Tracker.html\" title=\"struct kernel::Tracker\">Tracker</a>","synthetic":true,"types":["kernel::Tracker"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()