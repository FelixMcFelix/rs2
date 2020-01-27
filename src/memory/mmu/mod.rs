pub mod tlb;

use crate::core::{
	cop0::*,
	exceptions::L1Exception,
	EECore,
};
use tlb::Tlb;

pub struct Mmu {
	pub tlb: Tlb,
	pub page_mask: u32,

	/// Minimum value for random.
	///
	/// Maybe just pull from COP0 every time, since it's not shared.
	pub wired: u8,

	/// TLB entry for writing, via the TLBWR instruction.
	///
	/// Maybe just pull from COP0 every time since it has to be updated so frequently.
	pub random: u8,

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

impl Mmu {
	pub fn translate_address(&self, v_addr: u32, load: bool) -> MmuAddress {
		let vpn_shift_amount = page_mask_shift_amount(self.page_mask);

		let vpn = v_addr >> vpn_shift_amount;
		let vpn2 = vpn >> 1;
		let even_page = (vpn & 1) == 0;

		trace!("Translating {} -- VPN: {}", v_addr, vpn);

		let line = self.tlb.find_match(vpn2);

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
			} else {
				if !indiv_page.dirty {
					if !load {
						return Some(MmuAddress::Exception(L1Exception::TlbModified(v_addr)));
					}
				}
			}

			let offset = v_addr & (OFFSET_ALWAYS_ACTIVE_BITS | (self.page_mask >> 1));
			Some(MmuAddress::Address((indiv_page.page_frame_number << vpn_shift_amount) | offset))
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

	pub fn write_index(&mut self, page_mask: u32, entry_hi: u32, entry_lo0: u32, entry_lo1: u32) {
		self.tlb.lines[self.index as usize].update(page_mask, entry_hi, entry_lo0, entry_lo1);
	}
}

#[derive(Debug)]
pub enum MmuAddress {
	Address(u32),
	Exception(L1Exception),
}

impl Default for Mmu {
	fn default() -> Self {
		Self {
			tlb: Default::default(),
			page_mask: 0,
			wired: WIRED_DEFAULT as u8,
			random: RANDOM_DEFAULT as u8,
			index: 0,
			context: 0,
			asid: 0,
		}
	}
}