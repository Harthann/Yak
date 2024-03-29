use crate::time;
use crate::vec::Vec;
use core::mem::size_of;

/// Like blocks, each inode has a numerical address.
/// It is extremely important to note that unlike block addresses, inode addresses start at 1.
/// With Ext2 versions prior to Major version 1, inodes 1 to 10 are reserved and
/// should be in an allocated state.
/// Starting with version 1, the first non-reserved inode is indicated via a field in the Superblock.
/// Of the reserved inodes, number 2 subjectively has the most significance as it is used for the root directory.
/// Inodes have a fixed size of either 128 for version 0 Ext2 file systems, or as dictated by the field in the Superblock for version 1 file systems.
/// All inodes reside in inode tables that belong to block groups.
/// Therefore, looking up an inode is simply a matter of determining which
/// block group it belongs to and indexing that block group's inode table.
#[derive(Debug, Default, Copy, Clone)]
pub struct Inode {
	/// Type and Permissions (see below)
	pub tperm:      u16,
	/// User ID
	pub uid:        u16,
	/// Lower 32 bits of size in bytes
	pub size_lh:    u32,
	/// Last Access Time (in POSIX time)
	pub lat:        u32,
	/// Creation Time (in POSIX time)
	pub creatt:     u32,
	/// Last Modification time (in POSIX time)
	pub lmt:        u32,
	/// Deletion time (in POSIX time)
	pub delt:       u32,
	/// Group ID
	pub gid:        u16,
	/// Count of hard links (directory entries) to this inode. When this reaches 0, the data blocks are marked as unallocated.
	pub count_hl:   u16,
	/// Count of disk sectors (not Ext2 blocks) in use by this inode, not counting the actual inode structure nor directory entries linking to the inode.
	pub count_ds:   u32,
	/// Flags (see below)
	pub flags:      u32,
	/// Operating System Specific value #1
	os_specific1:   u32,
	/// Direct Block Pointers
	pub dbp:        [u32; 12],
	/// Singly Indirect Block Pointer (Points to a block that is a list of block pointers to data)
	pub sibp:       u32,
	/// Doubly Indirect Block Pointer (Points to a block that is a list of block pointers to Singly Indirect Blocks)
	pub dibp:       u32,
	/// Triply Indirect Block Pointer (Points to a block that is a list of block pointers to Doubly Indirect Blocks)
	pub tibp:       u32,
	/// Generation number (Primarily used for NFS)
	pub gen_no:     u32,
	/// In Ext2 version 0, this field is reserved. In version >= 1, Extended attribute block (File ACL).
	pub facl:       u32,
	/// In Ext2 version 0, this field is reserved. In version >= 1, Upper 32 bits of file size (if feature bit set) if it's a file, Directory ACL if it's a directory
	pub size_uh:    u32,
	/// Block address of fragment
	pub block_addr: u32,
	/// Operating System Specific Value #2
	os_specific2:   [u8; 12]
}

