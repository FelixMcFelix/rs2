use super::{Function, Opcode};

const OP_MASK: u32 = 0b0011_1111;
const REGISTER_MASK: u32 = 0b0001_1111;
const IMMEDIATE_MASK: u32 = 0xFF_FF;
const JUMP_MASK: u32 = 0x03_FF_FF_FF;

/// Add methods to a standard 32-bit MIPS instruction to extract individual data
/// or parameters, without requiring extra space.
pub trait Instruction {
	fn get_opcode(&self) -> u8;

	fn ri_get_source(&self) -> u8;
	fn ri_get_target(&self) -> u8;
	fn r_get_destination(&self) -> u8;
	fn r_get_shift_amount(&self) -> u8;
	fn r_get_function(&self) -> u8;

	fn i_get_immediate(&self) -> u16;
	fn i_get_immediate_signed(&self) -> i16;

	fn j_get_jump(&self) -> u32;

	fn set_opcode(&mut self, v: u8);

	fn ri_set_source(&mut self, v: u8);
	fn ri_set_target(&mut self, v: u8);
	fn r_set_destination(&mut self, v: u8);
	fn r_set_shift_amount(&mut self, v: u8);
	fn r_set_function(&mut self, v: u8);

	fn i_set_immediate(&mut self, v: u16);

	fn j_set_jump(&mut self, v: u32);
}

impl Instruction for u32 {
	#[inline]
	fn get_opcode(&self) -> u8 {
		(self >> 26) as u8
	}

	#[inline]
	fn ri_get_source(&self) -> u8 {
		((self >> 21) & REGISTER_MASK) as u8
	}

	#[inline]
	fn ri_get_target(&self) -> u8 {
		((self >> 16) & REGISTER_MASK) as u8
	}

	#[inline]
	fn r_get_destination(&self) -> u8 {
		((self >> 11) & REGISTER_MASK) as u8
	}

	#[inline]
	fn r_get_shift_amount(&self) -> u8 {
		((self >> 6) & REGISTER_MASK) as u8
	}

	#[inline]
	fn r_get_function(&self) -> u8 {
		(self &OP_MASK) as u8
	}

	#[inline]
	fn i_get_immediate(&self) -> u16 {
		(self & 0xFF_FF) as u16
	}

	#[inline]
	fn i_get_immediate_signed(&self) -> i16 {
		self.i_get_immediate() as i16
	}

	#[inline]
	fn j_get_jump(&self) -> u32 {
		self & 0x03_FF_FF_FF
	}

	#[inline]
	fn set_opcode(&mut self, v: u8) {
		*self &= !(OP_MASK << 26);
		*self |= u32::from(v) << 26;
	}

	#[inline]
	fn ri_set_source(&mut self, v: u8) {
		*self &= !(REGISTER_MASK << 21);
		*self |= u32::from(v) << 21;
	}

	#[inline]
	fn ri_set_target(&mut self, v: u8) {
		*self &= !(REGISTER_MASK << 16);
		*self |= u32::from(v) << 16;
	}

	#[inline]
	fn r_set_destination(&mut self, v: u8) {
		*self &= !(REGISTER_MASK << 11);
		*self |= u32::from(v) << 11;
	}

	#[inline]
	fn r_set_shift_amount(&mut self, v: u8) {
		*self &= !(REGISTER_MASK << 6);
		*self |= u32::from(v) << 6;
	}

	#[inline]
	fn r_set_function(&mut self, v: u8) {
		*self &= !OP_MASK;
		*self |= u32::from(v);
	}

	#[inline]
	fn i_set_immediate(&mut self, v: u16) {
		*self &= !IMMEDIATE_MASK;
		*self |= u32::from(v);
	}

	#[inline]
	fn j_set_jump(&mut self, v: u32) {
		*self &= !JUMP_MASK;
		*self |= v;
	}
}

#[inline]
pub fn build_op_register(function: Function, source: u8, target: u8, destination: u8, shift_amount: u8) -> u32 {
	build_op_register_custom(Opcode::Special, function as u8, source, target, destination, shift_amount)
}

#[inline]
pub fn build_op_register_custom(opcode: Opcode, function: u8, source: u8, target: u8, destination: u8, shift_amount: u8) -> u32 {
	let mut out = 0;

	out.set_opcode(opcode as u8);
	out.ri_set_source(source);
	out.ri_set_target(target);
	out.r_set_destination(destination);
	out.r_set_shift_amount(shift_amount);
	out.r_set_function(function);

	out
}

#[inline]
pub fn build_op_immediate(opcode: Opcode, source: u8, target: u8, immediate: u16) -> u32 {
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
pub fn build_op_jump(opcode: Opcode, jump_target: u32) -> u32 {
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
			build_op_register(Function::Add, 1, 2, 3, 0),
			ADD_1_2_3,
		);
	}

	#[test]
	fn build_immediate_op() {
		const ADDI_1_2_256: u32 = 0b001000_00001_00010_0000000100000000;
		assert_eq!(
			build_op_immediate(Opcode::AddI, 1, 2, 256),
			ADDI_1_2_256,
		);
	}

	#[test]
	fn build_jump_op() {
		const J_256: u32 = 0b000010_00000000000000000100000000;
		assert_eq!(
			build_op_jump(Opcode::J, 256),
			J_256,
		);
	}
}
