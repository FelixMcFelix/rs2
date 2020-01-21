use crate::core::{
	pipeline::*,
	EECore,
};
use super::instruction::Instruction;

pub fn sw(cpu: &mut EECore, data: &OpCode) {
	// mem[GPR[rs] + signed(imm)] <- (GPR[rt] as 32)
	trace!("I want to store {} in v_addr {:08x}",
		cpu.read_register(data.ri_get_target()) as u32,
		cpu.read_register(data.ri_get_source()) + ((data.i_get_immediate() as i16) as u64),
	);

	// should except if ttarget addr is badly aligned.
	// unimplemented!()
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
}