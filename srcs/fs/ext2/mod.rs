use crate::alloc::vec;
use crate::pci::ide::IDE;
use crate::string::ToString;
use crate::utils::math::roundup;
use crate::utils::path::Path;

mod bitmap;
pub mod block;
mod gdt;
pub mod inode;

pub static mut DISKNO: i8 = 0;

/// Current read/write use entire block to perform operations
/// In the filesystem created to test it this means we read/write 16 sectors for each operations
/// This is pretty ineffective and will probably need optimisation in later version
pub struct Ext2 {
	diskno:      u8,
	sector_size: usize,
	pub sblock:  block::BaseSuperblock
}

impl Ext2 {
	pub fn new(diskno: u8) -> Result<Self, u8> {
		let sector_size = {
			let binding = IDE.lock();
			let device = binding.get_device(diskno);
			if device.is_none() {
				return Err(1);
			}
			match device.unwrap().r#type {
				x if x == ide::IDEType::ATA as u16 => ide::ata::SECTOR_SIZE,
				x if x == ide::IDEType::ATAPI as u16 => ide::atapi::SECTOR_SIZE,
				_ => {
					panic!("Unrecognized disk.")
				}
			}
		};
		Ok(Self {
			diskno:      diskno,
			sector_size: sector_size as usize,
			sblock:      read_superblock(diskno, sector_size as usize)?
		})
	}

	pub fn is_valid(&self) -> bool {
		self.sblock.sig() == 0xef53
	}

	/// Convert an inode number to it's corresponding Group Number
	///
	/// # Arguments
	///
	/// * `inode` - Inode number to convert
	///
	/// # Example
	/// ```
	/// let ext2 = Ext2::new(unsafe { DISKNO as u8 }).unwrap();
	/// let groupno = ext2.inode_to_bgroup(45);
	/// ```
	pub fn inode_to_bgroup(&self, inode: u32) -> u32 {
		(inode - 1) / self.sblock.inode_per_grp()
	}

	/// Convert an inode number to it's corresponding Block Number inside it's group
	///
	/// # Arguments
	///
	/// * `inode` - Inode number to convert
	///
	/// # Example
	/// ```
	/// let ext2 = Ext2::new(unsafe { DISKNO as u8 }).unwrap();
	/// let blockno = ext2.inode_to_block(45);
	/// ```
	pub fn inode_to_block(&self, inode: u32) -> u32 {
		let inode_table_block = self
			.get_gdt_entry(self.inode_to_bgroup(inode as u32) as usize)
			.inode_table;
		let offset = (inode - 1) * self.inode_size() as u32;
		inode_table_block + offset / self.sblock.bsize() as u32
	}

	/// Convert an inode number to it's offset inside block
	///
	/// # Arguments
	///
	/// * `inode` - Inode number to convert
	///
	/// # Example
	/// ```
	/// let ext2 = Ext2::new(unsafe { DISKNO as u8 }).unwrap();
	/// let offset = ext2.inode_to_offset(45);
	/// ```
	pub fn inode_to_offset(&self, inode: u32) -> u32 {
		let inode_per_block = self.sblock.bsize() / self.inode_size() as usize;
		((inode - 1) % inode_per_block as u32) * self.inode_size() as u32
	}

	/// Give inode size indicated by Superblock
	pub fn inode_size(&self) -> u16 {
		self.sblock.inode_size()
	}

	/// Read an entire block from disk
	pub fn read_block(&self, block_no: u32) -> crate::vec::Vec<u8> {
		let mut bsize = self.sblock.bsize();
		let mut nb_sector = bsize / self.sector_size;
		// sector_size > bsize
		if nb_sector == 0 {
			nb_sector = 1;
		}
		let buffer: Vec<u8> = vec![0; self.sector_size];
		let sector_per_block = bsize as f64 / self.sector_size as f64;

		let first_sector = (bsize * block_no as usize) / self.sector_size;
		let mut block: crate::vec::Vec<u8> = crate::vec::Vec::new();
		for i in first_sector..first_sector + nb_sector {
			IDE.lock().read_sectors(
				self.diskno,
				1,
				i as u32,
				buffer.as_ptr() as u32
			);
			let mut start = 0;
			if sector_per_block < 1.0 {
				start = (block_no as usize % (1.0 / sector_per_block) as usize)
					* bsize;
			} else if sector_per_block > 1.0 {
				bsize = self.sector_size;
			}
			block.extend_from_slice(&buffer[start..start + bsize]);
		}
		block
	}

	fn write_block(&mut self, block_no: u32, block: &[u8]) {
		let bsize = self.sblock.bsize();
		let sector_per_block = bsize / self.sector_size as usize;

		let sector_no = bsize / self.sector_size as usize;
		IDE.lock().write_sectors(
			self.diskno,
			sector_no as u8,
			block_no * sector_per_block as u32,
			block.as_ptr() as u32
		);
	}

	fn write_slice(&mut self, block_no: u32, offset: usize, slice: &[u8]) {
		let mut block = self.read_block(block_no);
		crate::dprintln!("offset: {}", offset);
		crate::dprintln!("slice.len(): {}", slice.len());
		crate::dprintln!("block.len(): {}", block.len());
		block[offset..offset + slice.len()].copy_from_slice(slice);
		self.write_block(block_no, &block);
	}

	fn get_inode(&self, inodeno: usize) -> inode::Inode {
		let block_no = self.inode_to_block(inodeno as u32);
		let block = self.read_block(block_no);
		let index = self.inode_to_offset(inodeno as u32) as usize;
		inode::Inode::from(&block[index..index + self.inode_size() as usize])
	}

	fn write_inode(&mut self, inodeno: usize, inode: &inode::Inode) {
		if inodeno < 1 {
			panic!("Ext2fs inodes start indexing at 1");
		}
		let block_no = self.inode_to_block(inodeno as u32);
		let mut block = self.read_block(block_no);
		let index = self.inode_to_offset(inodeno as u32) as usize;
		crate::dprintln!(
			"Trying to get inode: {} found at index: {}",
			inodeno,
			index
		);
		let mut vec = Into::<Vec<u8>>::into(*inode);
		while vec.len() != self.inode_size() as usize {
			vec.push(0);
		}
		block[index..index + self.inode_size() as usize]
			.copy_from_slice(vec.as_slice());
		self.write_block(block_no, block.as_slice());
	}

	/// Read disk to recover Group Descriptor Table entry given an index
	fn get_gdt_entry(&self, entry: usize) -> gdt::GdtEntry {
		let entry_per_sector =
			self.sector_size as usize / core::mem::size_of::<gdt::GdtEntry>();
		let sector_no = entry / entry_per_sector;

		let block = self.read_block(1 + (self.sblock.bsize() == 1024) as u32);
		let entry_start = (sector_no * self.sector_size as usize)
			+ (entry % entry_per_sector)
				* core::mem::size_of::<gdt::GdtEntry>();
		let entry = gdt::GdtEntry::from(&block[entry_start..entry_start + 32]);
		entry
	}

	pub fn read_inode_map(&self, group: usize) -> bitmap::Bitmap {
		let inode_no = self.get_gdt_entry(group);
		let block = self.read_block(inode_no.bitmap_inode);
		bitmap::Bitmap::from(&block[0..block.len()])
	}
	pub fn read_block_map(&self, group: usize) -> bitmap::Bitmap {
		let inode_no = self.get_gdt_entry(group);
		let block = self.read_block(inode_no.bitmap_block);
		bitmap::Bitmap::from(&block[0..block.len()])
	}
	pub fn write_block_map(&mut self, group: usize, map: bitmap::Bitmap) {
		let inode_no = self.get_gdt_entry(group);
		self.write_block(inode_no.bitmap_block, &map.map);
	}
	pub fn write_inode_map(&mut self, group: usize, map: bitmap::Bitmap) {
		let inode_no = self.get_gdt_entry(group);
		self.write_block(inode_no.bitmap_inode, &map.map);
	}

	pub fn alloc_node(&mut self, group: usize) -> usize {
		let mut map = self.read_inode_map(group);
		let nodeno = map.get_free_node().unwrap();
		self.sblock.inode_unalloc -= 1;
		// TODO: write superblock
		// 		self.write_slice(nodeno as u32, 0, &*self.sblock.into_boxed_slice());
		self.write_inode_map(group, map);
		nodeno
	}

	pub fn free_node(&mut self, group: usize, inodeno: usize) {
		let mut map = self.read_inode_map(group);
		map.unset_node(inodeno);
		self.sblock.inode_unalloc += 1;
		// TODO: write superblock
		// self.write_slice(nodeno as u32, 0, &*self.sblock.into_boxed_slice());
		self.write_inode_map(group, map);
	}

	pub fn alloc_block(&mut self, group: usize) -> usize {
		let mut map = self.read_block_map(group);
		let nodeno = map.get_free_node().unwrap();
		self.sblock.blocks_unalloc -= 1;
		// TODO: write superblock
		// 		self.write_slice(nodeno as u32, 0, &*self.sblock.into_boxed_slice());
		self.write_block_map(group, map);
		self.write_block(
			nodeno as u32,
			crate::vec![0; self.sblock.bsize()].as_slice()
		);
		nodeno
	}

	/// Read disk to recover inode struct correcponding to the index passed as parameter
	///
	/// # Arguments
	///
	/// * `entry` - Inode index to read
	///
	/// # Example
	///
	/// ```
	/// let ext2 = Ext2::new(unsafe { DISKNO as u8 }).unwrap();
	///
	/// // Inode 2 is always inode to root dir
	/// let inode = ext2.get_inode_entry(2);
	/// crate::kprintln!("{:#?}", inode);
	/// ```
	pub fn get_inode_entry(&self, entry: usize) -> inode::Inode {
		if entry < 1 {
			panic!("Ext2fs inodes start indexing at 1");
		}
		let block = self.read_block(self.inode_to_block(entry as u32));
		let index = self.inode_to_offset(entry as u32) as usize;
		crate::dprintln!(
			"Trying to get inode: {} found at index: {}",
			entry,
			index
		);
		let inode = inode::Inode::from(
			&block[index..index + self.inode_size() as usize]
		);
		inode
	}

	/// Find file inside dentry given the dentry inode and file searched.
	/// Currently ignore error cases and remain basic
	///
	/// # Arguments
	///
	/// * `inodeno` - Inode number of a directory
	///
	/// * `filename` - Filename to look for inside this directory
	///
	/// # Example
	///
	/// ```
	/// let ext2 = Ext2::new(unsafe { DISKNO as u8 }).unwrap();
	/// // Look for "dev" inside inode 2 (inode 2 is the inode for root directory)
	/// let dentry = ext2.dentry_find(2, "dev");
	/// match dentry {
	///     None => crate::kprintln!("File not found"),
	///     Some(dir) => crate::kprintln!("Found: {:#?}", dir)
	/// };
	/// ```
	pub fn dentry_find(
		&self,
		inodeno: usize,
		filename: &str
	) -> Option<inode::Dentry> {
		// Retrieve inode at index inodeno
		let inode = self.get_inode_entry(inodeno);
		// Read block pointed by inode
		for block_no in inode.get_blocks_no() {
			let block = self.read_block(block_no);
			let mut entry_start = 0;
			while entry_start != self.sblock.bsize() {
				let dentry =
					inode::Dentry::from(&block[entry_start..block.len()]);
				entry_start = entry_start + dentry.dentry_size as usize;
				if dentry.name == filename {
					return Some(dentry);
				}
			}
		}
		None
	}

	/// Find file given it's path, start search from root directory
	///
	/// # Arguments
	///
	/// * `path` - Absolute path to searched entry
	///
	/// # Example
	///
	/// ```
	/// let ext2 = Ext2::new(unsafe { DISKNO as u8 }).unwrap();
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
	/// # Arguments
	///
	/// * `path` - Relative path to file from given inode directory number
	///
	/// * `inodeno` - Inode number of directory
	///
	/// # Example
	///
	/// ```
	/// let ext2 = Ext2::new(unsafe { DISKNO as u8 }).unwrap();
	/// // Look for vga named entry inside directory represented by inode 13
	/// let opt = ext2.recurs_find("vga", 13)
	/// match opt {
	///     None => crate::kprintln!("Entry does not exist"),
	///     Some((inodeno, _)) => crate::kprintln!("Entry exist at inode {}", inodeno)
	/// };
	/// ```
	pub fn recurs_find(
		&self,
		path: &str,
		inodeno: usize
	) -> Option<(usize, inode::Inode)> {
		if path.len() == 0 {
			// Caller has found the entry we search
			return Some((inodeno, self.get_inode_entry(inodeno)));
		}
		if path.starts_with('/') {
			return self.get_inode_of(path);
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

	pub fn write_dentries(
		&mut self,
		block_no: u32,
		mut dentries: Vec<inode::Dentry>
	) -> Result<(), ()> {
		let mut block = self.read_block(block_no);
		let mut entry_start: usize = 0;
		let len = dentries.len();
		for i in 0..len {
			if dentries[i].dentry_size as usize > block.len() - entry_start {
				return Err(());
			}
			if i == len - 1 {
				dentries[i].dentry_size = (block.len() - entry_start) as u16;
			}
			let mut vec = Into::<Vec<u8>>::into(dentries[i].clone());
			let len = vec.len();
			for _ in len..dentries[i].dentry_size as usize {
				vec.push(0);
			}
			block[entry_start..entry_start + dentries[i].dentry_size as usize]
				.copy_from_slice(&vec);
			entry_start += dentries[i].dentry_size as usize;
		}
		self.write_block(block_no, &block);
		Ok(())
	}

	pub fn get_dentries(&self, block_no: u32) -> Vec<inode::Dentry> {
		let block = self.read_block(block_no);
		let mut dentries: Vec<inode::Dentry> = Vec::new();
		let mut entry_start = 0;
		while entry_start < block.len() {
			let dentry = inode::Dentry::from(&block[entry_start..block.len()]);
			// Block is empty
			if dentry.dentry_size == 0 {
				return dentries;
			}
			dentries.push(dentry.clone());
			entry_start = entry_start + dentry.dentry_size as usize;
		}
		dentries
	}

	// TODO: do remove_dentry recursive
	pub fn remove_dentry(
		&mut self,
		parent_inodeno: usize,
		dentry: inode::Dentry
	) {
		let inode = self.get_inode_entry(parent_inodeno);
		for block_no in inode.get_blocks_no() {
			let mut dentries = self.get_dentries(block_no);
			match dentries.iter().position(|x| x.inode == dentry.inode) {
				Some(index) => {
					// TODO: free blocks
					self.free_node(
						self.inode_to_bgroup(dentry.inode as u32) as usize,
						dentry.inode as usize
					);
					dentries.remove(index);
					match self.write_dentries(block_no, dentries) {
						Ok(()) => return,
						Err(()) => todo!()
					}
				},
				None => {}
			}
		}
	}

	// get new inode
	// get new blocks
	// fill inode informations
	// Nothing for link?
	// fill block information (dentries for directory, file content for a file)
	// create dentry pointing to the inode
	// write dentry on disk
	// write inode on disk
	// write new imap
	// write new bmap
	pub fn add_dentry(&mut self, inodeno: usize, dentry: inode::Dentry) {
		// Get block and inode
		let group = self.inode_to_bgroup(inodeno as u32) as usize;
		let bmap = self.read_block_map(group);
		let imap = self.read_inode_map(group);
		crate::dprintln!("Space {} {}", bmap.get_space().0, bmap.get_space().1);
		crate::dprintln!("Space {} {}", imap.get_space().0, imap.get_space().1);

		self.write_block_map(group, bmap);
		self.write_inode_map(group, imap);

		// Write new dentry on the parent dir
		let inode = self.get_inode_entry(inodeno);
		for block_no in inode.get_blocks_no() {
			let block = self.read_block(block_no);
			let mut dentries = self.get_dentries(block_no);
			match dentries.last_mut() {
				Some(last) => {
					let len = roundup(8 + last.name.len(), 4) as u16;
					if dentry.dentry_size <= last.dentry_size - len {
						let mut new_dentry = dentry.clone();
						new_dentry.dentry_size = last.dentry_size - len;
						last.dentry_size = len;
						dentries.push(new_dentry);
						if self.write_dentries(block_no, dentries).is_err() {
							continue;
						}
						return;
					}
				},
				None => {
					let mut new_dentry = dentry.clone();
					new_dentry.dentry_size = block.len() as u16;
					dentries.push(new_dentry);
					if self.write_dentries(block_no, dentries).is_err() {
						continue;
					}
					return;
				}
			}
		}
		todo!("not enough space to write ?");
	}
}

use crate::pci::ide;

pub fn read_superblock(
	diskno: u8,
	sector_size: usize
) -> Result<block::BaseSuperblock, u8> {
	// superblock is at index 1024 and 1024 bytes long
	let mut nb_sector = roundup(2048 / sector_size, 1) as usize;
	// sector_size > 2048
	if nb_sector == 0 {
		nb_sector = 1;
	}
	let buffer: Vec<u8> = vec![0; nb_sector * sector_size];

	IDE.lock().read_sectors(
		diskno,
		nb_sector as u8,
		0,
		buffer.as_ptr() as u32
	)?;
	let mut sblock = block::BaseSuperblock::from(&buffer[1024..1024 + 84]);
	if sblock.version().0 >= 1 {
		sblock.set_extension(block::ExtendedSuperblock::from(
			&buffer[1024 + 84..1024 + 236]
		));
	}
	Ok(sblock)
}

pub fn is_ext2(diskno: u8) -> bool {
	Ext2::new(diskno as u8).is_ok_and(|fs| fs.is_valid())
}

use crate::vec::Vec;

fn get_block_content(block: Vec<u8>, size: usize) -> Vec<char> {
	let buffer = &block[0..size];
	buffer.iter().map(|x| *x as char).collect()
}

/// Helper function to get content of a file.
/// Does not yet check if found entry is really a file.
/// Does not yet take into account file bigger than 4096
pub fn get_file_content(path: &str, inode: usize) -> Vec<char> {
	let ext2 = Ext2::new(unsafe { DISKNO as u8 })
		.expect("Disk is not a ext2 filesystem.");
	let opt = ext2.recurs_find(path, inode);
	match opt {
		None => Vec::new(),
		Some((_, inode)) => {
			if inode.tperm & inode::ITYPE_REGU == 0 {
				crate::kprintln!("'{}': Not a regular file.", path);
				return Vec::new();
			}
			let mut size = inode.size();
			let mut file: Vec<char> = Vec::new();
			for block_no in inode.get_blocks_no() {
				if inode::Inode::is_valid_block(block_no) {
					let block = ext2.read_block(block_no);
					if size > ext2.sblock.bsize() as u64 {
						file.append(&mut get_block_content(
							block,
							ext2.sblock.bsize()
						));
						size -= ext2.sblock.bsize() as u64;
					} else {
						file.append(&mut get_block_content(
							block,
							(inode.size() % ext2.sblock.bsize() as u64)
								as usize
						));
						size -= inode.size() % ext2.sblock.bsize() as u64;
						break;
					}
				}
			}
			for block_no in inode.get_sibp_blocks_no(&ext2) {
				if inode::Inode::is_valid_block(block_no) {
					let block = ext2.read_block(block_no);
					if size > ext2.sblock.bsize() as u64 {
						file.append(&mut get_block_content(
							block,
							ext2.sblock.bsize()
						));
						size -= ext2.sblock.bsize() as u64;
					} else {
						file.append(&mut get_block_content(
							block,
							(inode.size() % ext2.sblock.bsize() as u64)
								as usize
						));
						size -= inode.size() % ext2.sblock.bsize() as u64;
						break;
					}
				}
			}
			for block_no in inode.get_dibp_blocks_no(&ext2) {
				if inode::Inode::is_valid_block(block_no) {
					let block = ext2.read_block(block_no);
					if size > ext2.sblock.bsize() as u64 {
						file.append(&mut get_block_content(
							block,
							ext2.sblock.bsize()
						));
						size -= ext2.sblock.bsize() as u64;
					} else {
						file.append(&mut get_block_content(
							block,
							(inode.size() % ext2.sblock.bsize() as u64)
								as usize
						));
						size -= inode.size() % ext2.sblock.bsize() as u64;
						break;
					}
				}
			}
			for block_no in inode.get_tibp_blocks_no(&ext2) {
				if inode::Inode::is_valid_block(block_no) {
					let block = ext2.read_block(block_no);
					if size > ext2.sblock.bsize() as u64 {
						file.append(&mut get_block_content(
							block,
							ext2.sblock.bsize()
						));
						size -= ext2.sblock.bsize() as u64;
					} else {
						file.append(&mut get_block_content(
							block,
							(inode.size() % ext2.sblock.bsize() as u64)
								as usize
						));
						size -= inode.size() % ext2.sblock.bsize() as u64;
						break;
					}
				}
			}
			file
		}
	}
}

/// Helper function to list all entries in a directory
/// Does not yet check if found entry is a directory or not
pub fn list_dir(path: &str, inode: usize) -> crate::vec::Vec<inode::Dentry> {
	let ext2 = Ext2::new(unsafe { DISKNO as u8 })
		.expect("Disk is not a ext2 filesystem.");
	let inode = ext2.recurs_find(path, inode);
	return match inode {
		None => crate::vec::Vec::new(),
		Some((_, inode)) => {
			let mut dentries: crate::vec::Vec<inode::Dentry> =
				crate::vec::Vec::new();
			for block_no in inode.get_blocks_no() {
				let block = ext2.read_block(block_no);

				let mut entry_start: usize = 0;
				while entry_start != ext2.sblock.bsize() {
					let dentry =
						inode::Dentry::from(&block[entry_start..block.len()]);
					entry_start = entry_start + dentry.dentry_size as usize;
					dentries.push(dentry);
				}
			}
			dentries
		}
	};
}

pub fn create_file(path: &str, inode_no: usize) {
	let mut ext2 = Ext2::new(unsafe { DISKNO as u8 })
		.expect("Disk is not a ext2 filesystem.");
	let path = Path::new(path);
	let filename = path.file_name().unwrap();
	let binding = path.parent().unwrap();
	let parent = binding.as_str();
	let inode = ext2.recurs_find(&parent, inode_no);
	match inode {
		None => {
			crate::kprintln!("Path not found: {}", parent);
		},
		Some((inode_no, _)) => {
			let check_exist = ext2.recurs_find(&filename, inode_no);
			if check_exist.is_some() {
				crate::kprintln!("'{}' already exists.", filename);
				return;
			}
			let mut new_inode = inode::Inode::new();
			// perm: Regular file and 0644
			new_inode.tperm =
				inode::ITYPE_REGU
					| inode::IPERM_UREAD | inode::IPERM_UWRIT
					| inode::IPERM_GREAD | inode::IPERM_OREAD;
			// hardlinks: 1
			new_inode.count_hl = 1;
			new_inode.count_ds = 1;
			new_inode.size_lh = 0;
			new_inode.dbp[0] = ext2
				.alloc_block(ext2.inode_to_bgroup(inode_no as u32) as usize)
				as u32;
			let new_inode_no =
				ext2.alloc_node(ext2.inode_to_bgroup(inode_no as u32) as usize);
			ext2.write_inode(new_inode_no, &new_inode); // copy inode to fs
			let dentry: inode::Dentry = inode::Dentry {
				inode:       new_inode_no as u32,
				dentry_size: roundup(8 + filename.len(), 4) as u16,
				name_length: filename.len() as u8,
				r#type:      inode::Dtype::Regular as u8,
				name:        filename.to_string()
			};
			ext2.add_dentry(inode_no, dentry);
		}
	}
}

pub fn remove_file(path: &str, inode_no: usize) {
	let mut ext2 = Ext2::new(unsafe { DISKNO as u8 })
		.expect("Disk is not a ext2 filesystem.");
	let path = Path::new(path);
	let filename = path.file_name().unwrap();
	let binding = path.parent().unwrap();
	let parent = binding.as_str();
	let inode = ext2.recurs_find(&parent, inode_no);
	match inode {
		None => {
			crate::kprintln!("Path not found: {}", parent);
		},
		Some((inode_no, _)) => {
			// TODO: Check for only file and not recursive delete ?
			let dentry = ext2.dentry_find(inode_no, &filename);
			if dentry.is_none() {
				crate::kprintln!("'{}' does not exist.", filename);
				return;
			}
			ext2.remove_dentry(inode_no, dentry.unwrap());
		}
	}
}

/// Helper function to create a folder at a given path
pub fn create_dir(path: &str, inode_no: usize) {
	let mut ext2 = Ext2::new(unsafe { DISKNO as u8 })
		.expect("Disk is not a ext2 filesystem.");
	let path = Path::new(path);
	let new_dir = path.file_name().unwrap();
	let binding = path.parent().unwrap();
	let parent = binding.as_str();
	let inode = ext2.recurs_find(&parent, inode_no);
	match inode {
		None => {
			crate::kprintln!("Path not found: {}", parent);
		},
		Some((inode_no, mut inode)) => {
			let check_exist = ext2.recurs_find(&new_dir, inode_no);
			if check_exist.is_some() {
				crate::kprintln!("'{}' already exists.", new_dir);
				return;
			}
			let mut new_inode = inode::Inode::new();
			// perm: directory and 0755
			new_inode.tperm =
				inode::ITYPE_DIR
					| inode::IPERM_UREAD | inode::IPERM_UWRIT
					| inode::IPERM_UEXEC | inode::IPERM_GREAD
					| inode::IPERM_GEXEC | inode::IPERM_OREAD
					| inode::IPERM_OEXEC;
			// hardlinks: directory and '.'
			new_inode.count_hl = 2;
			new_inode.count_ds = 2;
			new_inode.size_lh = ext2.sblock.bsize() as u32;
			new_inode.dbp[0] = ext2
				.alloc_block(ext2.inode_to_bgroup(inode_no as u32) as usize)
				as u32;
			// Allocate inode
			let new_inode_no =
				ext2.alloc_node(ext2.inode_to_bgroup(inode_no as u32) as usize);
			ext2.write_inode(new_inode_no, &new_inode); // copy inode to fs
			let dentry: inode::Dentry = inode::Dentry {
				inode:       new_inode_no as u32,
				dentry_size: roundup(8 + new_dir.len(), 4) as u16,
				name_length: new_dir.len() as u8,
				r#type:      inode::Dtype::Directory as u8,
				name:        new_dir.to_string()
			};
			ext2.add_dentry(inode_no, dentry);
			// Create . and ..
			let dot_inode_no = ext2
				.alloc_node(ext2.inode_to_bgroup(new_inode_no as u32) as usize);
			ext2.write_inode(dot_inode_no, &new_inode); // copy actual inode
			let dentry: inode::Dentry = inode::Dentry {
				inode:       dot_inode_no as u32,
				dentry_size: roundup(8 + ".".len(), 4) as u16,
				name_length: ".".len() as u8,
				r#type:      inode::Dtype::Directory as u8,
				name:        ".".to_string()
			};
			ext2.add_dentry(new_inode_no, dentry);
			let dotdot_inode_no = ext2
				.alloc_node(ext2.inode_to_bgroup(new_inode_no as u32) as usize);
			ext2.write_inode(dotdot_inode_no, &inode); // copy previous inode
			let dentry: inode::Dentry = inode::Dentry {
				inode:       dotdot_inode_no as u32,
				dentry_size: roundup(8 + "..".len(), 4) as u16,
				name_length: "..".len() as u8,
				r#type:      inode::Dtype::Directory as u8,
				name:        "..".to_string()
			};
			ext2.add_dentry(new_inode_no, dentry);
			inode.count_hl += 1; // add 1 hard-link (..) to parent
			ext2.write_inode(inode_no, &inode); // copy inode
		}
	}
}

pub fn show_inode_info(path: &str, inode_no: usize) {
	let ext2 = Ext2::new(unsafe { DISKNO as u8 })
		.expect("Disk is not a ext2 filesystem.");
	let inode = ext2.recurs_find(&path, inode_no);
	match inode {
		None => {
			crate::kprintln!("Path not found: {}", path);
		},
		Some((inode_no, inode)) => {
			crate::kprint!("Inode: {:<4} ", inode_no);
			crate::kprint!(
				"Type: {:<12}",
				match inode.tperm {
					x if x & inode::ITYPE_FIFO != 0 => "fifo",
					x if x & inode::ITYPE_CHARDEV != 0 => "chardev",
					x if x & inode::ITYPE_DIR != 0 => "directory",
					x if x & inode::ITYPE_BLOCK != 0 => "block",
					x if x & inode::ITYPE_REGU != 0 => "regular",
					x if x & inode::ITYPE_SYMF != 0 => "symbolic",
					x if x & inode::ITYPE_SOCK != 0 => "sock",
					_ => "unknown"
				}
			);
			// TODO: setuid bit
			crate::kprint!("Mode:  0{:3o}   ", inode.tperm & 0o777);
			crate::kprintln!("Flags: {:#x}", inode.flags);

			crate::kprint!("Generation: {:<4} ", inode.gen_no);
			// TODO: Version
			crate::kprintln!("Version: {:#010x}:{:08x}", 0x0, 0x0);
			crate::kprint!("User:  {:<6} ", inode.uid);
			crate::kprint!("Group:  {:<6} ", inode.gid);
			// TODO: idk
			crate::kprint!("Project:     0   ");
			crate::kprintln!("Size: {}", inode.size());
			crate::kprintln!("File ACL: {}", inode.facl);
			crate::kprint!("Links: {:<3} ", inode.count_hl);
			crate::kprintln!("Blockcount: {}", inode.count_ds);
			crate::kprint!("Fragment:  Address: {:<4} ", inode.block_addr);
			// TODO: idk
			crate::kprint!("Number: {:<4} ", 0);
			crate::kprintln!("Size: {}", inode.size_uh);
			crate::kprintln!(
				" ctime: {:#010x}:{:08x} -- {}",
				inode.creatt,
				0,
				crate::time::ctime(inode.creatt)
			);
			crate::kprintln!(
				" atime: {:#010x}:{:08x} -- {}",
				inode.lat,
				0,
				crate::time::ctime(inode.lat)
			);
			crate::kprintln!(
				" mtime: {:#010x}:{:08x} -- {}",
				inode.lmt,
				0,
				crate::time::ctime(inode.lmt)
			);
			// TODO: idk
			crate::kprintln!(
				"crtime: {:#010x}:{:08x} -- {}",
				0x0,
				0,
				crate::time::ctime(0x0)
			);
			// TODO: idk
			crate::kprintln!("Size of extra inode fields: {}", 0);
			crate::kprintln!("BLOCKS:");
			let mut index = 0;
			let mut total = 0;
			let print = |mut index: usize, blocks_no: Vec<u32>| -> usize {
				let len = blocks_no.len();
				let mut i = 0;
				while i < len {
					let mut stop = i;
					while stop + 1 < len
						&& blocks_no[stop] + 1 == blocks_no[stop + 1]
					{
						stop += 1;
					}
					if index != 0 {
						crate::kprint!(", ");
					}
					if stop != i {
						crate::kprint!(
							"({}-{}):{}-{}",
							index,
							index + stop - i,
							blocks_no[i],
							blocks_no[stop]
						);
					} else {
						crate::kprint!("({}):{}", index, blocks_no[i]);
					}
					index += stop - i + 1;
					i = stop + 1;
				}
				i
			};
			let blocks_no = inode.get_blocks_no();
			let ret = print(index, blocks_no);
			index += ret;
			total += ret;
			if inode.sibp != 0 {
				crate::kprint!(", (IND): {}", inode.sibp);
				total += 1;
				let ret = print(index, inode.get_sibp_blocks_no(&ext2));
				total += ret;
				index += ret;
			}
			if inode.dibp != 0 {
				crate::kprint!(", (DIND): {}", inode.dibp);
				total += 1;
				// Get (IND)
				for sibp in inode::Inode::_get_sibp_blocks_no(inode.dibp, &ext2)
				{
					crate::kprint!(", (IND): {}", sibp);
					total += 1;
					let ret = print(
						index,
						inode::Inode::_get_sibp_blocks_no(sibp, &ext2)
					);
					total += ret;
					index += ret;
				}
			}
			if inode.tibp != 0 {
				crate::kprint!(", (TIND): {} ", inode.tibp);
				total += 1;
				for dibp in inode::Inode::_get_sibp_blocks_no(inode.tibp, &ext2)
				{
					crate::kprint!(", (DIND): {}", dibp);
					total += 1;
					for sibp in inode::Inode::_get_sibp_blocks_no(dibp, &ext2) {
						crate::kprint!(", (IND): {}", sibp);
						total += 1;
						let ret = print(
							index,
							inode::Inode::_get_sibp_blocks_no(sibp, &ext2)
						);
						total += ret;
						index += ret;
					}
				}
			}
			crate::kprintln!("\nTOTAL: {}", total);
		}
	};
}
