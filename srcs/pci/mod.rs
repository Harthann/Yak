mod header;

/// This value is used to select which address config you want to use
/// This address act as a chipselect
const PCI_CONFIG_ADDRESS: u16 = 0xcf8;
/// This value is used to read data base on the config adress used. You can write to?
const PCI_CONFIG_DATA: u16 = 0xcfc;

pub fn pci_scan() {
    for bus in 0..=255 {
        for slot in 0..=255 {
            let mut device = PciDevice {
                bus: bus,
                slot: slot,
                config: header::PciHdr::default()
            };

            if device.read_device_id() != 0xffff {
                device.read_all_header();
                crate::kprintln!("Device {}:{}\n{}", bus, slot, device.config.header);
            }
        }
    }
}

/// This struct represent a Pci Device and embed pci config operations read/write
/// Each read operation is done using I/O which means it's time consuming.
///
/// # Read selection layout 
///
///|  Bit 31    |  Bits 30-24  |  Bits 23-16  |	Bits 15-11      |  Bits 10-8        |  Bits 7-0         |
///|------------|--------------|--------------|-----------------|-------------------|-------------------|
///| Enable Bit |  Reserved    |  Bus Number  |  Device Number  |  Function Number  |  Register Offset1 |
///
/// # Register layout example
///
///| Register | Offset | Bits 31-24 | Bits 23-16   | Bits 15-8     | Bits 7-0        |
///|:--------:|:------:|:----------:|:------------:|:-------------:|:---------------:|
///| 0x0      |  0x0   | Device ID  |  Device ID   | Vendor ID     |  Vendor ID      |
///| 0x1      |  0x4   |  Status    |   Status     |   Command     |    Command      |
///| 0x2      |  0x8   | Class code |  Subclass    | Prog IF       | Revision ID     |
///| 0x3      |  0xC   |    BIST    |  Header type | Latency Timer | Cache Line Size |
///
/// # Register / Offset selection
///
///| Bits 7-2 | Bits 1-0  |
///|----------|-----------|
///| Register | Word/Byte |
///
#[derive(Debug)]
struct PciDevice {
    bus:  u8,
    slot: u8,
    config: header::PciHdr
}

impl PciDevice {
    /// Perform IO operation to read selected word from PCI device at bus:slot
    ///
    /// * `func` -   I don't know yet it's purpose
    ///
    /// * `offset` - Select which word in which register to read.
    ///
    ///| Bits 7-2 | Bits 1-0 |
    ///|:--------:|:--------:|
    ///| Register |   Word   |
    ///
    /// # Examples
    ///
    /// ```
    /// let pci = PciDevice::new(0,0);
    /// // Will read the first word in the register 0
    /// pci.config_read_word(0, (0 << 2) | 0)
    /// // Will read the first word in the register 1
    /// pci.config_read_word(0, (1 << 2) | 0)
    /// // Will read the second word in the register 1
    /// pci.config_read_word(0, (1 << 2) | 1)
    /// ```
    pub fn config_read_word(&self, func: u8, offset: u8) -> u16 {
        return (self.config_read_reg(func, offset) >> ((offset & 0x1) * 16) & 0xffff) as u16;
    }

    /// Perform IO operation to read selected byte from PCI device at bus:slot
    ///
    /// # Arguments
    ///
    /// * `func` -   I don't know yet it's purpose
    ///
    /// * `offset` - Select which byte in which register to read.
    ///
    ///| Bits 7-2 | Bits 1-0 |
    ///|:--------:|:--------:|
    ///| Register |   Byte   |
    ///
    /// # Examples
    ///
    /// ```
    /// let pci = PciDevice::new(0,0);
    /// // Will read the first word in the register 0
    /// pci.config_read_byte(0, (0 << 2) | 0)
    /// // Will read the first word in the register 1
    /// pci.config_read_byte(0, (1 << 2) | 0)
    /// ```
    pub fn config_read_byte(&self, func: u8, offset: u8) -> u8 {
        return ((self.config_read_reg(func, offset) >> ((offset & 0x3) * 8)) & 0xff) as u8;
    }

    /// Perform IO operation to read selected register from PCI device at bus:slot
    ///
    /// # Arguments
    ///
    /// * `func` -   I don't know yet it's purpose
    ///
    /// * `offset` - Select which register to read.
    ///
    ///| Bits 7-2 | Bits 1-0 |
    ///|:--------:|:--------:|
    ///| Register | Ignored  |
    ///
    /// # Examples
    ///
    /// ```
    /// let pci = PciDevice::new(0,0);
    /// // Will read the first register
    /// pci.config_read_reg(0, 0 << 2)
    /// // Will read the second register
    /// pci.config_read_reg(0, 1 << 2)
    /// ```
    pub fn config_read_reg(&self, func: u8, offset: u8) -> u32 {
        let address: u32 =    ((self.bus as u32)  << 16) as u32
                            | ((self.slot as u32) << 11) as u32
                            | ((func as u32) << 8)  as u32
                            | (offset & 0xfc) as u32
                            | 0x80000000;
        crate::io::outl(PCI_CONFIG_ADDRESS, address);
        crate::io::inl(PCI_CONFIG_DATA)
    }

