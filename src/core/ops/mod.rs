mod arithmetic;
pub mod constants;
mod instruction;

use constants::*;
use crate::core::{
	pipeline::*,
	EECore,
};
use enum_primitive::FromPrimitive;
use instruction::Instruction;

rs2_macro::ops!([
	[
		(ADD, arithmetic::add, MipsFunction::Add, 1)
	],
	[
		(ADDI, arithmetic::addi, MipsOpcode::AddI, 1)
	],
]);

pub fn nop(_cpu: &mut EECore, _data: &OpCode) {
	// No Op.
}

#[inline]
pub fn build_op_register(function: MipsFunction, source: u8, target: u8, destination: u8, shift_amount: u8) -> u32 {
	let mut out = 0;

	out.set_opcode(0);
	out.ri_set_source(source);
	out.ri_set_target(target);
	out.r_set_destination(destination);
	out.r_set_shift_amount(shift_amount);
	out.r_set_function(function as u8);

	out
}

#[inline]
pub fn build_op_immediate(opcode: MipsOpcode, source: u8, target: u8, immediate: u16) -> u32 {
	let mut out = 0;

	out.set_opcode(opcode as u8);
	out.ri_set_source(source);
	out.ri_set_target(target);
	out.i_set_immediate(immediate);

	out
}

#[inline]
pub fn build_op_jump(opcode: MipsOpcode, jump_target: u32) -> u32 {
	let mut out = 0;

	out.set_opcode(opcode as u8);
	out.j_set_jump(jump_target);

	out
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn build_register_op() {
		const ADD_1_2_3: u32 = 0b000000_00001_00010_00011_00000_100000;
		assert_eq!(
			build_op_register(MipsFunction::Add, 1, 2, 3, 0),
			ADD_1_2_3,
		);
	}

	#[test]
	fn build_immediate_op() {
		const ADDI_1_2_256: u32 = 0b001000_00001_00010_0000000100000000;
		assert_eq!(
			build_op_immediate(MipsOpcode::AddI, 1, 2, 256),
			ADDI_1_2_256,
		);
	}

	#[test]
	fn build_jump_op() {
		const J_256: u32 = 0b000010_00000000000000000100000000;
		assert_eq!(
			build_op_jump(MipsOpcode::J, 256),
			J_256,
		);
	}
}