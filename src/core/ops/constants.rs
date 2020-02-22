use crate::core::cop0::Register;
use enum_primitive::*;
use super::instruction::Instruction;

pub const NOP: u32 = 0x0000_0000;

enum_from_primitive!{
#[derive(Debug, PartialEq)]
pub enum MipsOpcode {
	Special = 0b00_0000,
	Cache   = 0b10_1111,
	Cop0    = 0b01_0000,
	Cop1    = 0b01_0001,
	RegImm  = 0b00_0001,

	AddI    = 0b00_1000,
	AddIU   = 0b00_1001,
	AndI    = 0b00_1100,
	BNE     = 0b00_0101,
	J       = 0b00_0010,
	JaL     = 0b00_0011,
	LUI     = 0b00_1111,
	OrI     = 0b00_1101,
	SD      = 0b11_1111,
	SLTI    = 0b00_1010,
	SW      = 0b10_1011,
}
}

enum_from_primitive!{
#[derive(Debug, PartialEq)]
pub enum MipsFunction {
	Add   = 0b10_0000,
	AddU  = 0b10_0001,
	And   = 0b10_0100,
	DAddU = 0b10_1101,
	JaLR  = 0b00_1001,
	JR    = 0b00_1000,
	Mult  = 0b01_1000,
	SLL   = 0b00_0000,
	Sync  = 0b00_1111,
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
	MTBPC,
	MTC0,
	TlbWI,
	TlbWR,
}

pub const MF0:   u8 = 0b0_0000;
pub const C0:    u8 = 0b1_0000;
pub const BC0:   u8 = 0b0_1000;
pub const MT0:   u8 = 0b0_0100;
pub const TLBWI: u8 = 0b0_0010;
pub const TLBWR: u8 = 0b0_0110;

const LAST_11: u32 = 0b0111_1111_1111;

impl Cop0Function {
	#[inline(always)]
	pub fn decode(instruction: u32) -> Option<Self> {
		let family = instruction.ri_get_source();
		match family {
			MF0 => {
				trace!("MF0");
				match instruction & LAST_11 {
					0 => if instruction.r_get_destination() == Register::Debug as u8 {
						Some(Cop0Function::MFBPC)
					} else {
						Some(Cop0Function::MFC0)
					},
					_ => None,
				}
			},
			C0 => {
				trace!("C0");
				let func = instruction.r_get_function(); 
				match func {
					TLBWI => Some(Cop0Function::TlbWI),
					TLBWR => Some(Cop0Function::TlbWR),
					_ => None,
				}
			},
			BC0 => {
				trace!("BC0");
				None
			},
			MT0 => {
				trace!("MT0");
				match instruction & LAST_11 {
					0 => if instruction.r_get_destination() == Register::Debug as u8 {
						Some(Cop0Function::MTBPC)
					} else {
						Some(Cop0Function::MTC0)
					},
					_ => None,
				}
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
	pub fn decode(_instruction: u32) -> Option<Self> {
		None
	}
}

enum_from_primitive!{
#[derive(Debug, PartialEq)]
pub enum CacheFunction {
	BFH    = 0b0_1100,
	BHINBT = 0b0_1010,
	BXLBT  = 0b0_0010,
	BXSBT  = 0b0_0110,
	DHIN   = 0b1_1010,
	DHWBIN = 0b1_1000,
	DHWOIN = 0b1_1100,
	DXIN   = 0b1_0110,
	DXLDT  = 0b1_0001,
	DXLTG  = 0b1_0000,
	DXSDT  = 0b1_0011,
	DXSTG  = 0b1_0010,
	DXWBIN = 0b1_0100,
	IFL    = 0b0_1110,
	IHIN   = 0b0_1011,
	IXIN   = 0b0_0111,
	IXLDT  = 0b0_0001,
	IXLTG  = 0b0_0000,
	IXSDT  = 0b0_0101,
	IXSTG  = 0b0_0100,
}
}

impl CacheFunction {
	#[inline(always)]
	pub fn decode(instruction: u32) -> Option<Self> {
		let raw_func = instruction.ri_get_target();
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

pub mod timings {
	//! Timings relating to operations and instructions.
	//! These are informed by the *EE Core User's Manual 6.0*,
	//! page 58.

	pub const INTEGER_SUM_LOGIC_DELAY: u8 = 1;
	pub const INTEGER_HI_LO_TRANSFER_DELAY: u8 = 1;
	pub const INTEGER_SHIFT_LUI_DELAY: u8 = 1;
	pub const INTEGER_BRANCH_JUMP_DELAY: u8 = 1;
	pub const INTEGER_CONDITIONAL_MOVE_DELAY: u8 = 1;
	pub const INTEGER_MULT_DELAY: u8 = 4;
	pub const INTEGER_DIV_DELAY: u8 = 37;
	pub const INTEGER_MADD_DELAY: u8 = 4;
	pub const INTEGER_LOAD_STORE_DELAY: u8 = 1;

	pub const FLOAT_MTC1_DELAY: u8 = 2;
	pub const FLOAT_ADD_NEG_COND_DELAY: u8 = 4;
	pub const FLOAT_CVT_DELAY: u8 = 4;
	pub const FLOAT_MUL_DELAY: u8 = 4;
	pub const FLOAT_MFC1_DELAY: u8 = 2;
	pub const FLOAT_MOVE_DELAY: u8 = 4;
	pub const FLOAT_DIV_DELAY: u8 = 8;
	pub const FLOAT_SQRT_DELAY: u8 = 8;
	pub const FLOAT_RSQRT_DELAY: u8 = 14;
	pub const FLOAT_MADD_DELAY: u8 = 4;
	pub const FLOAT_LWC1_DELAY: u8 = 2;
}