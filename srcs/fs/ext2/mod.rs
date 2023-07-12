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
    pub fn new() -> Self {
        Self {
            sblock:read_superblock()
        }
    }

    pub fn is_valid(&self) -> bool {
        self.sblock.sig() == 0xef53
    }

    pub fn inode_to_bgroup(&self, inode: u32) -> u32 {
        (inode - 1) / self.sblock.inode_per_grp()
    }

    pub fn inode_size(&self) -> u16 {
        self.sblock.inode_size()
    }

    pub fn read_block(&self, block_no: u32) -> crate::vec::Vec<u8> {
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

    pub fn get_gdt_entry(&self, entry: usize) -> gdt::GdtEntry {
        let entry_per_sector = SECTOR_SIZE as usize / core::mem::size_of::<gdt::GdtEntry>();
        let sector_no = entry / entry_per_sector;

        let block = self.read_block(1 + (self.sblock.bsize() == 1024) as u32);
        let entry_start = (sector_no * SECTOR_SIZE as usize) + (entry % entry_per_sector) * core::mem::size_of::<gdt::GdtEntry>();
        let entry = gdt::GdtEntry::from(&block[entry_start..entry_start+32]);
        entry
    }

    pub fn get_inode_entry(&self, entry: usize) -> inode::Inode {
        if entry < 1 {
            panic!("Ext2fs inodes start indexing at 1");
        }
        let inode_table_block = self.get_gdt_entry(self.inode_to_bgroup(entry as u32) as usize).inode_table;
        
        let index = (entry - 1) * self.inode_size() as usize;
        let block = self.read_block(inode_table_block + (index / self.sblock.bsize() as usize) as u32);
        let index = index % self.sblock.bsize() as usize;
        crate::dprintln!("Trying to get inode: {} found at index: {}", entry, index);
        let inode = inode::Inode::from(&block[index..index + self.inode_size() as usize]);
        inode
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
pub fn get_file_content(path: &str) -> Vec<char> {
    let index = path.rfind('/');
    let filename = match index {
        None => path,
        Some(x) => &path[x+1..path.len()]
    };
    crate::dprintln!("Looking for {filename} in {path}");
    let dentries = list_dir(path.trim_end_matches(filename));
    crate::dprintln!("Found {} entries", dentries.len());
    let mut file: Vec<char> = Vec::new();
    let ext2 = Ext2::new();
    for i in dentries {
        if i.name == filename {
            crate::dprintln!("Found correspondig dentry: {}", filename);
            let inode = ext2.get_inode_entry(i.inode as usize);
            crate::dprintln!("{:#?}", inode);
            let block = ext2.read_block(inode.dbp0);
            for i in 0..inode.size() {
                file.push(block[i as usize] as char);
            }
        }
    }
    file
}

pub fn list_dir(path: &str) -> crate::vec::Vec<inode::Dentry> {
    _list_dir(path.trim_start_matches('/'), 2)
}

pub fn _list_dir(path: &str, inode: usize) -> crate::vec::Vec<inode::Dentry> {
    let ext2 = Ext2::new();
    let inodeentry = ext2.get_inode_entry(inode);
    crate::dprintln!("Found inode: {} {:#?}", inode, inodeentry);
    let block = ext2.read_block(inodeentry.dbp0);

    let opt = path.find('/');
    if path.len() == 0 {
        // list current dir
        crate::dprintln!("Looking in current dir block: {}", inode);
        let mut dentries: crate::vec::Vec<inode::Dentry> = crate::vec::Vec::new();
        let mut entry_start = 0;
        let tmpinode = inode::Inode::from(&block[entry_start..block.len()]);
        crate::dprintln!("Found inode: {:#?}", tmpinode);
        while entry_start != 4096 {
            let dentry = inode::Dentry::from(&block[entry_start..block.len()]);
            
            entry_start = entry_start + dentry.dentry_size as usize;
            dentries.push(dentry);
        }
        return dentries;
    }
    let filename = match opt {
        Some(index) => &path[..index],
        None => path
    };
    crate::dprintln!("Looking in subdirectories: {} {}", path, filename);
    
    let mut entry_start = 0;
    while entry_start != 4096 {
        let dentry = inode::Dentry::from(&block[entry_start..block.len()]);
        entry_start = entry_start + dentry.dentry_size as usize;
        if dentry.name == filename {
            crate::dprintln!("Found dentry: {:#?}", dentry);
            return _list_dir(path.trim_start_matches(filename).trim_start_matches('/'), dentry.inode as usize);
        }
    }
    return crate::alloc::vec::Vec::new();
}


pub fn test_ext2() {
    let ext2 = Ext2::new();
    // Get inode of root directory (always index 2)
    let inode = ext2.get_inode_entry(2 as usize);
    // Read the block of root directory
    crate::dprintln!("{:#?}", inode);
    let mut block = ext2.read_block(inode.dbp0);


    let mut entry_start = 0;
    while entry_start < 4096 {
        let dentry = inode::Dentry::from(&block[entry_start..block.len()]);
        crate::kprintln!("{} {} {}", entry_start, dentry.dentry_size, dentry.name);
        entry_start = entry_start + dentry.dentry_size as usize;
        let tmp = ext2.get_inode_entry(dentry.inode as usize);
        let dtype = match dentry.r#type {
            2 => 'd',
            _ => '-'
        };
        //crate::kprintln!("{}{} {}", dtype, tmp, dentry.name);
    }
}

