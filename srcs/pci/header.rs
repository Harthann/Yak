use core::default::Default;

#[derive(Default, Debug)]
pub struct PciHdr {
    pub common: PciCommonHdr,
    pub header: Headers
}

#[derive(Default, Debug)]
pub enum Headers {
    Standard(StandardHdr),
    PciBridge(PciBridgeHdr),
    CardBusBridge(CardBusBridgeHdr),
    #[default]
    Unknown
}

#[derive(Default,Debug)]
pub struct PciCommonHdr {
    pub vendor_id:       u16,
    pub device_id:       u16,
    pub command:         u16,
    pub status:          u16,
    pub revision_id:     u8,
    pub prog_if:         u8,
    pub subclass:        u8,
    pub class_code:      u8,
    pub cache_line_size: u8,
    pub latency_timer:   u8,
    pub header_type:     u8,
    pub bist:            u8
}

/// Represent the status register of a pci device, allowing friendly interface to read/write bit on
/// this register
pub struct StatusRegister(u16);

/// Represent the command register of a pci device, allowing friendly interface to read/write bit on
/// this register
pub struct CommandRegister(u16);

impl StatusRegister {
    pub const PIN0: u16 = 1 << 0;

    pub fn set(&mut self, pin: u16) {
        self.0 |= pin;
    }
}

/// Header layout for standard Pci 
#[derive(Debug)]
pub struct StandardHdr {
}

/// Header layout for Pci to Pci bridge 
#[derive(Debug)]
pub struct PciBridgeHdr {
}

/// Header layout for Pci to CardBus bridge
#[derive(Debug)]
pub struct CardBusBridgeHdr {
}

