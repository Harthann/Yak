mod bitmap;
pub mod block;
mod gdt;
pub mod inode;

const SECTOR_SIZE: u32 = 512;
const DISKNO: u8 = 1;

use crate::pci::ide::IDE;
use crate::utils::math::roundup;
use crate::string::{String, ToString};

/// Current read/write use entire block to perform operations
/// In the filesystem created to test it this means we read/write 16 sectors for each operations
/// This is pretty ineffective and will probably need optimisation in later version
pub struct Ext2 {
	pub sblock: block::BaseSuperblock
}

impl Ext2 {
	// Temporary hardcoded values (disk number, sector size etc)
	pub fn new() -> Self {
		Self { sblock: read_superblock() }
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
	/// let ext2 = Ext2::new();
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
	/// let ext2 = Ext2::new();
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
	/// let ext2 = Ext2::new();
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
		let buffer: [u8; SECTOR_SIZE as usize] = [0; SECTOR_SIZE as usize];
		let bsize = self.sblock.bsize();
		let sector_per_block = bsize / SECTOR_SIZE as usize;

		let sector_no = bsize / SECTOR_SIZE as usize;
		let mut block: crate::vec::Vec<u8> = crate::vec::Vec::new();
		for i in 0..sector_no {
			unsafe {
				IDE.lock().read_sectors(
					DISKNO,
					1,
					(block_no * sector_per_block as u32) + i as u32,
					buffer.as_ptr() as u32
				);
				block.extend_from_slice(&buffer);
			}
		}
		block
	}

	fn write_block(&mut self, block_no: u32, block: &[u8]) {
		let bsize = self.sblock.bsize();
		let sector_per_block = bsize / SECTOR_SIZE as usize;

		let sector_no = bsize / SECTOR_SIZE as usize;
		unsafe {
			IDE.lock().write_sectors(
				DISKNO,
				sector_no as u8,
				block_no * sector_per_block as u32,
				block.as_ptr() as u32
			);
		};
	}
	fn write_slice(&mut self, block_no: u32, offset: usize, slice: &[u8]) {
		let mut block = self.read_block(block_no);
		crate::kprintln!("offset: {}", offset);
		crate::kprintln!("slice.len(): {}", slice.len());
		crate::kprintln!("block.len(): {}", block.len());
		block[offset..offset + slice.len()].copy_from_slice(slice);
		self.write_block(block_no, &block);
	}

	fn write_inode(&mut self, inodeno: usize, inode: inode::Inode) {
		todo!()
	}

	/// Read disk to recover Group Descriptor Table entry given an index
	fn get_gdt_entry(&self, entry: usize) -> gdt::GdtEntry {
		let entry_per_sector =
			SECTOR_SIZE as usize / core::mem::size_of::<gdt::GdtEntry>();
		let sector_no = entry / entry_per_sector;

		let block = self.read_block(1 + (self.sblock.bsize() == 1024) as u32);
		let entry_start = (sector_no * SECTOR_SIZE as usize)
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
		self.write_slice(nodeno as u32, 0, &*self.sblock.into_boxed_slice());
		self.write_inode_map(group, map);
		nodeno
	}

