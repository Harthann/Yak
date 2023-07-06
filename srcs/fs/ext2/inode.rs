
pub struct inode {
    tperm:        u16,
    uid:          u16,
    size_lh:      u32,
    lat:          u32,
    creatt:       u32,
    lmt:          u32,
    dt:           u32,
    gid:          u16,
    cound_hl:     u16,
    count_ds:     u32,
    flags:        u32,
    os_specific1: u32,
    dbp0:         u32,
    dbp1:         u32,
    dbp2:         u32,
    dbp3:         u32,
    dbp4:         u32,
    dbp5:         u32,
    dbp6:         u32,
    dbp7:         u32,
    dbp8:         u32,
    dbp9:         u32,
    dbp10:        u32,
    dbp11:        u32,
    sibp:         u32,
    dibp:         u32,
    tibp:         u32,
    gen_no:       u32,
    facl:         u32,
    size_uh:      u32,
    block_addr:   u32,
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

