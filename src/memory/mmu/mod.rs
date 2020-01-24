pub mod tlb;

use crate::core::{
	cop0::*,
	EECore,
};
use tlb::Tlb;

pub struct Mmu {
	tlb: Tlb,
	page_mask: u32,

	/// Minimum value for random.
	///
	/// Maybe just pull from COP0 every time, since it's not shared.
	wired: u8,

	/// TLB entry for writing, via the TLBWR instruction.
	///
	/// Maybe just pull from COP0 every time since it has to be updated so frequently.
	random: u8,

	/// TLB entry for reading or writing, via TLBR and TLBWI instructions.
	index: u8,
	context: u32,
}

impl Mmu {
	pub fn translate_address(cpu: &mut EECore, v_addr: u32) -> Option<u32> {
		unimplemented!()
		// None
	}
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
		}
	}
}