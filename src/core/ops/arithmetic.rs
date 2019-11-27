use crate::core::{
	pipeline::*,
	EECore,
};
use super::instruction::Instruction;

pub fn add(cpu: &mut EECore, data: &OpCode) {
	println!("Calling add");
	// FIXME: Need to throw an exception here if 32-bit overflow.
	// FIXME: Need to sign-extend from 32-bit.
	cpu.write_register(
		data.raw.r_get_destination(),
		cpu.read_register(data.raw.ri_get_source()) + cpu.read_register(data.raw.ri_get_target()),
	);
}

pub fn addi(cpu: &mut EECore, data: &OpCode) {
	println!("Calling addi");
	// FIXME: Need to throw an exception here if 32-bit overflow.
	// FIXME: Need to sign-extend from 32-bit.
	cpu.write_register(
		data.raw.ri_get_target(),
		cpu.read_register(data.raw.ri_get_source()) + u64::from(data.raw.i_get_immediate()),
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
		unimplemented!();
	}
}