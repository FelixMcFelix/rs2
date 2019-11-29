use crate::core::{
	pipeline::*,
	EECore,
};
use super::instruction::Instruction;

const PC_HO_BITS: u32 = 0b1111 << 28;

pub fn j(cpu: &mut EECore, data: &OpCode) {
	// this MUST be delayed.
	// NOTE: target computation relies on NEXT PC val.
	cpu.pc_register = (cpu.pc_register & PC_HO_BITS) | (data.j_get_jump() << 2);
}

#[cfg(test)]
mod tests {
	use super::*;
	use byteorder::{
		ByteOrder,
		LittleEndian,
	};
	use crate::core::ops::{
		self,
		constants::*,
	};

	#[test]
	fn jump_not_instant() {
		// Execute ONLY a jump instruction. No change to PC.
		let jump_offset = 0x12_34_56;
		let jump_target = 0 | (jump_offset << 2);

		let program = vec![
			ops::build_op_jump(MipsOpcode::J, jump_offset),
		];
		let mut program_bytes = Vec::with_capacity(4 * program.len());
		LittleEndian::write_u32_into(&program[..], &mut program_bytes[..]);

		let mut test_ee = EECore::new();

		for _i in 0..program.len() {
			test_ee.cycle(&program_bytes[..]);
		}

		assert_ne!(test_ee.pc_register, jump_target);
	}

	#[test]
	fn basic_jump() {
		// Execute a jump instruction and a NOP. PC changes to new target.
		let jump_offset: u32 = 0x12_34_56;
		let jump_target = 0 | (jump_offset << 2);

		let program = vec![
			ops::build_op_jump(MipsOpcode::J, jump_offset),
			ops::build_op_register(MipsFunction::Add, 0, 0, 0, 0),
		];
		let mut program_bytes = Vec::with_capacity(4 * program.len());
		LittleEndian::write_u32_into(&program[..], &mut program_bytes[..]);

		let mut test_ee = EECore::new();

		for _i in 0..program.len() {
			test_ee.cycle(&program_bytes[..]);
		}

		assert_eq!(test_ee.pc_register, jump_target);
	}

	#[test]
	fn jump_delay_slot_fires() {
		// Execute a jump instruction and an ADD. ADD should have taken effect.
		let jump_offset: u32 = 0x12_34_56;
		let jump_target = 0 | (jump_offset << 2);

		let program = vec![
			ops::build_op_jump(MipsOpcode::J, jump_offset),
			ops::build_op_register(MipsFunction::Add, 1, 2, 3, 0),
		];
		let mut program_bytes = Vec::with_capacity(4 * program.len());
		LittleEndian::write_u32_into(&program[..], &mut program_bytes[..]);

		let mut test_ee = EECore::new();

		let in_1 = 111;
		let in_2 = 256;
		test_ee.write_register(1, in_1);
		test_ee.write_register(2, in_2);

		for _i in 0..program.len() {
			test_ee.cycle(&program_bytes[..]);
		}

		assert_eq!(test_ee.read_register(3), in_1 + in_2);
	}
}