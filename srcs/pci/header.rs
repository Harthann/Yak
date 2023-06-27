use core::default::Default;
use core::fmt;

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

impl fmt::Display for Headers {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Headers::Standard(hdr) => {
				write!(f, "{}", hdr)
			},
			Headers::PciBridge(hdr) => {
				write!(f, "{}", hdr)
			},
			Headers::CardBusBridge(hdr) => {
				write!(f, "{}", hdr)
			},
			Headers::Unknown => {
				write!(f, "Unknown header")
			}
		}
	}
}

/// Header layout common for all PCI device
#[derive(Default, Debug)]
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
#[derive(Debug, Default)]
pub struct StandardHdr {
	// Reg [0x4 to 0x9]
	pub base_addr:   [u32; 6],
	// Reg 0xa
	pub cis_ptr:     u32,
	// Reg 0xb
	pub subs_vid:    u16,
	pub subs_id:     u16,
	// Reg 0xc
	pub rom_bar:     u32,
	// Reg 0xd
	pub cap_ptr:     u8,
	// End of reg 0xd and all Reg 0xe
	pub reserved:    [u8; 7],
	// Reg 0xf
	pub int_line:    u8,
	pub int_pin:     u8,
	pub min_grant:   u8,
	pub max_latency: u8
}

impl fmt::Display for StandardHdr {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(
			f,
			"StandardHdr {{
    BAR0: {:#x}
    Bar1: {:#x}
    Bar2: {:#x}
    Bar3: {:#x}
    Bar4: {:#x}
    Bar5: {:#x}
    CardBus CIS pointer: {:#x}
    SubSystem Vendor ID: {:X}
    SubSystem ID: {:x}
    Expansion ROM BAR: {:#x}
    Capabilities ptr: {:#x}
    Interrupt Line: {}
    Interrupt PIN: {}
    Min Grant: {}
    Max Latency: {}
}}",
			self.base_addr[0],
			self.base_addr[1],
			self.base_addr[2],
			self.base_addr[3],
			self.base_addr[4],
			self.base_addr[5],
			self.cis_ptr,
			self.subs_vid,
			self.subs_id,
			self.rom_bar,
			self.cap_ptr,
			self.int_line,
			self.int_pin,
			self.min_grant,
			self.max_latency
		)
	}
}

/// Header layout for Pci to Pci bridge
#[derive(Debug)]
pub struct PciBridgeHdr {}

impl fmt::Display for PciBridgeHdr {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Not yet implemented")
	}
}

/// Header layout for Pci to CardBus bridge
#[derive(Debug)]
pub struct CardBusBridgeHdr {}

impl fmt::Display for CardBusBridgeHdr {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Not yet implemented")
	}
}