    #[inline]
    fn read_device_id(&mut self) -> u16 {
        self.config.common.device_id = self.config_read_word(0, (0 << 2) | 1);
        self.config.common.device_id
    }
    #[inline]
    fn read_vendor_id(&mut self) -> u16 {
        self.config.common.vendor_id = self.config_read_word(0, (0 << 2) | 0);
        self.config.common.vendor_id
    }

    #[inline]
    fn read_command(&mut self) -> u16 {
        self.config.common.command = self.config_read_word(0, (1 << 2) | 0);
        self.config.common.command
    }
    #[inline]
    fn read_status(&mut self) -> u16 {
        self.config.common.status = self.config_read_word(0, (1 << 2) | 1);
        self.config.common.status
    }

    #[inline]
    fn read_revision_id(&mut self) -> u8 {
        self.config.common.revision_id = self.config_read_byte(0, (2 << 2) | 0);
        self.config.common.revision_id
    }
    #[inline]
    fn read_prog_if(&mut self) -> u8 {
        self.config.common.prog_if = self.config_read_byte(0, (2 << 2) | 1);
        self.config.common.prog_if
    }
    #[inline]
    fn read_subclass(&mut self) -> u8 {
        self.config.common.subclass = self.config_read_byte(0, (2 << 2) | 2);
        self.config.common.subclass
    }
    #[inline]
    fn read_class_code(&mut self) -> u8 {
        self.config.common.class_code = self.config_read_byte(0, (2 << 2) | 3);
        self.config.common.class_code
    }


    #[inline]
    fn read_cache_line_size(&mut self) -> u8 {
        self.config.common.cache_line_size = self.config_read_byte(0, (3 << 2) | 0);
        self.config.common.cache_line_size
    }
    #[inline]
    fn read_latency_timer(&mut self) -> u8 {
        self.config.common.latency_timer = self.config_read_byte(0, (3 << 2) | 1);
        self.config.common.latency_timer
    }
    #[inline]
    fn read_header_type(&mut self) -> u8 {
        self.config.common.header_type = self.config_read_byte(0, (3 << 2) | 2);
        self.config.common.header_type
    }
    #[inline]
    fn read_bist(&mut self) -> u8 {
        self.config.common.bist = self.config_read_byte(0, (3 << 2) | 3);
        self.config.common.bist
    }

    /// Call each read function to fill the common header
    pub fn read_common_header(&mut self) {
        self.read_vendor_id();
        self.read_device_id();
        self.read_command();
        self.read_status();
        self.read_revision_id();
        self.read_prog_if();
        self.read_class_code();
        self.read_subclass();
        self.read_cache_line_size();
        self.read_latency_timer();
        self.read_header_type();
        self.read_bist();
    }

    /// Read the end of the header based on the HeaderType stored in the common header
    /// The common header need to be readed first in order to know the type of this header
    pub fn read_device_header(&mut self) {
        // Bit 7 indicate if the device as multiple functions
        // Bit 0-2 indicate the header type
        match self.config.common.header_type & 0x3 {
            0x0 => { self.config.header = header::Headers::Standard(self.read_standard_header()) },
            0x1 => { todo!() },
            0x2 => { todo!() },
            _   => {}
        };
    }

    /// Read the register from 0x4 to 0xf as if it was a Standard header
    fn read_standard_header(&self) -> header::StandardHdr {
        let mut hdr = header::StandardHdr::default();
        for i in 0..=5 {
            hdr.base_addr[i] = self.config_read_reg(0, (0x4 + i as u8) << 2 );
        }
        hdr.cis_ptr      = self.config_read_reg(0, 0xa << 2 );
        hdr.subs_vid     = self.config_read_word(0, (0xb << 2) | 0 );
        hdr.subs_id      = self.config_read_word(0, (0xb << 2) | 1 );
        hdr.rom_bar      = self.config_read_reg(0, 0xc << 2 );
        hdr.cap_ptr      = self.config_read_byte(0, (0xd << 2) | 0 );
        hdr.int_line     = self.config_read_byte(0, (0xf << 2) | 0 );
        hdr.int_pin      = self.config_read_byte(0, (0xf << 2) | 1 );
        hdr.min_grant    = self.config_read_byte(0, (0xf << 2) | 2 );
        hdr.max_latency  = self.config_read_byte(0, (0xf << 2) | 3 );
        hdr
    }

    /// Read both commond header and device header
    pub fn read_all_header(&mut self) {
        self.read_common_header();
        self.read_device_header();
    }
}

use core::fmt;
impl fmt::Display for PciDevice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Bus {:<03} Device {:<03}: ID {:x}:{:x}", self.bus, self.slot, self.config.common.vendor_id, self.config.common.device_id)
    }
}
