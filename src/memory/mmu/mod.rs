pub mod tlb;

use crate::core::{
	cop0::*,
	exceptions::L1Exception,
};
use tlb::Tlb;

pub struct Mmu {
	pub tlb: Tlb,
	pub page_mask: u32,

	/// Minimum value for random.
	///
	/// Maybe just pull from COP0 every time, since it's not shared.
	pub wired: u8,

	/// TLB entry for reading or writing, via TLBR and TLBWI instructions.
	pub index: u8,
	pub context: u32,

	pub asid: u8,
}

const VPN_ALWAYS_ACTIVE_BITS:    u32 = 0b1111_1110_0000_0000_0000_0000_0000_0000;
const OFFSET_ALWAYS_ACTIVE_BITS: u32 = 0b0000_0000_0000_0000_0000_1111_1111_1111;

const RAW_MASK_4KB:   u32 = 0b0000_0000_0000;
const RAW_MASK_16KB:  u32 = 0b0000_0000_0011;
const RAW_MASK_64KB:  u32 = 0b0000_0000_1111;
const RAW_MASK_256KB: u32 = 0b0000_0011_1111;
const RAW_MASK_1MB:   u32 = 0b0000_1111_1111;
const RAW_MASK_4MB:   u32 = 0b0011_1111_1111;
const RAW_MASK_16MB:  u32 = 0b1111_1111_1111;

const PAGE_MASK_4KB: u32 = RAW_MASK_4KB << 13;
const PAGE_MASK_16KB: u32 = RAW_MASK_16KB << 13;
const PAGE_MASK_64KB: u32 = RAW_MASK_64KB << 13;
const PAGE_MASK_256KB: u32 = RAW_MASK_256KB << 13;
const PAGE_MASK_1MB: u32 = RAW_MASK_1MB << 13;
const PAGE_MASK_4MB: u32 = RAW_MASK_4MB << 13;
const PAGE_MASK_16MB: u32 = RAW_MASK_16MB << 13;

#[inline]
/// Given a page mask, return the shift needed to extract the virtual page number.
pub fn page_mask_shift_amount(p_mask: u32) -> u32 {
	12 + match p_mask {
		PAGE_MASK_4KB => 0,
		PAGE_MASK_16KB => 2,
		PAGE_MASK_64KB => 4,
		PAGE_MASK_256KB => 6,
		PAGE_MASK_1MB => 8,
		PAGE_MASK_4MB => 10,
		PAGE_MASK_16MB => 12,
		_ => unreachable!(),
	}
}

pub fn page_mask_size(p_mask: u32) -> &'static str {
	match p_mask {
		PAGE_MASK_4KB => "4KB",
		PAGE_MASK_16KB => "16KB",
		PAGE_MASK_64KB => "64KB",
		PAGE_MASK_256KB => "256KB",
		PAGE_MASK_1MB => "1MB",
		PAGE_MASK_4MB => "4MB",
		PAGE_MASK_16MB => "16MB",
		_ => unreachable!(),
	}
}

const SPR_SHIFT_AMOUNT: u32 = 12 + 2 + 1;

impl Mmu {
	pub fn translate_address(&self, v_addr: u32, load: bool) -> MmuAddress {
		let vpn_shift_amount = page_mask_shift_amount(self.page_mask);

		let vpn = v_addr >> vpn_shift_amount;
		let vpn2 = (vpn >> 1) << (vpn_shift_amount - 12);
		let spr_vpn2 = (v_addr >> SPR_SHIFT_AMOUNT) << (SPR_SHIFT_AMOUNT - 12 - 1);
		let even_page = (vpn & 1) == 0;

		trace!("Translating {} -- VPN: {}", v_addr, vpn2);

		let line = self.tlb.find_match(vpn2, spr_vpn2);

		let out = line.and_then(|line| {
			if !line.global && line.asid != self.asid {
				return None;
			}

			let indiv_page = if even_page {
				&line.even
			} else {
				&line.odd
			};

			if !indiv_page.valid {
				if load {
					return Some(MmuAddress::Exception(L1Exception::TlbFetchLoadInvalid(v_addr)));
				} else {
					return Some(MmuAddress::Exception(L1Exception::TlbStoreInvalid(v_addr)));
				}
			} else if !indiv_page.dirty && !load {
				return Some(MmuAddress::Exception(L1Exception::TlbModified(v_addr)));
			}

			Some(if line.scratchpad {
				let offset = v_addr & (OFFSET_ALWAYS_ACTIVE_BITS | (PAGE_MASK_16KB >> 1));
				MmuAddress::Scratchpad(offset)
			} else {
				let offset = v_addr & (OFFSET_ALWAYS_ACTIVE_BITS | (self.page_mask >> 1));
				MmuAddress::Address((indiv_page.page_frame_number << vpn_shift_amount) | offset)
			})
		}).unwrap_or_else(|| MmuAddress::Exception(if load {
			L1Exception::TlbFetchLoadRefill(v_addr)
		} else {
			L1Exception::TlbStoreRefill(v_addr)
		}));

		// Now, switch between cache, memory and scratchpad.
		// probably don't care about cache...

		trace!("Result: {:?}", out);

		out
	}

	pub fn write_index(&mut self, entry_hi: u32, entry_lo0: u32, entry_lo1: u32) {
		self.tlb.lines[self.index as usize].update(self.page_mask, entry_hi, entry_lo0, entry_lo1);

		trace!("Put into line {}: {:?}", self.index, self.tlb.lines[self.index as usize]);
	}

	pub fn write_random(&mut self, random_index: u32, entry_hi: u32, entry_lo0: u32, entry_lo1: u32) {
		self.tlb.lines[random_index as usize].update(self.page_mask, entry_hi, entry_lo0, entry_lo1);

		trace!("Put into line {}: {:?}", self.index, self.tlb.lines[self.index as usize]);
	}
}

#[derive(Debug, PartialEq)]
pub enum MmuAddress {
	Address(u32),
	Scratchpad(u32),
	Exception(L1Exception),
}

impl Default for Mmu {
	fn default() -> Self {
		Self {
			tlb: Default::default(),
			page_mask: 0,
			wired: WIRED_DEFAULT as u8,
			index: 0,
			context: 0,
			asid: 0,
		}
	}
}