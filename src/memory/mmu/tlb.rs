use crate::core::{
	cop0::{
		EntryHi,
		EntryLo,
	},
	EECore,
};

/// TLB of the EE Core is 48 entries wide.
pub const EE_TLB_WIDTH: usize = 48;

// FIXME: magic number
pub struct Tlb {
	pub lines: [TlbLine; EE_TLB_WIDTH],
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
	pub mask: u32,
	pub virtual_page_number_half: u32,
	pub global: bool,
	pub asid: u8,

	pub scratchpad: bool,
	pub even: TlbPageInfo,
	pub odd: TlbPageInfo,
}

#[derive(Clone, Copy, Default)]
pub struct TlbPageInfo {
	pub page_frame_number: u32,
	pub cache_mode: u8,
	pub dirty: bool,
	pub valid: bool,
}

impl Tlb {
	pub fn find_match(& self, vpn_2: u32) -> Option<& TlbLine> {
		let mut out = None;

		for line in &self.lines[..] {
			if vpn_2 == line.virtual_page_number_half {
				out = Some(line);
				break;
			}
		}

		out
	}
}

impl TlbLine {
	fn from_cop0(cpu: &EECore) -> Self {
		unimplemented!()
		// Default::default()
	}

	pub fn update(&mut self, page_mask: u32, entry_hi: u32, entry_lo0: u32, entry_lo1: u32) {
		self.mask = page_mask;

		self.virtual_page_number_half = entry_hi.get_vpn2();
		self.asid = entry_hi.get_asid();

		self.scratchpad = entry_lo0.is_scratchpad();
		self.even.update(entry_lo0);
		self.odd.update(entry_lo1);

		self.global = entry_lo0.is_global() && entry_lo1.is_global();
	}
}

impl TlbPageInfo {
	pub fn update(&mut self, entry_lo: u32) {
		self.page_frame_number = entry_lo.get_pfn();
		self.cache_mode = entry_lo.get_cache_mode();
		self.dirty = entry_lo.is_dirty();
		self.valid = entry_lo.is_valid();
	}
}