impl Inode {
	pub fn new() -> Self {
		// TODO: not working: only get jiffies not unix timestamp
		let time = time::get_timestamp();
		Self {
			tperm:        0,
			uid:          0,
			size_lh:      0,
			lat:          time.second as u32,
			creatt:       time.second as u32,
			lmt:          time.second as u32,
			delt:         time.second as u32,
			gid:          0,
			count_hl:     1,
			count_ds:     1,
			flags:        0,
			os_specific1: 0,
			dbp:          [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
			sibp:         0,
			dibp:         0,
			tibp:         0,
			gen_no:       0,
			facl:         0,
			size_uh:      0,
			block_addr:   0,
			os_specific2: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
		}
	}

	pub fn size(&self) -> u64 {
		self.size_lh as u64 | ((self.size_uh as u64) << 32)
	}

	pub fn get_hardlinks(&self) -> u16 {
		self.count_hl
	}

	pub fn get_perms(&self) -> u16 {
		// Get only perms and ignore type
		self.tperm & 0x7777
	}

	pub fn is_dir(&self) -> bool {
		self.tperm & ITYPE_DIR != 0
	}

	pub fn is_valid_block(block_no: u32) -> bool {
		block_no != 0
	}

	fn get_blocks_no_from_u32_slice(slice: &[u32]) -> Vec<u32> {
		let mut blocks_no = Vec::new();
		for i in 0..slice.len() {
			if Inode::is_valid_block(slice[i]) {
				blocks_no.push(slice[i]);
			}
		}
		blocks_no
	}

	fn get_blocks_no_from_u8_slice(slice: &[u8]) -> Vec<u32> {
		let mut blocks_no: Vec<u32> = Vec::new();
		for i in (0..slice.len()).step_by(size_of::<u32>()) {
			let block_no = u32::from_le_bytes(
				slice[i..i + size_of::<u32>()].try_into().unwrap()
			);
			if Inode::is_valid_block(block_no) {
				blocks_no.push(block_no);
			}
		}
		blocks_no
	}

	pub fn get_blocks_no(&self) -> Vec<u32> {
		Inode::get_blocks_no_from_u32_slice(&self.dbp)
	}

	pub fn _get_sibp_blocks_no(sibp: u32, ext2: &super::Ext2) -> Vec<u32> {
		let singly_block = ext2.read_block(sibp);
		Inode::get_blocks_no_from_u8_slice(&singly_block)
	}

	pub fn get_sibp_blocks_no(&self, ext2: &super::Ext2) -> Vec<u32> {
		Inode::_get_sibp_blocks_no(self.sibp, ext2)
	}

	pub fn _get_dibp_blocks_no(dibp: u32, ext2: &super::Ext2) -> Vec<u32> {
		let doubly_block = ext2.read_block(dibp);
		let mut blocks_no: Vec<u32> = Vec::new();
		let singly_blocks = Inode::get_blocks_no_from_u8_slice(&doubly_block);
		for singly_block in singly_blocks {
			blocks_no.extend_from_slice(&Inode::_get_sibp_blocks_no(
				singly_block,
				ext2
			));
		}
		blocks_no
	}

	pub fn get_dibp_blocks_no(&self, ext2: &super::Ext2) -> Vec<u32> {
		Inode::_get_dibp_blocks_no(self.dibp, ext2)
	}

	pub fn _get_tibp_blocks_no(tibp: u32, ext2: &super::Ext2) -> Vec<u32> {
		let triply_block = ext2.read_block(tibp);
		let mut blocks_no: Vec<u32> = Vec::new();
		let doubly_blocks = Inode::get_blocks_no_from_u8_slice(&triply_block);
		for doubly_block in doubly_blocks {
			blocks_no.extend_from_slice(&Inode::_get_dibp_blocks_no(
				doubly_block,
				ext2
			));
		}
		blocks_no
	}

	pub fn get_tibp_blocks_no(&self, ext2: &super::Ext2) -> Vec<u32> {
		Inode::_get_tibp_blocks_no(self.tibp, ext2)
	}
}

impl From<&[u8]> for Inode {
	fn from(buffer: &[u8]) -> Self {
		if buffer.len() < 128 {
			panic!("Wrong size while converting slice to Superblock");
		}
		// Safe beceause len is forced to be 83
		let mut inode = Self {
			tperm:        u16::from_le_bytes(buffer[0..2].try_into().unwrap()),
			uid:          u16::from_le_bytes(buffer[2..4].try_into().unwrap()),
			size_lh:      u32::from_le_bytes(buffer[4..8].try_into().unwrap()),
			lat:          u32::from_le_bytes(buffer[8..12].try_into().unwrap()),
			creatt:       u32::from_le_bytes(
				buffer[12..16].try_into().unwrap()
			),
			lmt:          u32::from_le_bytes(
				buffer[16..20].try_into().unwrap()
			),
			delt:         u32::from_le_bytes(
				buffer[20..24].try_into().unwrap()
			),
			gid:          u16::from_le_bytes(
				buffer[24..26].try_into().unwrap()
			),
			count_hl:     u16::from_le_bytes(
				buffer[26..28].try_into().unwrap()
			),
			count_ds:     u32::from_le_bytes(
				buffer[28..32].try_into().unwrap()
			),
			flags:        u32::from_le_bytes(
				buffer[32..36].try_into().unwrap()
			),
			os_specific1: u32::from_le_bytes(
				buffer[36..40].try_into().unwrap()
			),
			dbp:          [
				u32::from_le_bytes(buffer[40..44].try_into().unwrap()),
				u32::from_le_bytes(buffer[44..48].try_into().unwrap()),
				u32::from_le_bytes(buffer[48..52].try_into().unwrap()),
				u32::from_le_bytes(buffer[52..56].try_into().unwrap()),
				u32::from_le_bytes(buffer[56..60].try_into().unwrap()),
				u32::from_le_bytes(buffer[60..64].try_into().unwrap()),
				u32::from_le_bytes(buffer[64..68].try_into().unwrap()),
				u32::from_le_bytes(buffer[68..72].try_into().unwrap()),
				u32::from_le_bytes(buffer[72..76].try_into().unwrap()),
				u32::from_le_bytes(buffer[76..80].try_into().unwrap()),
				u32::from_le_bytes(buffer[80..84].try_into().unwrap()),
				u32::from_le_bytes(buffer[84..88].try_into().unwrap())
			],
			sibp:         u32::from_le_bytes(
				buffer[88..92].try_into().unwrap()
			),
			dibp:         u32::from_le_bytes(
				buffer[92..96].try_into().unwrap()
			),
			tibp:         u32::from_le_bytes(
				buffer[96..100].try_into().unwrap()
			),
			gen_no:       u32::from_le_bytes(
				buffer[100..104].try_into().unwrap()
			),
			facl:         u32::from_le_bytes(
				buffer[104..108].try_into().unwrap()
			),
			size_uh:      u32::from_le_bytes(
				buffer[108..112].try_into().unwrap()
			),
			block_addr:   u32::from_le_bytes(
				buffer[112..116].try_into().unwrap()
			),
			os_specific2: [0; 12]
		};
		inode.os_specific2[0..12].copy_from_slice(&buffer[116..128]);
		inode
	}
}

impl Into<crate::alloc::vec::Vec<u8>> for Inode {
	fn into(self) -> crate::alloc::vec::Vec<u8> {
		let mut v = crate::alloc::vec::Vec::new();
		v.extend_from_slice(&self.tperm.to_le_bytes());
		v.extend_from_slice(&self.uid.to_le_bytes());
		v.extend_from_slice(&self.size_lh.to_le_bytes());
		v.extend_from_slice(&self.lat.to_le_bytes());
		v.extend_from_slice(&self.creatt.to_le_bytes());
		v.extend_from_slice(&self.lmt.to_le_bytes());
		v.extend_from_slice(&self.delt.to_le_bytes());
		v.extend_from_slice(&self.gid.to_le_bytes());
		v.extend_from_slice(&self.count_hl.to_le_bytes());
		v.extend_from_slice(&self.count_ds.to_le_bytes());
		v.extend_from_slice(&self.flags.to_le_bytes());
		v.extend_from_slice(&self.os_specific1.to_le_bytes());
		v.extend_from_slice(&self.dbp[0].to_le_bytes());
		v.extend_from_slice(&self.dbp[1].to_le_bytes());
		v.extend_from_slice(&self.dbp[2].to_le_bytes());
		v.extend_from_slice(&self.dbp[3].to_le_bytes());
		v.extend_from_slice(&self.dbp[4].to_le_bytes());
		v.extend_from_slice(&self.dbp[5].to_le_bytes());
		v.extend_from_slice(&self.dbp[6].to_le_bytes());
		v.extend_from_slice(&self.dbp[7].to_le_bytes());
		v.extend_from_slice(&self.dbp[8].to_le_bytes());
		v.extend_from_slice(&self.dbp[9].to_le_bytes());
		v.extend_from_slice(&self.dbp[10].to_le_bytes());
		v.extend_from_slice(&self.dbp[11].to_le_bytes());
		v.extend_from_slice(&self.sibp.to_le_bytes());
		v.extend_from_slice(&self.dibp.to_le_bytes());
		v.extend_from_slice(&self.tibp.to_le_bytes());
		v.extend_from_slice(&self.gen_no.to_le_bytes());
		v.extend_from_slice(&self.facl.to_le_bytes());
		v.extend_from_slice(&self.size_uh.to_le_bytes());
		v.extend_from_slice(&self.block_addr.to_le_bytes());
		v.extend_from_slice(&self.os_specific2);
		v
	}
}

use core::fmt;
impl fmt::Display for Inode {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let size: u64 = self.size();

