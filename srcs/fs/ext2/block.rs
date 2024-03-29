// mount_no indicate the number of mount since last fsck
// mount_no_max indicate the number of mount between each fsck
/// Superblock always take 1024 bytes with/without Extended block
#[derive(Default, Debug, PartialEq, Eq)]
pub struct BaseSuperblock {
	/// Total number of inodes in file system
	inode_count:          u32,
	/// Total number of blocks in file system
	blocks_count:         u32,
	/// Number of blocks reserved for superuser (see offset 80)
	rblocks_num:          u32,
	/// Total number of unallocated blocks
	pub blocks_unalloc:   u32,
	/// Total number of unallocated inodes
	pub inode_unalloc:    u32,
	/// Block number of the block containing the superblock (also the starting block number, NOT always zero.)
	pub superblock_block: u32,
	/// log2 (block size) - 10. (In other words, the number to shift 1,024 to the left by to obtain the block size)
	block_size:           u32,
	/// log2 (fragment size) - 10. (In other words, the number to shift 1,024 to the left by to obtain the fragment size)
	frag_size:            u32,
	/// Number of blocks in each block group
	bgroup_bno:           u32,
	/// Number of fragments in each block group
	bgroup_fno:           u32,
	/// Number of inodes in each block group
	bgroup_ino:           u32,
	/// Last mount time (in POSIX time)
	last_mt:              u32,
	/// Last written time (in POSIX time)
	last_wt:              u32,
	/// Number of times the volume has been mounted since its last consistency check (fsck)
	mount_no:             u16,
	/// Number of mounts allowed before a consistency check (fsck) must be done
	mount_no_max:         u16,
	/// Ext2 signature (0xef53), used to help confirm the presence of Ext2 on a volume
	ext2_sig:             u16,
	/// File system state (see below)
	fs_state:             u16,
	/// What to do when an error is detected (see below)
	err_handle:           u16,
	/// Minor portion of version (combine with Major portion below to construct full version field)
	minor:                u16,
	/// POSIX time of last consistency check (fsck)
	last_fsck:            u32,
	/// Interval (in POSIX time) between forced consistency checks (fsck)
	fsck_interval:        u32,
	/// Operating system ID from which the filesystem on this volume was created (see below)
	osid:                 u32,
	/// Major portion of version (combine with Minor portion above to construct full version field)
	major:                u32,
	/// User ID that can use reserved blocks
	pub uid:              u16,
	/// Group ID that can use reserved blocks
	pub gid:              u16,
	extension:            Option<ExtendedSuperblock>
}

impl BaseSuperblock {
	pub fn sig(&self) -> u16 {
		self.ext2_sig
	}

	pub fn version(&self) -> (u32, u16) {
		(self.major, self.minor)
	}

	pub fn bsize(&self) -> usize {
		1024 << self.block_size
	}
	pub fn inode_size(&self) -> u16 {
		if self.major < 1 {
			return 128;
		}
		match &self.extension {
			Some(ext) => ext.inode_size,
			_ => panic!("Superblock extension not set with major >= 1")
		}
	}

	pub fn block_per_grp(&self) -> u32 {
		self.bgroup_bno
	}

	pub fn inode_per_grp(&self) -> u32 {
		self.bgroup_ino
	}

	pub fn inode_count(&self) -> u32 {
		self.inode_count
	}

	pub fn block_count(&self) -> u32 {
		self.blocks_count
	}

	pub fn block_grp_count(&self) -> u32 {
		(self.blocks_count / self.bgroup_bno)
			+ (self.blocks_count % self.bgroup_bno != 0) as u32
	}

	pub fn set_extension(&mut self, extension: ExtendedSuperblock) {
		self.extension = Some(extension);
	}
}

