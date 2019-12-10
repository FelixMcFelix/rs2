use enum_primitive::*;
use super::instruction::Instruction;

pub const NOP: u32 = 0x0000_0000;

enum_from_primitive!{
#[derive(Debug, PartialEq)]
pub enum MipsOpcode {
	Special = 0b00_0000,
	Cop0    = 0b01_0000,
	Cop1    = 0b01_0001,

	AddI    = 0b00_1000,
	AddIU   = 0b00_1001,
	BNE     = 0b00_0101,
	J       = 0b00_0010,
	JaL     = 0b00_0011,
	SLTI    = 0b00_1010, 
}
}

enum_from_primitive!{
#[derive(Debug, PartialEq)]
pub enum MipsFunction {
	Add  = 0b10_0000,
	AddU = 0b10_0001,
	And  = 0b10_0100,
	JaLR = 0b00_1001,
	JR   = 0b00_1000,
	SLL  = 0b00_0000,
}
}

impl MipsFunction {
	pub fn decode(instruction: u32) -> Option<Self> {
		let raw_func = instruction.r_get_function();
		Self::from_u8(raw_func)
	}
}

#[derive(Debug, PartialEq)]
pub enum Cop0Function {
}

impl Cop0Function {
	pub fn decode(instruction: u32) -> Option<Self> {
		None
	}
}

#[derive(Debug, PartialEq)]
pub enum Cop1Function {
}

impl Cop1Function {
	pub fn decode(instruction: u32) -> Option<Self> {
		None
	}
}