	pub fn alloc_block(&mut self, group: usize) -> usize {
		let mut map = self.read_block_map(group);
		let nodeno = map.get_free_node().unwrap();
		self.sblock.blocks_unalloc -= 1;
		self.write_slice(nodeno as u32, 0, &*self.sblock.into_boxed_slice());
		self.write_block_map(group, map);
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
	/// let ext2 = Ext2::new();
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
	/// let ext2 = Ext2::new();
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
	/// # Arguments
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
	pub fn add_dentry(&mut self, inodeno: usize, mut dentry: inode::Dentry) {
		// Get block and inode
		let group = self.inode_to_bgroup(inodeno as u32) as usize;
		let mut bmap = self.read_block_map(group);
		let mut imap = self.read_inode_map(group);
		crate::kprintln!("Space {} {}", bmap.get_space().0, bmap.get_space().1);
		crate::kprintln!("Space {} {}", imap.get_space().0, imap.get_space().1);
		let dentry_block = bmap.get_free_node().unwrap();
		let dentry_inode = imap.get_free_node().unwrap();
		dentry.inode = dentry_inode as u32;

		self.write_block_map(group, bmap);
		self.write_inode_map(group, imap);

		// Write new dentry on the parent dir
		let inode = self.get_inode_entry(inodeno);
		for block_no in inode.get_blocks_no() {
			let mut block = self.read_block(block_no);

			let mut entry_start = 0;
			while entry_start < block.len() {
				crate::kprintln!("Blocklen {} {}", block.len(), entry_start);
				let mut tmp =
					inode::Dentry::from(&block[entry_start..block.len()]);
				let calculated = roundup(tmp.name_length + 8, 4);
				if (calculated as usize) < tmp.dentry_size as usize {
					if entry_start
						+ calculated as usize + (dentry.dentry_size as usize)
						< block.len() as usize
					{
						// rewrite tmp but with actual size
						tmp.dentry_size = calculated as u16;
						let mut vec = Into::<Vec<u8>>::into(tmp);
						while vec.len() != calculated as usize {
							vec.push(0);
						}
						block[entry_start
							..entry_start + calculated as usize]
							.copy_from_slice(
								vec.as_slice()
							);
						entry_start = entry_start + calculated as usize;
						let dentrysize =
							roundup(dentry.name_length + 8, 4) as usize;
						// write our entry but with the block rest
						let calculated = block.len() as u16 - entry_start as u16;
						dentry.dentry_size = calculated;
						let mut vec = Into::<Vec<u8>>::into(dentry);
						while vec.len() != dentrysize as usize {
							vec.push(0);
						}
						block[entry_start..entry_start + dentrysize]
							.copy_from_slice(
								vec.as_slice()
							);
						for block_no in inode.get_blocks_no() {
							self.write_block(block_no, block.as_slice());
						}
						return;
					}
				}
				entry_start = entry_start + tmp.dentry_size as usize;
			}
		}
		crate::kprintln!("no write");
	}
}

pub fn read_superblock() -> block::BaseSuperblock {
	let buffer: [u8; SECTOR_SIZE as usize] = [0; SECTOR_SIZE as usize];

	unsafe {
		IDE.lock()
			.read_sectors(DISKNO, 1, 2, buffer.as_ptr() as u32);
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

fn get_block_content(block: Vec<u8>, size: usize) -> Vec<char> {
	let buffer = &block[0..size];
	buffer.iter().map(|x| *x as char).collect()
}

/// Helper function to get content of a file.
/// Does not yet check if found entry is really a file.
/// Does not yet take into account file bigger than 4096
pub fn get_file_content(path: &str, inode: usize) -> Vec<char> {
	let ext2 = Ext2::new();
	let opt = ext2.recurs_find(path, inode);
	match opt {
		None => Vec::new(),
		Some((_, inode)) => {
			let mut file: Vec<char> = Vec::new();
			let blocks_no = inode.get_blocks_no();
			let nb_blocks: usize =
				(inode.size() / ext2.sblock.bsize() as u64 + 1) as usize;
			let n_loop = if nb_blocks > 12 { 12 } else { nb_blocks };
			let nb_singly_block =
				ext2.sblock.bsize() / core::mem::size_of::<u32>();
			for i in 0..nb_blocks {
				let block;
				if i < 12 {
					block = ext2.read_block(blocks_no[i as usize]);
				} else if i >= 12 && i < 12 + nb_singly_block {
					// Singly indirect block pointer
					let singly_block = ext2.read_block(inode.sibp);
					let off = (i - 12) * core::mem::size_of::<u32>();
					let block_no = u32::from_le_bytes(
						singly_block[off..off + core::mem::size_of::<u32>()]
							.try_into()
							.unwrap()
					);
					block = ext2.read_block(block_no);
				} else {
					// Doubly or Triply indirect block pointer
					todo!();
				}
				if i != nb_blocks - 1 {
					file.append(&mut get_block_content(
						block,
						ext2.sblock.bsize()
					));
				} else {
					file.append(&mut get_block_content(
						block,
						(inode.size() % ext2.sblock.bsize() as u64) as usize
					));
				}
			}
			file
		}
	}
}

/// Helper function to list all entries in a directory
/// Does not yet check if found entry is a directory or not
pub fn list_dir(path: &str, inode: usize) -> crate::vec::Vec<inode::Dentry> {
	let ext2 = Ext2::new();
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

/// Helper function to create a folder at a given path
pub fn create_dir(path: &str, inode_no: usize) {
	let mut ext2 = Ext2::new();
	let root = path.starts_with('/');
	let mut splited: Vec<&str> = path.split("/").collect();
	splited.retain(|a| a.len() != 0);
	let (to_create, mut path): (String, String) = match splited.pop() {
		Some(x) => {
			(x.to_string(), splited.join("/"))
		},
		None => {
			(splited.join("/").to_string(), "".to_string())
		}
	};
	if root {
		path.insert_str(0, "/");
	}
	crate::kprintln!("to_create: {}", to_create);
	crate::kprintln!("path: {}", path);
	let inode = ext2.recurs_find(&path, inode_no);
	match inode {
		None => {crate::kprintln!("Path not found: {}", path);}
		Some((inode_no, inode)) => {
			let new_inode = ext2.alloc_node(ext2.inode_to_bgroup(inode_no as u32) as usize);
			let dentry: inode::Dentry = inode::Dentry {
				inode: new_inode as u32,
				dentry_size: roundup(8 + to_create.len(), 4) as u16,
				name_length: to_create.len() as u8,
				r#type: inode::Dtype::Directory as u8,
				name: to_create
			};
			ext2.add_dentry(inode_no, dentry);
		}
	}
}