impl From<&[u8]> for BaseSuperblock {
	fn from(buffer: &[u8]) -> Self {
		if buffer.len() != 84 {
			panic!("Wrong size while converting slice to Superblock");
		}
		// Safe beceause len is forced to be 84
		Self {
			inode_count:      u32::from_le_bytes(
				buffer[0..4].try_into().unwrap()
			),
			blocks_count:     u32::from_le_bytes(
				buffer[4..8].try_into().unwrap()
			),
			rblocks_num:      u32::from_le_bytes(
				buffer[8..12].try_into().unwrap()
			),
			blocks_unalloc:   u32::from_le_bytes(
				buffer[12..16].try_into().unwrap()
			),
			inode_unalloc:    u32::from_le_bytes(
				buffer[16..20].try_into().unwrap()
			),
			superblock_block: u32::from_le_bytes(
				buffer[20..24].try_into().unwrap()
			),
			block_size:       u32::from_le_bytes(
				buffer[24..28].try_into().unwrap()
			),
			frag_size:        u32::from_le_bytes(
				buffer[28..32].try_into().unwrap()
			),
			bgroup_bno:       u32::from_le_bytes(
				buffer[32..36].try_into().unwrap()
			),
			bgroup_fno:       u32::from_le_bytes(
				buffer[36..40].try_into().unwrap()
			),
			bgroup_ino:       u32::from_le_bytes(
				buffer[40..44].try_into().unwrap()
			),
			last_mt:          u32::from_le_bytes(
				buffer[44..48].try_into().unwrap()
			),
			last_wt:          u32::from_le_bytes(
				buffer[48..52].try_into().unwrap()
			),
			mount_no:         u16::from_le_bytes(
				buffer[52..54].try_into().unwrap()
			),
			mount_no_max:     u16::from_le_bytes(
				buffer[54..56].try_into().unwrap()
			),
			ext2_sig:         u16::from_le_bytes(
				buffer[56..58].try_into().unwrap()
			),
			fs_state:         u16::from_le_bytes(
				buffer[58..60].try_into().unwrap()
			),
			err_handle:       u16::from_le_bytes(
				buffer[60..62].try_into().unwrap()
			),
			minor:            u16::from_le_bytes(
				buffer[62..64].try_into().unwrap()
			),
			last_fsck:        u32::from_le_bytes(
				buffer[64..68].try_into().unwrap()
			),
			fsck_interval:    u32::from_le_bytes(
				buffer[68..72].try_into().unwrap()
			),
			osid:             u32::from_le_bytes(
				buffer[72..76].try_into().unwrap()
			),
			major:            u32::from_le_bytes(
				buffer[76..80].try_into().unwrap()
			),
			uid:              u16::from_le_bytes(
				buffer[80..82].try_into().unwrap()
			),
			gid:              u16::from_le_bytes(
				buffer[82..84].try_into().unwrap()
			),
			extension:        None
		}
	}
}

impl BaseSuperblock {
	pub fn into_boxed_slice(&self) -> crate::alloc::boxed::Box<[u8]> {
		let mut vec = crate::alloc::vec::Vec::new();

		vec.extend_from_slice(&self.inode_count.to_le_bytes());
		vec.extend_from_slice(&self.blocks_count.to_le_bytes());
		vec.extend_from_slice(&self.rblocks_num.to_le_bytes());
		vec.extend_from_slice(&self.blocks_unalloc.to_le_bytes());
		vec.extend_from_slice(&self.inode_unalloc.to_le_bytes());
		vec.extend_from_slice(&self.superblock_block.to_le_bytes());
		vec.extend_from_slice(&self.block_size.to_le_bytes());
		vec.extend_from_slice(&self.frag_size.to_le_bytes());
		vec.extend_from_slice(&self.bgroup_bno.to_le_bytes());
		vec.extend_from_slice(&self.bgroup_fno.to_le_bytes());
		vec.extend_from_slice(&self.bgroup_ino.to_le_bytes());
		vec.extend_from_slice(&self.last_mt.to_le_bytes());
		vec.extend_from_slice(&self.last_wt.to_le_bytes());
		vec.extend_from_slice(&self.mount_no.to_le_bytes());
		vec.extend_from_slice(&self.mount_no_max.to_le_bytes());
		vec.extend_from_slice(&self.ext2_sig.to_le_bytes());
		vec.extend_from_slice(&self.fs_state.to_le_bytes());
		vec.extend_from_slice(&self.err_handle.to_le_bytes());
		vec.extend_from_slice(&self.minor.to_le_bytes());
		vec.extend_from_slice(&self.last_fsck.to_le_bytes());
		vec.extend_from_slice(&self.fsck_interval.to_le_bytes());
		vec.extend_from_slice(&self.osid.to_le_bytes());
		vec.extend_from_slice(&self.major.to_le_bytes());
		vec.extend_from_slice(&self.uid.to_le_bytes());
		vec.extend_from_slice(&self.gid.to_le_bytes());

		vec.into_boxed_slice()
	}
}

use core::fmt;
impl fmt::Display for BaseSuperblock {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let lwt = crate::time::ctime(self.last_wt);
		let lct = crate::time::ctime(self.last_fsck);
		write!(
			f,
			"Superblock: {{
Sig: {:#x}
Version: {}.{}
Block Size: {:#x}
Last write time: {lwt},
Last check time: {lct},
}}",
			self.ext2_sig,
			self.major,
			self.minor,
			1024 << self.block_size,
		)
	}
}

const FSSTATE_CLEAN: u16 = 1;
const FSSTATE_ERROR: u16 = 2;

const FSERROR_IGN: u16 = 1;
const FSERROR_MRO: u16 = 2;
const FSERROR_KPAN: u16 = 3;

