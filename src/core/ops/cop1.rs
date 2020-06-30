use byteorder::{
	ByteOrder,
	LittleEndian,
};
use crate::{
	core::{
		exceptions::L1Exception,
		pipeline::*,
		EECore,
	},
	isa::mips::Instruction,
	utils::*,
};
use std::mem::size_of;

pub fn swc1(cpu: &mut EECore, data: &OpCode) {
	debug!("SWC1 not reading from Cop1 -- Writing zeroes!");

	let to_store = 0 as u32;
	let offset: u32 = data.i_get_immediate_signed().s_ext();
	let v_addr = (cpu.read_register(data.ri_get_source()) as u32).wrapping_add(offset);

	// FIXME: make size info part of address resolution.
	if v_addr & 0b11 != 0 {
		cpu.throw_l1_exception(L1Exception::AddressErrorStore(v_addr));
		return;
	}

	if let Some(loc) = cpu.read_memory_mut(v_addr as u32, size_of::<u32>()) {
		LittleEndian::write_u32(loc, to_store);
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn basic_swc1() {
		unimplemented!();
	}
}