
// mount_no indicate the number of mount since last fsck
// mount_no_max indicate the number of mount between each fsck
/// Superblock always take 1024 bytes with/without Extended block
struct BaseSuperblock {
    ///Total number of inodes in file system
    inode_tnum:       u32,
    ///Total number of blocks in file system
    blocks_tnum:      u32,
    ///Number of blocks reserved for superuser (see offset 80)
    rblocks_num:      u32,
    ///Total number of unallocated blocks
    blocks_unalloc:   u32,
    ///Total number of unallocated inodes
    inode_unalloc:    u32,
    ///Block number of the block containing the superblock (also the starting block number, NOT always zero.)
    superblock_block: u32,
    ///log2 (block size) - 10. (In other words, the number to shift 1,024 to the left by to obtain the block size)
    block_size:       u32,
    ///log2 (fragment size) - 10. (In other words, the number to shift 1,024 to the left by to obtain the fragment size)
    frag_size:        u32,
    ///Number of blocks in each block group
    bgroup_bno :      u32,
    ///Number of fragments in each block group
    bgroup_fno:       u32,
    ///Number of inodes in each block group
    bgroup_ino:       u32,
    ///Last mount time (in POSIX time)
    last_mt:          u32,
    ///Last written time (in POSIX time)
    last_wt:          u32,
    ///Number of times the volume has been mounted since its last consistency check (fsck)
    mount_no:         u16,
    ///Number of mounts allowed before a consistency check (fsck) must be done
    mount_no_max:     u16,
    ///Ext2 signature (0xef53), used to help confirm the presence of Ext2 on a volume
    ext2_sig:         u16,
    ///File system state (see below)
    fs_state:         u16,
    ///What to do when an error is detected (see below)
    err_handle:       u16,
    ///Minor portion of version (combine with Major portion below to construct full version field)
    minor:            u16,
    ///POSIX time of last consistency check (fsck)
    last_fsck:        u32,
    ///Interval (in POSIX time) between forced consistency checks (fsck)
    fsck_interval:    u32,
    ///Operating system ID from which the filesystem on this volume was created (see below)
    osid:             u32,
    ///Major portion of version (combine with Minor portion above to construct full version field)
    major:            u32,
    ///User ID that can use reserved blocks
    uid:              u16,
    ///Group ID that can use reserved blocks 
    gid:              u16,
    extension: Option<ExtendedSuperblock>
}


const FSSTATE_CLEAN: u16 = 1;
const FSSTATE_ERROR: u16 = 2;

const FSERROR_IGN:  u16 = 1;
const FSERROR_MRO:  u16 = 2;
const FSERROR_KPAN: u16 = 3;

const FSCREAT_LINUX: u16 = 0;
const FSCREAT_GNU:   u16 = 1;
const FSCREAT_MASIX: u16 = 2;
const FSCREAT_FBSD:  u16 = 3;
const FSCREAT_OTHER: u16 = 4;

/// Present if Major >= 1
/// Bytes from 236 to 1023 aren't used
struct ExtendedSuperblock {
 	///First non-reserved inode in file system. (In versions < 1.0, this is fixed as 11)
    first_inode: u32,
 	///Size of each inode structure in bytes. (In versions < 1.0, this is fixed as 128)
    inode_size: u16,
 	///Block group that this superblock is part of (if backup copy)
    bgroup_superblock: u16,
 	///Optional features present (features that are not required to read or write, but usually result in a performance increase. see below)
    opt_features: u32,
 	///Required features present (features that are required to be supported to read or write. see below)
    req_features: u32,
 	///Features that if not supported, the volume must be mounted read-only see below)
    ro_features: u32,
 	///File system ID (what is output by blkid)
    fsid: [u8; 16],
 	///Volume name (C-style string: characters terminated by a 0 byte)
    vol_name: [u8; 16],
 	///Path volume was last mounted to (C-style string: characters terminated by a 0 byte)
    last_path: [u8; 64],
 	///Compression algorithms used (see Required features above)
    compr: u32,
 	///Number of blocks to preallocate for files
    prealloc_blocks_files: u8,
 	///Number of blocks to preallocate for directories
    prealloc_block_dir: u8,
 	///(Unused)
    unused: u16,
 	///Journal ID (same style as the File system ID above)
    journ_id: [u8; 16],
 	///Journal inode
    journ_inode: u32,
 	///Journal device
    journ_dev: u32,
 	///Head of orphan inode list
    orphan_inode_lst: u32
}

const OPTFEAT_PREALLOC:   u32 = 0x0001;
const OPTFEAT_AFSSERV:    u32 = 0x0002;
const OPTFEAT_JOURN:      u32 = 0x0004;
const OPTFEAT_INODEEXT:   u32 = 0x0008;
const OPTFEAT_RESIZE:     u32 = 0x0010;
const OPTFEAT_HASH_INDEX: u32 = 0x0020;

const REQFEAT_COMPR:        u32 = 0x0001;
const REQFEAT_DE_TYPEFIELD: u32 = 0x0002;
const REQFEAT_REPLAY_JOURN: u32 = 0x0004;
const REQFEAT_JOURN_DEV:    u32 = 0x0008;

const ROFEAT_SPARS:      u32 = 0x0001;
const ROFEAT_64B:        u32 = 0x0002;
const ROFEAT_DIR_BTRE:   u32 = 0x0004;