const FSCREAT_LINUX: u16 = 0;
const FSCREAT_GNU: u16 = 1;
const FSCREAT_MASIX: u16 = 2;
const FSCREAT_FBSD: u16 = 3;
const FSCREAT_OTHER: u16 = 4;

/// Present if Major >= 1
/// Bytes from 236 to 1023 aren't used
#[derive(Debug, PartialEq, Eq)]
pub struct ExtendedSuperblock {
	/// First non-reserved inode in file system. (In versions < 1.0, this is fixed as 11)
	first_inode:           u32,
	/// Size of each inode structure in bytes. (In versions < 1.0, this is fixed as 128)
	inode_size:            u16,
	/// Block group that this superblock is part of (if backup copy)
	bgroup_superblock:     u16,
	/// Optional features present (features that are not required to read or write, but usually result in a performance increase. see below)
	opt_features:          u32,
	/// Required features present (features that are required to be supported to read or write. see below)
	req_features:          u32,
	/// Features that if not supported, the volume must be mounted read-only see below)
	ro_features:           u32,
	/// File system ID (what is output by blkid)
	fsid:                  [u8; 16],
	/// Volume name (C-style string: characters terminated by a 0 byte)
	vol_name:              [u8; 16],
	/// Path volume was last mounted to (C-style string: characters terminated by a 0 byte)
	last_path:             [u8; 64],
	/// Compression algorithms used (see Required features above)
	compr:                 u32,
	/// Number of blocks to preallocate for files
	prealloc_blocks_files: u8,
	/// Number of blocks to preallocate for directories
	prealloc_block_dir:    u8,
	/// (Unused)
	unused:                u16,
	/// Journal ID (same style as the File system ID above)
	journ_id:              [u8; 16],
	/// Journal inode
	journ_inode:           u32,
	/// Journal device
	journ_dev:             u32,
	/// Head of orphan inode list
	orphan_inode_lst:      u32
}

impl From<&[u8]> for ExtendedSuperblock {
	fn from(buffer: &[u8]) -> Self {
		if buffer.len() != 152 {
			panic!("Wrong size for Extended Super block parsing");
		}
		let mut exblock = Self {
			first_inode:           u32::from_le_bytes(
				buffer[0..4].try_into().unwrap()
			),
			inode_size:            u16::from_le_bytes(
				buffer[4..6].try_into().unwrap()
			),
			bgroup_superblock:     u16::from_le_bytes(
				buffer[6..8].try_into().unwrap()
			),
			opt_features:          u32::from_le_bytes(
				buffer[8..12].try_into().unwrap()
			),
			req_features:          u32::from_le_bytes(
				buffer[12..16].try_into().unwrap()
			),
			ro_features:           u32::from_le_bytes(
				buffer[16..20].try_into().unwrap()
			),
			fsid:                  [0; 16],
			vol_name:              [0; 16],
			last_path:             [0; 64],
			compr:                 u32::from_le_bytes(
				buffer[116..120].try_into().unwrap()
			),
			prealloc_blocks_files: buffer[120],
			prealloc_block_dir:    buffer[121],
			unused:                0x0,
			journ_id:              [0; 16],
			journ_inode:           u32::from_le_bytes(
				buffer[140..144].try_into().unwrap()
			),
			journ_dev:             u32::from_le_bytes(
				buffer[144..148].try_into().unwrap()
			),
			orphan_inode_lst:      u32::from_le_bytes(
				buffer[148..152].try_into().unwrap()
			)
		};
		exblock.fsid[0..16].copy_from_slice(&buffer[20..36]);
		exblock.vol_name[0..16].copy_from_slice(&buffer[36..52]);
		exblock.last_path[0..64].copy_from_slice(&buffer[52..116]);
		exblock.journ_id[0..16].copy_from_slice(&buffer[124..140]);

		exblock
	}
}

const OPTFEAT_PREALLOC: u32 = 0x0001;
const OPTFEAT_AFSSERV: u32 = 0x0002;
const OPTFEAT_JOURN: u32 = 0x0004;
const OPTFEAT_INODEEXT: u32 = 0x0008;
const OPTFEAT_RESIZE: u32 = 0x0010;
const OPTFEAT_HASH_INDEX: u32 = 0x0020;

const REQFEAT_COMPR: u32 = 0x0001;
const REQFEAT_DE_TYPEFIELD: u32 = 0x0002;
const REQFEAT_REPLAY_JOURN: u32 = 0x0004;
const REQFEAT_JOURN_DEV: u32 = 0x0008;

const ROFEAT_SPARS: u32 = 0x0001;
const ROFEAT_64B: u32 = 0x0002;
const ROFEAT_DIR_BTRE: u32 = 0x0004;
