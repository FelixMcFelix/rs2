use crate::core::{
	cop0::{
		EntryHi,
		EntryLo,
	},
	EECore,
};
use super::PAGE_MASK_16KB;

/// TLB of the EE Core is 48 entries wide.
pub const EE_TLB_WIDTH: usize = 48;

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
#[derive(Clone, Copy, Debug, Default)]
pub struct TlbLine {
	pub mask: u32,
	pub virtual_page_number_half: u32,
	pub global: bool,
	pub asid: u8,

	pub scratchpad: bool,
	pub even: TlbPageInfo,
	pub odd: TlbPageInfo,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct TlbPageInfo {
	pub page_frame_number: u32,
	pub cache_mode: u8,
	pub dirty: bool,
	pub valid: bool,
}

impl Tlb {
	pub fn find_match(&self, vpn_2: u32, spr_vpn_2: u32) -> Option<&TlbLine> {
		let mut out = None;
		println!("v: {:08x} sv: {:08x}", vpn_2, spr_vpn_2);

		for line in &self.lines[..] {
			println!("sp? {} vpn {:08x}", line.scratchpad, line.virtual_page_number_half);
			if (line.scratchpad && spr_vpn_2 == line.virtual_page_number_half)
					|| vpn_2 == line.virtual_page_number_half {
				out = Some(line);
				break;
			}
		}

		out
	}
}

impl TlbLine {
	pub fn update(&mut self, page_mask: u32, entry_hi: u32, entry_lo0: u32, entry_lo1: u32) {
		self.mask = page_mask;

		self.virtual_page_number_half = entry_hi.get_vpn2();
		self.asid = entry_hi.get_asid();

		self.scratchpad = entry_lo0.is_scratchpad();
		self.even.update(entry_lo0);
		self.odd.update(entry_lo1);

		if self.scratchpad {
			// Valid SPRAM must meet some... interesting conditions.
			// * Scratchpad set to true.
			// * MASK set to zero (treat as 16-bit)
			// * Matching D and V between even and odd. (note D!=v)
			// * Must be mapped onto 16-bit aligned V-addr
			// * Disregard PFN, C (C=2 when read)
			let valid_spram =
				self.mask == 0
				&& self.even.dirty == self.odd.dirty
				&& self.even.valid == self.odd.valid
				&& (self.virtual_page_number_half & PAGE_MASK_16KB) == 0;
			assert!(valid_spram);

			// FIXME: magic number
			self.even.cache_mode = 2;
			self.odd.cache_mode = 2;
		}

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