		let mut perms: crate::string::String = crate::string::String::new();
		if self.tperm & IPERM_OREAD != 0 {
			perms.push('r');
		} else {
			perms.push('-');
		}
		if self.tperm & IPERM_OWRIT != 0 {
			perms.push('w');
		} else {
			perms.push('-');
		}
		if self.tperm & IPERM_OEXEC != 0 {
			perms.push('x');
		} else {
			perms.push('-');
		}

		if self.tperm & IPERM_GREAD != 0 {
			perms.push('r');
		} else {
			perms.push('-');
		}
		if self.tperm & IPERM_GWRIT != 0 {
			perms.push('w');
		} else {
			perms.push('-');
		}
		if self.tperm & IPERM_GEXEC != 0 {
			perms.push('x');
		} else {
			perms.push('-');
		}

		if self.tperm & IPERM_UREAD != 0 {
			perms.push('r');
		} else {
			perms.push('-');
		}
		if self.tperm & IPERM_UWRIT != 0 {
			perms.push('w');
		} else {
			perms.push('-');
		}
		if self.tperm & IPERM_UEXEC != 0 {
			perms.push('x');
		} else {
			perms.push('-');
		}

		let date = crate::time::ctime(self.lmt);
		write!(
			f,
			"{perms} {hardlinks} {uid} {gid} {size:>4} {date}",
			hardlinks = self.count_hl,
			uid = self.uid,
			gid = self.gid
		)
	}
}

