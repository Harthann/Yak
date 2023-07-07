

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
pub struct Inode {
    /// Type and Permissions (see below)
    tperm:        u16,
    /// User ID
    uid:          u16,
    /// Lower 32 bits of size in bytes
    size_lh:      u32,
    /// Last Access Time (in POSIX time)
    lat:          u32,
    /// Creation Time (in POSIX time)
    creatt:       u32,
    /// Last Modification time (in POSIX time)
    lmt:          u32,
    /// Deletion time (in POSIX time)
    delt:         u32,
    /// Group ID
    gid:          u16,
    /// Count of hard links (directory entries) to this inode. When this reaches 0, the data blocks are marked as unallocated.
    cound_hl:     u16,
    /// Count of disk sectors (not Ext2 blocks) in use by this inode, not counting the actual inode structure nor directory entries linking to the inode.
    count_ds:     u32,
    /// Flags (see below)
    flags:        u32,
    /// Operating System Specific value #1
    os_specific1: u32,
    /// Direct Block Pointer 0
    dbp0:         u32,
    /// Direct Block Pointer 1
    dbp1:         u32,
    /// Direct Block Pointer 2
    dbp2:         u32,
    /// Direct Block Pointer 3
    dbp3:         u32,
    /// Direct Block Pointer 4
    dbp4:         u32,
    /// Direct Block Pointer 5
    dbp5:         u32,
    /// Direct Block Pointer 6
    dbp6:         u32,
    /// Direct Block Pointer 7
    dbp7:         u32,
    /// Direct Block Pointer 8
    dbp8:         u32,
    /// Direct Block Pointer 9
    dbp9:         u32,
    /// Direct Block Pointer 10
    dbp10:        u32,
    /// Direct Block Pointer 11
    dbp11:        u32,
    /// Singly Indirect Block Pointer (Points to a block that is a list of block pointers to data)
    sibp:         u32,
    /// Doubly Indirect Block Pointer (Points to a block that is a list of block pointers to Singly Indirect Blocks)
    dibp:         u32,
    /// Triply Indirect Block Pointer (Points to a block that is a list of block pointers to Doubly Indirect Blocks)
    tibp:         u32,
    /// Generation number (Primarily used for NFS)
    gen_no:       u32,
    /// In Ext2 version 0, this field is reserved. In version >= 1, Extended attribute block (File ACL).
    facl:         u32,
    /// In Ext2 version 0, this field is reserved. In version >= 1, Upper 32 bits of file size (if feature bit set) if it's a file, Directory ACL if it's a directory
    size_uh:      u32,
    /// Block address of fragment
    block_addr:   u32,
    /// Operating System Specific Value #2
    op_specific2: [u8; 12]
}

// Inode type occupy bit [12-15]
const ITYPE_FIFO:    u16 = 0x1 << 12;
const ITYPE_CHARDEV: u16 = 0x2 << 12;
const ITYPE_DIR:     u16 = 0x4 << 12;
const ITYPE_BLOCK:   u16 = 0x6 << 12;
const ITYPE_REGU:    u16 = 0x8 << 12;
const ITYPE_SYMF:    u16 = 0xa << 12;
const ITYPE_SOCK:    u16 = 0xc << 12;
// Inode perm occupy bit [0-11]
const IPERM_OEXEC:   u16 = 0x001;
const IPERM_OWRIT:   u16 = 0x002;
const IPERM_OREAD:   u16 = 0x004;
const IPERM_GEXEC:   u16 = 0x008;
const IPERM_GWRIT:   u16 = 0x010;
const IPERM_GREAD:   u16 = 0x020;
const IPERM_UEXEC:   u16 = 0x040;
const IPERM_UWRIT:   u16 = 0x080;
const IPERM_UREAD:   u16 = 0x100;
const IPERM_STICK:   u16 = 0x200;
const IPERM_SETGID:  u16 = 0x400;
const IPERM_SETUID:  u16 = 0x800;

// Inode flags
const IFLAG_SECDEL:     u32 = 0x00000001;
const IFLAG_KEEPCPY:    u32 = 0x00000002;
const IFLAG_FILECOMPR:  u32 = 0x00000004;
const IFLAG_SYNC:       u32 = 0x00000008;
const IFLAG_IMMUTABLE:  u32 = 0x00000010;
const IFLAG_OAPPEN:     u32 = 0x00000020;
const IFLAG_NODUMP:     u32 = 0x00000040;
const IFLAG_NOUPDATE:   u32 = 0x00000080;
const IFLAG_HASHINDEX:  u32 = 0x00010000;
const IFLAG_ASDIR:      u32 = 0x00020000;
const IFLAG_JOURN:      u32 = 0x00040000;

