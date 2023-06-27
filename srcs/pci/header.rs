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

/// | Register | Offset | Bits 31-24 | Bits 23-16 | Bits 15-8 | Bits 7-0 |
/// |:--------:|:------:|:----------:|:----------:|:---------:|:--------:|
/// |   0x4 	  |  0x10  | Base address #0 (BAR0)                   |-|-|-|
/// |   0x5 	  |  0x14  | Base address #1 (BAR1)                   |-|-|-|
/// |   0x6 	  |  0x18  | Secondary Latency Timer | Subordinate Bus Number | Secondary Bus Number | Primary Bus Number |
/// |   0x7 	  |  0x1C  | Secondary Status       |-| I/O Limit | I/O Base |-|
/// |   0x8 	  |  0x20  | Memory Limit           |-| Memory Base         |-|
/// |   0x9 	  |  0x24  | Prefetchable Memory Limit |-| Prefetchable Memory Base |-|
/// |   0xA 	  |  0x28  | Prefetchable Base Upper 32 Bits             |-|-|-|
/// |   0xB 	  |  0x2C  | Prefetchable Limit Upper 32 Bits            |-|-|-|
/// |   0xC 	  |  0x30  | I/O Limit Upper 16 Bits | I/O Base Upper 16 Bits |
/// |   0xD 	  |  0x34  | Reserved              |-|-| Capability Pointer |
/// |   0xE 	  |  0x38  | Expansion ROM base address               |-|-|-|
/// |   0xF 	  |  0x3C  | Bridge Control        |-| Interrupt PIN | Interrupt Line  |
///
/// Header layout for Pci to Pci bridge
#[derive(Debug, Default)]
pub struct PciBridgeHdr {
	pub bar0:               u32,
	pub bar1:               u32,
	pub sec_lt:             u8,
	pub sub_bno:            u8,
	pub sec_bno:            u8,
	pub prim_bno:           u8,
	pub sec_status:         u16,
	pub io_limit:           u8,
	pub io_base:            u8,
	pub mem_limit:          u16,
	pub mem_base:           u16,
	pub prefetch_mem_limit: u16,
	pub prefetch_mem_base:  u16,
	pub prefetch_base_up:   u32,
	pub prefetch_limit_up:  u32,
	pub io_limit_up:        u16,
	pub io_base_up:         u16,
	pub reserves:           [u8; 3],
	pub cap_ptr:            u8,
	pub rom_base_addr:      u32,
	pub bridge_ctrl:        u16,
	pub int_pin:            u8,
	pub int_line:           u8
}

impl fmt::Display for PciBridgeHdr {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Not yet implemented")
	}
}

/// <table style="text-align: center">
/// <tbody><tr>
/// <th> Register </th><th> Offset </th><th> Bits 31-24 </th><th> Bits 23-16 </th><th> Bits 15-8 </th><th> Bits 7-0</th></tr>
/// <tr><td> 0x4</td><td> 0x10</td>
/// <td colspan="4"> CardBus Socket/ExCa base address</td></tr>
/// <tr><td> 0x5</td><td> 0x14</td>
/// <td colspan="2"> Secondary status</td><td> Reserved</td><td> Offset of capabilities list</td></tr>
/// <tr><td> 0x></td><td> 0x18</td>
/// <td> CardBus latency timer</td><td> Subordinate bus number</td>
/// <td> CardBus bus number</td><td> PCI bus number</td></tr>
/// <tr><td> 0x7</td><td> 0x1C</td>
/// <td colspan="4"> Memory Base Address 0</td></tr>
/// <tr><td> 0x8</td><td> 0x20</td>
/// <td colspan="4"> Memory Limit 0</td></tr>
/// <tr><td> 0x9</td><td> 0x24</td>
/// <td colspan="4"> Memory Base Address 1</td></tr>
/// <tr><td> 0xA</td><td> 0x28</td>
/// <td colspan="4"> Memory Limit 1</td></tr>
/// <tr><td> 0xB</td><td> 0x2C</td>
/// <td colspan="4"> I/O Base Address 0</td></tr>
/// <tr><td> 0xC</td><td> 0x30</td>
/// <td colspan="4"> I/O Limit 0</td></tr>
/// <tr><td> 0xD</td><td> 0x34</td>
/// <td colspan="4"> I/O Base Address 1</td></tr>
/// <tr><td> 0xE</td><td> 0x38</td>
/// <td colspan="4"> I/O Limit 1</td></tr>
/// <tr><td> 0xF</td><td> 0x3C</td>
/// <td colspan="2"> Bridge Control</td><td> Interrupt PIN</td><td> Interrupt Line</td></tr>
/// <tr><td> 0x10</td><td> 0x40</td>
/// <td colspan="2"> Subsystem Vendor ID</td><td colspan="2"> Subsystem Device ID</td></tr>
/// <tr><td> 0x11</td><td> 0x44</td>
/// <td colspan="4"> 16-bit PC Card legacy mode base address
/// </td></tr></tbody></table>
#[derive(Debug, Default)]
pub struct CardBusBridgeHdr {
	pub cb_socket:        u32,
	pub sec_status:       u16,
	pub reserved:         u8,
	pub cap_offset:       u8,
	pub cb_latency_timer: u8,
	pub sub_bno:          u8,
	pub cb_bno:           u8,
	pub pci_bno:          u8,
	pub mem_baddr_0:      u32,
	pub mem_limit_0:      u32,
	pub mem_baddr_1:      u32,
	pub mem_limit_1:      u32,
	pub io_baddr_0:       u32,
	pub io_limit_0:       u32,
	pub io_baddr_1:       u32,
	pub io_limit_1:       u32,
	pub bridge_ctrl:      u16,
	pub int_pin:          u8,
	pub int_line:         u8,
	pub subs_vid:         u16,
	pub subs_id:          u16,
	pub lmod_baddr:       u32
}

impl fmt::Display for CardBusBridgeHdr {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Not yet implemented")
	}
}
