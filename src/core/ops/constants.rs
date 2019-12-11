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
	LUI     = 0b00_1111,
	OrI     = 0b00_1101,
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
	#[inline(always)]
	pub fn decode(instruction: u32) -> Option<Self> {
		let raw_func = instruction.r_get_function();
		Self::from_u8(raw_func)
	}
}

#[derive(Debug, PartialEq)]
pub enum Cop0Function {
	MFBPC,
	MFC0,
}

const MF0: u8 = 0b0_0000;
const C0:  u8 = 0b1_0000;
const BC0: u8 = 0b0_1000;
const MT0: u8 = 0b0_0100;

const LAST_11: u32 = 0b0111_1111_1111;

impl Cop0Function {
	#[inline(always)]
	pub fn decode(instruction: u32) -> Option<Self> {
		let family = instruction.ri_get_source();
		match family {
			MF0 => {
				trace!("MF0");
				match instruction & LAST_11 {
					0 => if instruction.r_get_destination() == 0b1_1000 {
						Some(Cop0Function::MFBPC)
					} else {
						Some(Cop0Function::MFC0)
					},
					_ => None,
				}
			},
			C0 => {
				trace!("C0");
				None
			},
			BC0 => {
				trace!("BC0");
				None
			},
			MT0 => {
				trace!("MT0");
				None
			},
			_ => {
				unreachable!();
			}
		}

	}
}

#[derive(Debug, PartialEq)]
pub enum Cop1Function {
}

impl Cop1Function {
	#[inline(always)]
	pub fn decode(instruction: u32) -> Option<Self> {
		None
	}
}