mod inode;
mod block;
mod gdt;

use core::mem::transmute;
use core::ptr::copy;

const SECTOR_SIZE: u32 = 512;
const DISKNO: u8 = 1;

use crate::pci::ide::IDE;

struct Ext2 {
    sblock: block::BaseSuperblock
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
        let inode_table_block = self.get_gdt_entry(0/* group number */).inode_table;
        let block = self.read_block(inode_table_block);
        
        let index = entry * core::mem::size_of::<inode::Inode>();
        let inode = inode::Inode::from(&block[index..index + core::mem::size_of::<inode::Inode>()]);
        inode
    }
}

pub fn read_superblock() -> block::BaseSuperblock {
    let mut buffer: [u8; 2 * SECTOR_SIZE as usize] = [0; 2 * SECTOR_SIZE as usize];

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

pub fn list_dir() -> crate::vec::Vec<inode::Dentry> {
    let ext2 = Ext2::new();
    let inode = ext2.get_inode_entry(2).dbp0;
    let block = ext2.read_block(inode);

    let mut dentries: crate::vec::Vec<inode::Dentry> = crate::vec::Vec::new();
    
    let mut entry_start = 0;
    while entry_start != 4096 {
        let dentry = inode::Dentry::from(&block[entry_start..block.len()]);
        entry_start = entry_start + dentry.dentry_size as usize;
        dentries.push(dentry);
    }
    dentries
}

pub fn test_ext2() {
    let ext2 = Ext2::new();
    let entry = ext2.get_gdt_entry(0);
    let inode = ext2.get_inode_entry(2).dbp0;
    let block = ext2.read_block(inode);
    
    let mut entry_start = 0;
    while entry_start != 4096 {
        let dentry = inode::Dentry::from(&block[entry_start..block.len()]);
        entry_start = entry_start + dentry.dentry_size as usize;
        //crate::kprintln!("{} {}", dentry.dentry_size, dentry.name);
    }
}

