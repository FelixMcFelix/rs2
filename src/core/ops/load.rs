use crate::core::{
	pipeline::*,
	EECore,
};
use super::instruction::Instruction;

pub fn lui(cpu: &mut EECore, data: &OpCode) {
	// load sign extended shifted value of immediate into rt.
	let v = data.i_get_immediate() as i64;
	cpu.write_register(data.ri_get_target(), (v << 16) as u64);
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
	fn basic_lui() {
		// Place a 16-bit value into bits 32..16.
		let in_1: i16 = 12345;

		let mut test_ee = EECore::new();

		let instruction = ops::build_op_immediate(MipsOpcode::LUI, 0, 1, in_1 as u16);

		test_ee.execute(ops::process_instruction(instruction));

		assert_eq!(test_ee.read_register(1) >> 16, in_1 as u64);
	}
}