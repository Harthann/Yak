pub mod inode;
mod block;
mod gdt;

use core::mem::transmute;
use core::ptr::copy;

const SECTOR_SIZE: u32 = 512;
const DISKNO: u8 = 1;

use crate::pci::ide::IDE;

pub struct Ext2 {
    pub sblock: block::BaseSuperblock
}

impl Ext2 {
    // Temporary hardcoded values (disk number, sector size etc)
    pub fn new() -> Self {
        Self {
            sblock:read_superblock()
        }
    }

    pub fn is_valid(&self) -> bool {
        self.sblock.sig() == 0xef53
    }

    /// Convert an inode number to it's corresponding Group Number
    ///
    /// * `inode` - Inode number to convert
    ///
    /// # Example 
    /// ```
    /// let ext2 = Ext2::new();
    /// let groupno = ext2.inode_to_bgroup(45);
    /// ```
    pub fn inode_to_bgroup(&self, inode: u32) -> u32 {
        (inode - 1) / self.sblock.inode_per_grp()
    }

    /// Convert an inode number to it's corresponding Block Number inside it's group
    ///
    /// * `inode` - Inode number to convert
    ///
    /// # Example 
    /// ```
    /// let ext2 = Ext2::new();
    /// let blockno = ext2.inode_to_block(45);
    /// ```
    pub fn inode_to_block(&self, inode: u32) -> u32 {
        let inode_table_block = self.get_gdt_entry(self.inode_to_bgroup(inode as u32) as usize).inode_table;
        let offset = (inode - 1) * self.inode_size() as u32;
        inode_table_block + offset / self.sblock.bsize()
    }

    /// Convert an inode number to it's offset inside block
    ///
    /// * `inode` - Inode number to convert
    ///
    /// # Example 
    /// ```
    /// let ext2 = Ext2::new();
    /// let offset = ext2.inode_to_offset(45);
    /// ```
    pub fn inode_to_offset(&self, inode: u32) -> u32 {
        let inode_per_block = self.sblock.bsize() as usize /  self.inode_size() as usize;
        ((inode - 1) % inode_per_block as u32) * self.inode_size() as u32
    }

    /// Give inode size indicated by Superblock
    pub fn inode_size(&self) -> u16 {
        self.sblock.inode_size()
    }

    /// Read an entire block from disk
    fn read_block(&self, block_no: u32) -> crate::vec::Vec<u8> {
        let buffer: [u8; SECTOR_SIZE as usize] = [0; SECTOR_SIZE as usize];
        let bsize = self.sblock.bsize() as usize;
        let sector_per_block = bsize / SECTOR_SIZE as usize;

        let sector_no = bsize / SECTOR_SIZE as usize;
        let mut block: crate::vec::Vec<u8> = crate::vec::Vec::new();

        for i in 0..sector_no {
            unsafe {
                 IDE::read_sectors(DISKNO, 1, (block_no * sector_per_block as u32) + i as u32, buffer.as_ptr() as u32);
                 block.extend_from_slice(&buffer[0..SECTOR_SIZE as usize]);
            }
        }
        block
    }

    /// Read disk to recover Group Descriptor Table entry given an index
    fn get_gdt_entry(&self, entry: usize) -> gdt::GdtEntry {
        let entry_per_sector = SECTOR_SIZE as usize / core::mem::size_of::<gdt::GdtEntry>();
        let sector_no = entry / entry_per_sector;

        let block = self.read_block(1 + (self.sblock.bsize() == 1024) as u32);
        let entry_start = (sector_no * SECTOR_SIZE as usize) + (entry % entry_per_sector) * core::mem::size_of::<gdt::GdtEntry>();
        let entry = gdt::GdtEntry::from(&block[entry_start..entry_start+32]);
        entry
    }

    /// Read disk to recover inode struct correcponding to the index passed as parameter
    ///
    /// * `entry` - Inode index to read
    ///
    /// # Example
    ///
    /// ```
    /// let ext2 = Ext2::new();
    ///
    /// // Inode 2 is always inode to root dir
    /// let inode = ext2.get_inode_entry(2);
    /// crate::kprintln!("{:#?}", inode);
    ///
    /// ```
    pub fn get_inode_entry(&self, entry: usize) -> inode::Inode {
        if entry < 1 {
            panic!("Ext2fs inodes start indexing at 1");
        }
        let block = self.read_block(self.inode_to_block(entry as u32));
        let index = self.inode_to_offset(entry as u32) as usize;
        crate::dprintln!("Trying to get inode: {} found at index: {}", entry, index);
        let inode = inode::Inode::from(&block[index..index + self.inode_size() as usize]);
        inode
    }

