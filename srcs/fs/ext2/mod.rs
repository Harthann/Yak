mod inode;
mod block;

use core::mem::transmute;
use core::ptr::copy;

const BLOCK_SIZE: u32 = 512;
const DISKNO: u8 = 1;

use crate::pci::ide::IDE;

pub fn read_superblock() -> block::BaseSuperblock {
    let mut buffer: [u8; 2 * BLOCK_SIZE as usize] = [0; 2 * BLOCK_SIZE as usize];

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
    let sblock = read_superblock();
    sblock.sig() == 0xef53
}

pub fn test_ext2() {
    let sblock = read_superblock();
    crate::kprintln!("{}", sblock);
//    crate::kprintln!("{:#x} {} {}", sblock.bsize(), sblock.block_per_grp(), sblock.inode_per_grp());
}
