mod node;

pub use node::Vfsnode;
use crate::alloc::string::ToString;
use crate::alloc::string::String;
use crate::utils::arcm::Arcm;
use super::ext2;

/// Virtual File System, this wraps the hardware fs interaction, currently ext2, and give an
/// interface for software to interact with it. This vfs is also use as a dir/file map avoiding I/O
/// with the hardrive for small operation like listing
pub struct Vfs {
    mount_point: String,
    root: Arcm<Vfsnode>
}

impl Vfs {
    pub fn new(mount_point: &str, root: Vfsnode) -> Self {
        Self {
            mount_point: mount_point.to_string(),
            root: Arcm::new(root)
        }
    }

    /// Insert node inside the filesystem tree using path as index
    pub fn insert(&mut self, path: &str, node: Vfsnode) {
        self.try_insert(path, node).expect("Failed to insert node in Vfs")
    }

    /// Insert node inside the filesystem tree using path as index.
    /// Return errr if something fail or node already exist.
    pub fn try_insert(&mut self, path: &str, node: Vfsnode) -> Result<(),()> {
        self.root.lock().insert_child(path.trim_start_matches("/"), Arcm::new(node));
        Ok(())
    }

    pub fn remove(&mut self, path: &str) {
        self.try_remove(path).expect("Failed to remove item")
    }
    // Return error if fail to remove (ex: node does not exist)
    pub fn try_remove(&mut self, path: &str) -> Result<(), ()> {
        todo!()
    }

    pub fn find(&mut self, path: &str) -> Result<Arcm<Vfsnode>, ()> {
        todo!()
    }

    pub fn print_tree(&self) {
        Vfsnode::recurs_print(self.root.clone(), 0);
    }
}

mod test {
    use super::Vfs;
    use super::node::Vfsnode;
    use super::ext2::inode;
    use crate::alloc::string::String;

    #[crate::sys_macros::test_case]
    fn vfs_insert() {
        let mut vfs = Vfs::new("/", Vfsnode::create_node("/", inode::Dtype::Directory));

        vfs.insert("/", Vfsnode::create_node("sys", inode::Dtype::Directory));
        vfs.insert("/", Vfsnode::create_node("tmp", inode::Dtype::Directory));
        vfs.insert("/", Vfsnode::create_node("proc", inode::Dtype::Directory));
        vfs.insert("/proc", Vfsnode::create_node("subdir", inode::Dtype::Directory));
        vfs.insert("/proc/subdir", Vfsnode::create_node("subdir2", inode::Dtype::Directory));
        vfs.insert("/sys", Vfsnode::create_node("subdir3", inode::Dtype::Directory));

        vfs.print_tree();
    }
}
