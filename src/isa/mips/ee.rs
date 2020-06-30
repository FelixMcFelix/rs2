use crate::core::cop0::Register;
use enum_primitive::*;
use super::instruction::Instruction;

enum_from_primitive!{
#[derive(Debug, PartialEq)]
pub enum C0Function {
	TlbWI   = 0b00_0010,
	TlbWR   = 0b00_0110,
}
}

impl C0Function {
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
// pub const TLBWI: u8 = 0b0_0010;
// pub const TLBWR: u8 = 0b0_0110;

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
				use C0Function::*;
				match C0Function::decode(instruction) {
					Some(TlbWI) => Some(Cop0Function::TlbWI),
					Some(TlbWR) => Some(Cop0Function::TlbWR),
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
