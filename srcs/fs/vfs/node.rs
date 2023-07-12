use super::ext2;
use crate::alloc::collections::LinkedList;
use crate::utils::arcm::Arcm;
use crate::alloc::string::String;
use super::ext2::inode;

#[derive(Default)]
pub struct Vfsnode {
    name: String,
    size: u64,
    r#type: u8,
    inode: u32,
    links: u32,
    master: u32,
    father: Option<Arcm<Vfsnode>>,
    childs: LinkedList<Arcm<Vfsnode>>,
    perms: u32
}

impl Vfsnode {
    pub const fn new() -> Self {
        Self {
            name: String::new(),
            size: 0,
            r#type: 0,
            inode: 0,
            links: 0,
            master: 0,
            father: None,
            childs: LinkedList::new(),
            perms: 0
        }
    }

    pub fn create_vinode(dentry: ext2::inode::Dentry, inode: ext2::inode::Inode) -> Vfsnode {
        let mut vinode: Vfsnode = Vfsnode::default();
    
        vinode.name = dentry.name.clone();
        vinode.size = inode.size();
        vinode.r#type = dentry.r#type;
        vinode.inode = dentry.inode;
        vinode.links = inode.get_hardlinks() as u32;
        vinode.master = 0; // idk 
        vinode.perms = inode.get_perms() as u32;
        vinode
    }

    pub fn set_father(&mut self, father: Arcm<Vfsnode>) {
        self.father = Some(father);
    }
    pub fn get_father(&self) -> Option<Arcm<Vfsnode>> {
        self.father.clone()
    }
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn recurs_print(node: Arcm<Vfsnode>, depth: usize) {
        let childs = node.lock().list_childs();
        let padding = depth * 3;
        crate::kprintln!("{:>padding$.padding$} {}", "->", node.lock().name());
        for i in childs {
            Self::recurs_print(i, depth + 1);
        }
    }

    pub fn list_childs(&self) -> LinkedList<Arcm<Vfsnode>> {
        self.childs.clone()
    }

    pub fn insert_child(&mut self, path: &str, child: Arcm<Vfsnode>) {
        let opt = path.find('/');
        match opt {
            Some(index) => {
                let filename = &path[..index];
                crate::dprintln!("Path: {}, Index; {}", path, index);
                crate::dprintln!("Looking for: {} in childs", filename);
                for i in &self.childs {
                    let mut guard = i.lock();
                    if guard.name == filename {                         
                        guard.insert_child(path.trim_start_matches(filename)
                                               .trim_start_matches('/'), child);
                        break ;
                    }
                }
            },
            None => {
                match path.len() {
                    0 => self.create_child(child),
                    _ => {
                        for i in &self.childs {
                            let mut guard = i.lock();
                            if guard.name == path {                         
                                guard.insert_child("", child);
                                break ;
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn create_child(&mut self, child: Arcm<Vfsnode>) {
        crate::dprintln!("Creating chile: {}", child.lock().name);
        self.childs.push_back(child);
    }

    pub fn create_node(name: &str, r#type: inode::Dtype) -> Vfsnode {
        let mut dentry = inode::Dentry::default();
        dentry.r#type = r#type as u8;
        dentry.name = String::from(name);
        let inode = inode::Inode::default();
        Vfsnode::create_vinode(dentry, inode)
    }


    // While removing child should remove all subsequent file/dir ?
    pub fn remove_child(&mut self, name: &str) {
        todo!()
    }

    pub fn clear(&mut self) {
        todo!()
        /*
        for mut i in self.childs {
            i.clear()
        }
        */
    }
}


use core::fmt;
impl fmt::Display for Vfsnode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        write!(f, "{:>8} {}{:x} {}\n", "->", self.r#type, self.perms, self.name)?;
        for i in &self.childs {
            write!(f, "{}", *i.lock())?;
        }
        Ok(())
    }
}
