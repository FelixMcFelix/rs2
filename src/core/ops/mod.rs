mod arithmetic;
mod branch;
pub mod constants;
mod cop0;
pub mod instruction;
mod load;

pub use constants::*;
use crate::core::{
	pipeline::*,
	EECore,
};
use enum_primitive::FromPrimitive;
use instruction::Instruction;
use timings::*;

rs2_macro::ops!([
	[
		(MipsOpcode::Special, "R", MipsFunction::decode, [
			(ADD, arithmetic::add, MipsFunction::Add, INTEGER_SUM_LOGIC_DELAY),
			(ADDU, arithmetic::addu, MipsFunction::AddU, INTEGER_SUM_LOGIC_DELAY),
			(AND, arithmetic::and, MipsFunction::And, INTEGER_SUM_LOGIC_DELAY),
			(JALR, branch::jalr, MipsFunction::JaLR, INTEGER_BRANCH_JUMP_DELAY),
			(JR, branch::jr, MipsFunction::JR, INTEGER_BRANCH_JUMP_DELAY),
			(SLL, arithmetic::sll, MipsFunction::SLL, INTEGER_SHIFT_LUI_DELAY),
		]),
		(MipsOpcode::Cop0, "COP0", Cop0Function::decode, [
			(MFBPC, cop0::mfc0, Cop0Function::MFBPC, INTEGER_LOAD_STORE_DELAY),
			(MFC0, cop0::mfc0, Cop0Function::MFC0, INTEGER_LOAD_STORE_DELAY),
		]),
		(MipsOpcode::Cop1, "COP1", Cop1Function::decode, [
			// N/A
		]),
	],
	[
		(ADDI, arithmetic::addi, MipsOpcode::AddI, INTEGER_SUM_LOGIC_DELAY),
		(ADDIU, arithmetic::addiu, MipsOpcode::AddIU, INTEGER_SUM_LOGIC_DELAY),
		(BNE, branch::bne, MipsOpcode::BNE, INTEGER_BRANCH_JUMP_DELAY),
		(J, branch::j, MipsOpcode::J, INTEGER_BRANCH_JUMP_DELAY),
		(JAL, branch::jal, MipsOpcode::JaL, INTEGER_BRANCH_JUMP_DELAY),
		(LUI, load::lui, MipsOpcode::LUI, INTEGER_SHIFT_LUI_DELAY),
		(ORI, arithmetic::ori, MipsOpcode::OrI, INTEGER_SUM_LOGIC_DELAY),
		(SLTI, arithmetic::slti, MipsOpcode::SLTI, INTEGER_SUM_LOGIC_DELAY),
	],
]);

pub fn nop(_cpu: &mut EECore, _data: &OpCode) {
	// No Op.
	trace!("NOP FIRED");
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

/// Build a jump opcode.
///
/// Assumes that `jump_target` can be represented using 26 bits.
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