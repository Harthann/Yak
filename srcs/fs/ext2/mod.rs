mod inode;
mod block;

use core::mem::transmute;
use core::ptr::copy;

const BLOCK_SIZE: u32 = 512;
const DISKNO: u8 = 1;

use crate::pic::ide::IDE;

pub fn read_supeblock() {
    let mut buffer: [u8; 2 * BLOCK_SIZE as usize] = [0; 2 * BLOCK_SIZE as usize];


    unsafe {
        IDE::read_sectors(DISKNO, 1, 2, buffer.as_ptr() as u32);
    }

    let block: block::BaseSuperblock = block::BaseSuperblock::from(&buffer[0..83]);
    crate::kprintln!("{}", block);
}
