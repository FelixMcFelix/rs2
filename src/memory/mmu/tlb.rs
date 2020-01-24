use crate::core::EECore;

/// TLB of the EE Core is 48 entries wide.
pub const EE_TLB_WIDTH: usize = 48;

// FIXME: magic number
pub struct Tlb {
	lines: [TlbLine; EE_TLB_WIDTH],
}

impl Default for Tlb {
	fn default() -> Self {
		Self {
			lines: [Default::default(); EE_TLB_WIDTH],
		}
	}
}

/// Defined on pp 123--124.
#[derive(Clone, Copy, Default)]
pub struct TlbLine {
	mask: u32,
	virtual_page_number: u32,
	global: bool,
	asid: u8,

	scratchpad: bool,
	odd_page_frame_number: u32,
	odd_cache_mode: u8,
	odd_dirty: bool,
	odd_valid: bool,

	even_page_frame_number: u32,
	even_cache_mode: u8,
	even_dirty: bool,
	even_valid: bool,
}

impl Tlb {
	
}

impl TlbLine {
	fn from_cop0(cpu: &EECore) -> Self {
		unimplemented!()
		// Default::default()
	}
}