    /// Find file inside dentry given the dentry inode and file searched.
    /// Currently ignore error cases and remain basic
    ///
    /// * `inodeno` - Inode number of a directory
    ///
    /// * `filename` - Filename to look for inside this directory
    ///
    /// # Example
    ///
    /// ```
    /// let ext2 = Ext2::new();
    /// // Look for "dev" inside inode 2 (inode 2 is the inode for root directory)
    /// let dentry = ext2.dentry_find(2, "dev");
    /// match dentry {
    ///     None => crate::kprintln!("File not found"),
    ///     Some(dir) => crate::kprintln!("Found: {:#?}", dir)
    /// };
    ///
    /// ```
    pub fn dentry_find(&self, inodeno: usize, filename: &str) -> Option<inode::Dentry> {
        // Retrieve inode at index inodeno
        let inode = self.get_inode_entry(inodeno);
        // Read block pointed by inode
        let block = self.read_block(inode.dbp0);
        let mut entry_start = 0;
        while entry_start != 4096 {
            let dentry = inode::Dentry::from(&block[entry_start..block.len()]);
            entry_start = entry_start + dentry.dentry_size as usize;
            if dentry.name == filename {
                return Some(dentry);
            }
        }
        None
    }

    /// Find file given it's path, start search from root directory
    ///
    /// * `path` - Absolute path to searched entry
    ///
    /// # Example
    ///
    /// ```
    /// let ext2 = Ext2::new();
    /// let opt = ext2.get_inode_of("/dev/vga");
    /// match opt {
    ///     Some((inodeno, inode)) => crate::kprintln!("Found at inode {}:\n{:#?}", inodeno, inode);
    ///     None => crate::kprintln!("Not found")
    /// };
    /// ```
    pub fn get_inode_of(&self, path: &str) -> Option<(usize, inode::Inode)> {
        self.recurs_find(path.trim_start_matches('/'), 2)
    }

    /// Perform recursive call to find file pass as argument starting at inodeno
    ///
    /// * `path` - Relative path to file from given inode directory number
    ///
    /// * `inodeno` - Inode number of directory
    ///
    /// # Example
    ///
    /// ```
    /// let ext2 = Ext2::new();
    /// // Look for vga named entry inside directory represented by inode 13
    /// let opt = ext2.recurs_find("vga", 13)
    /// match opt {
    ///     None => crate::kprintln!("Entry does not exist"),
    ///     Some((inodeno, _)) => crate::kprintln!("Entry exist at inode {}", inodeno)
    /// };
    /// ```
    pub fn recurs_find(&self, path: &str, inodeno: usize) -> Option<(usize, inode::Inode)> {
        if path.len() == 0 {
            // Caller has found the entry we search
            return Some((inodeno, self.get_inode_entry(inodeno)));
        }
        let opt = path.find('/');
        let filename = match opt {
            Some(index) => &path[..index],
            None => path
        };
        let dentry = self.dentry_find(inodeno, filename)?;
        let newpath = path.trim_start_matches(filename).trim_start_matches('/');
        self.recurs_find(newpath, dentry.inode as usize)
    }
}

pub fn read_superblock() -> block::BaseSuperblock {
    let mut buffer: [u8; SECTOR_SIZE as usize] = [0; SECTOR_SIZE as usize];

    unsafe {
        IDE::read_sectors(DISKNO, 1, 2, buffer.as_ptr() as u32);
    }
    let mut sblock = block::BaseSuperblock::from(&buffer[0..84]);
    if sblock.version().0 >= 1 {
        sblock.set_extension(block::ExtendedSuperblock::from(&buffer[84..236]));
    }
    sblock
}

pub fn is_ext2() -> bool {
    let fs = Ext2::new();
    fs.is_valid()
}

use crate::vec::Vec;
/// Helper function to get content of a file.
/// Does not yet check if found entry is really a file.
/// Does not yet take into account file bigger than 4096
pub fn get_file_content(path: &str) -> Vec<char> {
    let ext2 = Ext2::new();
    let opt = ext2.get_inode_of(path);
    match opt {
        None => Vec::new(),
        Some((_, inode)) => {
            let block = ext2.read_block(inode.dbp0);
            let file: Vec<char> = block[0..inode.size() as usize].iter()
                                                                 .map(|x| *x as char)
                                                                 .collect();
            file
        }
    }
}

/// Helper function to list all entries in a directory
/// Does not yet check if found entry is a directory or not
pub fn list_dir(path: &str) -> crate::vec::Vec<inode::Dentry> {
    let ext2 = Ext2::new();
    let inode = ext2.get_inode_of(path);
    return match inode {
        None => crate::vec::Vec::new(),
        Some((_, inode)) => {
            let block = ext2.read_block(inode.dbp0);
            let mut dentries: crate::vec::Vec<inode::Dentry> = crate::vec::Vec::new();
            let mut entry_start = 0;
            while entry_start != 4096 {
                let dentry = inode::Dentry::from(&block[entry_start..block.len()]);
                
                entry_start = entry_start + dentry.dentry_size as usize;
                dentries.push(dentry);
            }
            dentries
        }
    };
}

