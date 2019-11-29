use crate::core::{
	pipeline::*,
	EECore,
};
use super::instruction::Instruction;

pub fn add(cpu: &mut EECore, data: &OpCode) {
	// FIXME: Need to throw an exception here if 32-bit overflow.
	// FIXME: Need to sign-extend from 32-bit.
	cpu.write_register(
		data.r_get_destination(),
		cpu.read_register(data.ri_get_source()) + cpu.read_register(data.ri_get_target()),
	);
}

pub fn addi(cpu: &mut EECore, data: &OpCode) {
	// FIXME: Need to throw an exception here if 32-bit overflow.
	// FIXME: Need to sign-extend from 32-bit.
	cpu.write_register(
		data.ri_get_target(),
		cpu.read_register(data.ri_get_source()) + u64::from(data.i_get_immediate()),
	);
}

pub fn addiu(cpu: &mut EECore, data: &OpCode) {
	// FIXME: Need to sign-extend from 32-bit.
	cpu.write_register(
		data.ri_get_target(),
		cpu.read_register(data.ri_get_source()) + u64::from(data.i_get_immediate()),
	);
}

pub fn addu(cpu: &mut EECore, data: &OpCode) {
	// FIXME: Need to sign-extend from 32-bit.
	cpu.write_register(
		data.r_get_destination(),
		cpu.read_register(data.ri_get_source()) + cpu.read_register(data.ri_get_target()),
	);
}

pub fn and(cpu: &mut EECore, data: &OpCode) {
	cpu.write_register(
		data.r_get_destination(),
		cpu.read_register(data.ri_get_source()) & cpu.read_register(data.ri_get_target()),
	);
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::core::ops::{
		self,
		constants::*,
	};

	#[test]
	fn basic_add() {
		// Place a value into registers 1 and 2, store their sum in register 3.
		let in_1 = 36;
		let in_2 = 19;

		let mut test_ee = EECore::new();
		test_ee.write_register(1, in_1);
		test_ee.write_register(2, in_2);

		let instruction = ops::build_op_register(MipsFunction::Add, 1, 2, 3, 0);

		test_ee.execute(ops::process_instruction(instruction));

		assert_eq!(test_ee.read_register(3), in_1 + in_2);
	}

	#[test]
	fn add_overflow_exception() {
		// 32-bit signed overflow should trap.
		// Destination register should be unaffected.
		unimplemented!();
	}

	#[test]
	fn basic_addi() {
		// Place a value into register 1, store their sum in register 2.
		let in_1 = 36;
		let in_2 = 19;

		let mut test_ee = EECore::new();
		test_ee.write_register(1, in_1);

		let instruction = ops::build_op_immediate(MipsOpcode::AddI, 1, 2, in_2);

		test_ee.execute(ops::process_instruction(instruction));

		assert_eq!(test_ee.read_register(2), in_1 + u64::from(in_2));
	}

	#[test]
	fn addi_overflow_exception() {
		// 32-bit signed overflow should trap.
		// Destination register should be unaffected.
		unimplemented!();
	}

	#[test]
	fn basic_addu() {
		// Place a value into registers 1 and 2, store their sum in register 3.
		let in_1 = 23467;
		let in_2 = 34578;

		let mut test_ee = EECore::new();
		test_ee.write_register(1, in_1);
		test_ee.write_register(2, in_2);

		let instruction = ops::build_op_register(MipsFunction::AddU, 1, 2, 3, 0);

		test_ee.execute(ops::process_instruction(instruction));

		assert_eq!(test_ee.read_register(3), in_1 + in_2);
	}

	#[test]
	fn addu_no_overflow_exception() {
		// Signed overflow SHOULD be allowed.
		unimplemented!();
	}

	#[test]
	fn basic_addiu() {
		// Place a value into register 1, store their sum in register 2.
		let in_1 = 23467;
		let in_2 = 34578;

		let mut test_ee = EECore::new();
		test_ee.write_register(1, in_1);

		let instruction = ops::build_op_immediate(MipsOpcode::AddIU, 1, 2, in_2);

		test_ee.execute(ops::process_instruction(instruction));

		assert_eq!(test_ee.read_register(2), in_1 + u64::from(in_2));
	}

	#[test]
	fn addiu_no_overflow_exception() {
		// Signed overflow SHOULD be allowed.
		unimplemented!();
	}

	#[test]
	fn basic_and() {
		// Need to ensure this works on full 64-bit width.
		let in_1 = 0b1000_0000_0000_0000_0000_0000_0100_1111_0010_0000_0000_0010_0000_0000_1111_0001;
		let in_2 = 0b1000_1111_0000_0000_0100_0000_0100_1111_0010_0000_0000_0000_0000_0000_0000_0001;

		let mut test_ee = EECore::new();
		test_ee.write_register(1, in_1);
		test_ee.write_register(2, in_2);

		let instruction = ops::build_op_register(MipsFunction::And, 1, 2, 3, 0);

		test_ee.execute(ops::process_instruction(instruction));

		assert_eq!(test_ee.read_register(3), in_1 & in_2);
	}
}