(function() {var implementors = {};
implementors["kernel"] = [{"text":"impl&lt;T, A:&nbsp;<a class=\"trait\" href=\"kernel/memory/allocator/trait.Allocator.html\" title=\"trait kernel::memory::allocator::Allocator\">Allocator</a>&gt; DerefMut for <a class=\"struct\" href=\"kernel/memory/allocator/boxed/struct.Box.html\" title=\"struct kernel::memory::allocator::boxed::Box\">Box</a>&lt;T, A&gt;","synthetic":false,"types":["kernel::memory::allocator::boxed::Box"]},{"text":"impl DerefMut for <a class=\"struct\" href=\"kernel/string/struct.String.html\" title=\"struct kernel::string::String\">String</a>","synthetic":false,"types":["kernel::string::String"]},{"text":"impl&lt;T, A:&nbsp;<a class=\"trait\" href=\"kernel/memory/allocator/trait.Allocator.html\" title=\"trait kernel::memory::allocator::Allocator\">Allocator</a>&gt; DerefMut for <a class=\"struct\" href=\"kernel/vec/struct.Vec.html\" title=\"struct kernel::vec::Vec\">Vec</a>&lt;T, A&gt;","synthetic":false,"types":["kernel::vec::Vec"]},{"text":"impl&lt;'a, T:&nbsp;?Sized, const INT:&nbsp;bool&gt; DerefMut for <a class=\"struct\" href=\"kernel/spin/struct.MutexGuard.html\" title=\"struct kernel::spin::MutexGuard\">MutexGuard</a>&lt;'a, T, INT&gt;","synthetic":false,"types":["kernel::spin::MutexGuard"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()