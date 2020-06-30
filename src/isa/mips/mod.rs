pub mod ee;
mod instruction;

use crate::core::cop0::Register;
use enum_primitive::*;
pub use instruction::*;

pub const NOP: u32 = 0x0000_0000;

enum_from_primitive!{
#[derive(Debug, PartialEq)]
pub enum Opcode {
	Special = 0b00_0000,
	Cache   = 0b10_1111,
	Cop0    = 0b01_0000,
	Cop1    = 0b01_0001,
	RegImm  = 0b00_0001,

	AddI    = 0b00_1000,
	AddIU   = 0b00_1001,
	AndI    = 0b00_1100,
	BEq     = 0b00_0100,
	BEqL    = 0b01_0100,
	BGTZ    = 0b00_0111,
	BLEZ    = 0b00_0110,
	BNE     = 0b00_0101,
	BNEL    = 0b01_0101,
	J       = 0b00_0010,
	JaL     = 0b00_0011,
	LB      = 0b10_0000,
	LBU     = 0b10_0100,
	LD      = 0b11_0111,
	LHU     = 0b10_0101,
	LUI     = 0b00_1111,
	LW      = 0b10_0011,
	OrI     = 0b00_1101,
	SB      = 0b10_1000,
	SD      = 0b11_1111,
	SLTI    = 0b00_1010,
	SLTIU   = 0b00_1011,
	SW      = 0b10_1011,
	SWC1    = 0b11_1001,
}
}

enum_from_primitive!{
#[derive(Debug, PartialEq)]
pub enum Function {
	Add   = 0b10_0000,
	AddU  = 0b10_0001,
	And   = 0b10_0100,
	Break = 0b00_1101,
	DAddU = 0b10_1101,
	Div   = 0b01_1010,
	DivU  = 0b01_1011,
	JaLR  = 0b00_1001,
	JR    = 0b00_1000,
	MFHi  = 0b01_0000,
	MFLo  = 0b01_0010,
	MovN  = 0b00_1011,
	Mult  = 0b01_1000,
	Or    = 0b10_0101,
	SLL   = 0b00_0000,
	SLT   = 0b10_1010,
	SLTU  = 0b10_1011,
	SRA   = 0b00_0011,
	SRL   = 0b00_0010,
	SubU  = 0b10_0011,
	Sync  = 0b00_1111,
}
}

impl Function {
	#[inline(always)]
	pub fn decode(instruction: u32) -> Option<Self> {
		let raw_func = instruction.r_get_function();
		Self::from_u8(raw_func)
	}
}

enum_from_primitive!{
#[derive(Debug, PartialEq)]
pub enum RegImmFunction {
	BGEZ   = 0b0_0001,
	BLTZ   = 0b0_0000,
}
}

impl RegImmFunction {
	#[inline(always)]
	pub fn decode(instruction: u32) -> Option<Self> {
		let raw_func = instruction.ri_get_target();
		trace!("RI {:05b}", raw_func);
		Self::from_u8(raw_func)
	}
}
