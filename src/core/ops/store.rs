use byteorder::{ByteOrder, LittleEndian};
use crate::core::{
	pipeline::*,
	EECore,
};
use super::instruction::Instruction;

pub fn sw(cpu: &mut EECore, data: &OpCode) {
	// mem[GPR[rs] + signed(imm)] <- (GPR[rt] as 32)
	let to_store = cpu.read_register(data.ri_get_target()) as u32;
	let v_addr = cpu.read_register(data.ri_get_source()) + ((data.i_get_immediate() as i16) as u64);
	trace!("I want to store {} in v_addr {:08x}",
		to_store,
		v_addr,
	);

	if let Some(loc) = cpu.read_memory_mut(v_addr as u32, 4) {
		LittleEndian::write_u32(loc, to_store);
	}

	// should except if ttarget addr is badly aligned.
	// unimplemented!()
}

pub fn sd(cpu: &mut EECore, data: &OpCode) {
	// mem[GPR[rs] + signed(imm)] <- (GPR[rt] as 64)
	let to_store = cpu.read_register(data.ri_get_target());
	let v_addr = cpu.read_register(data.ri_get_source()) + (data.i_get_immediate_signed() as u64);
	trace!("I want to store {} in v_addr {:08x} + {:08x}",
		to_store,
		cpu.read_register(data.ri_get_source()),
		data.i_get_immediate_signed(),
	);

	if let Some(loc) = cpu.read_memory_mut(v_addr as u32, 8) {
		LittleEndian::write_u64(loc, to_store);
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use byteorder::{
		ByteOrder,
		LittleEndian,
	};
	use crate::{
		core::ops::{
			self,
			constants::*,
		},
		memory::constants::*,
	};

	#[test]
	fn basic_sw() {
		unimplemented!()
	}

	#[test]
	fn basic_sd() {
		unimplemented!()
	}
}