// Inode type occupy bit [12-15]
pub const ITYPE_FIFO: u16 = 0x1 << 12;
pub const ITYPE_CHARDEV: u16 = 0x2 << 12;
pub const ITYPE_DIR: u16 = 0x4 << 12;
pub const ITYPE_BLOCK: u16 = 0x6 << 12;
pub const ITYPE_REGU: u16 = 0x8 << 12;
pub const ITYPE_SYMF: u16 = 0xa << 12;
pub const ITYPE_SOCK: u16 = 0xc << 12;
// Inode perm occupy bit [0-11]
pub const IPERM_OEXEC: u16 = 0x001;
pub const IPERM_OWRIT: u16 = 0x002;
pub const IPERM_OREAD: u16 = 0x004;
pub const IPERM_GEXEC: u16 = 0x008;
pub const IPERM_GWRIT: u16 = 0x010;
pub const IPERM_GREAD: u16 = 0x020;
pub const IPERM_UEXEC: u16 = 0x040;
pub const IPERM_UWRIT: u16 = 0x080;
pub const IPERM_UREAD: u16 = 0x100;
pub const IPERM_STICK: u16 = 0x200;
pub const IPERM_SETGID: u16 = 0x400;
pub const IPERM_SETUID: u16 = 0x800;

// Inode flags
pub const IFLAG_SECDEL: u32 = 0x00000001;
pub const IFLAG_KEEPCPY: u32 = 0x00000002;
pub const IFLAG_FILECOMPR: u32 = 0x00000004;
pub const IFLAG_SYNC: u32 = 0x00000008;
pub const IFLAG_IMMUTABLE: u32 = 0x00000010;
pub const IFLAG_OAPPEN: u32 = 0x00000020;
pub const IFLAG_NODUMP: u32 = 0x00000040;
pub const IFLAG_NOUPDATE: u32 = 0x00000080;
pub const IFLAG_HASHINDEX: u32 = 0x00010000;
pub const IFLAG_ASDIR: u32 = 0x00020000;
pub const IFLAG_JOURN: u32 = 0x00040000;

#[derive(Debug, Default, Clone)]
pub struct Dentry {
	pub inode:       u32,
	pub dentry_size: u16,
	pub name_length: u8,
	pub r#type:      u8,
	pub name:        crate::string::String
}

#[repr(u8)]
pub enum Dtype {
	Unkown,
	Regular,
	Directory,
	Chardev,
	Blockdev,
	Fifo,
	Socket,
	Sym
}

use crate::alloc::string::ToString;
impl From<&[u8]> for Dentry {
	fn from(buffer: &[u8]) -> Self {
		let mut dentry = Self {
			inode:       u32::from_le_bytes(buffer[0..4].try_into().unwrap()),
			dentry_size: u16::from_le_bytes(buffer[4..6].try_into().unwrap()),
			name_length: u8::from_le_bytes(buffer[6..7].try_into().unwrap()),
			r#type:      u8::from_le_bytes(buffer[7..8].try_into().unwrap()),
			name:        crate::string::String::new()
		};
		if dentry.name_length != 0 {
			dentry.name = core::str::from_utf8(
				&buffer[8..8 + dentry.name_length as usize]
			)
			.expect("Error")
			.to_string();
		}
		dentry
	}
}

impl Into<crate::alloc::vec::Vec<u8>> for Dentry {
	fn into(self) -> crate::alloc::vec::Vec<u8> {
		let mut v = crate::alloc::vec::Vec::new();
		v.extend_from_slice(&self.inode.to_le_bytes());
		v.extend_from_slice(&self.dentry_size.to_le_bytes());
		v.extend_from_slice(&self.name_length.to_le_bytes());
		v.extend_from_slice(&self.r#type.to_le_bytes());
		v.extend_from_slice(&self.name.as_bytes());
		v
	}
}
