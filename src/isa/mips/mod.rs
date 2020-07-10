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

/// Consistent functions shared between a MIPS CPU.
pub trait Cpu {
	type Register;

	fn read_register(&self, index: u8) -> Self::Register;
	fn write_register(&mut self, index: u8, value: Self::Register);

	fn read_cop0(&self, index: u8) -> Self::Register;
	fn write_cop0(&mut self, index: u8, value: Self::Register);

	fn read_hi(&self) -> Self::Register;
	fn write_hi(&mut self, value: Self::Register);

	fn read_lo(&self) -> Self::Register;
	fn write_lo(&mut self, value: Self::Register);
}

/// Mask of available/required registers and instruction pipes for issuing.
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct Capability {
	/// Combination of registers being written to, and CPU instruction pipes consumed.
	pub write: u64,

	/// Combination of registers being read from.
	pub read: u64,
}

/// Wrapper around `Capability` to encode instructions
/// who are compatible with fitting into several pipeline slots.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Requirement<T> {
	Joint(T),
	Disjoint(T, T),
}

impl<T: Default> Default for Requirement<T> {
	fn default() -> Self {
		Requirement::Joint(Default::default())
	}
}

impl<T> Requirement<T> {
	pub fn first(&self) -> &T {
		match self {
			Requirement::Joint(a) => &a,
			Requirement::Disjoint(a, _b) => &a,
		}
	}

	pub fn second(&self) -> Option<&T> {
		match self {
			Requirement::Joint(_a) => None,
			Requirement::Disjoint(_a, b) => Some(&b),
		}
	}
}

impl Capability {
	pub const REG_PC: u64 = 1 << 32;
	pub const REG_SA: u64 = 1 << 33;
	pub const REG_HI: u64 = 1 << 34;
	pub const REG_LO: u64 = 1 << 35;
	pub const REG_HI1: u64 = 1 << 36;
	pub const REG_LO1: u64 = 1 << 37;

	/// Shift amount for pipeline requirements.
	pub const PIPELINE_SHIFT: u64 = 38;

	/// Mask to select all registers.
	pub const REGISTER_MASK: u64 = (1 << Self::PIPELINE_SHIFT) - 1;

	/// Mask to select all non-registers.
	pub const PIPE_MASK: u64 = !Self::REGISTER_MASK;

	#[inline]
	pub fn normalised(write: u64, read: u64) -> Self {
		Self {
			write,
			read: read & (read ^ write),
		}
	}

	pub fn all() -> Self {
		Self {
			write: u64::MAX,
			read: u64::MAX,
		}
	}

	pub fn write_d_read_ts(i: u32) -> Self {
		Self::normalised(
			1 << i.r_get_destination(),
			(1 << i.ri_get_target()) | (1 << i.ri_get_source()),
		)
	}

	pub fn write_d_read_s(i: u32) -> Self {
		Self::normalised(
			1 << i.r_get_destination(),
			1 << i.ri_get_source(),
		)
	}

	pub fn write_d_read_t(i: u32) -> Self {
		Self::normalised(
			1 << i.r_get_destination(),
			1 << i.ri_get_target(),
		)
	}

	pub fn write_d(i: u32) -> Self {
		Self::normalised(
			1 << i.r_get_destination(),
			0,
		)
	}

	pub fn write_t_read_s(i: u32) -> Self {
		Self::normalised(
			1 << i.ri_get_target(),
			1 << i.ri_get_source(),
		)
	}

	pub fn write_t_read_d(i: u32) -> Self {
		Self::normalised(
			1 << i.ri_get_target(),
			1 << i.r_get_destination(),
		)
	}

	pub fn write_t(i: u32) -> Self {
		Self::normalised(
			1 << i.ri_get_target(),
			0,
		)
	}

	pub fn read_ts(i: u32) -> Self {
		Self::normalised(
			0,
			(1 << i.ri_get_target()) | (1 << i.ri_get_source()),
		)
	}

	pub fn read_s(i: u32) -> Self {
		Self::normalised(
			0,
			1 << i.ri_get_source(),
		)
	}

	pub fn read_t(i: u32) -> Self {
		Self::normalised(
			0,
			1 << i.ri_get_target(),
		)
	}

	pub fn read_td(i: u32) -> Self {
		Self::normalised(
			0,
			(1 << i.ri_get_target()) | 1 << i.r_get_destination(),
		)
	}

	pub fn jump(_i: u32) -> Self {
		Self {
			write: Self::REG_PC,
			read: 0,
		}
	}

	pub fn jump_link(_i: u32) -> Self {
		Self {
			write: Self::REG_PC | (1 << 31),
			read: 0,
		}
	}

	pub fn jump_link_reg(i: u32) -> Self {
		Self {
			write: Self::REG_PC | (1 << i.r_get_destination()),
			read: (1 << i.ri_get_source()),
		}
	}

	pub fn jump_reg(i: u32) -> Self {
		Self {
			write: Self::REG_PC,
			read: (1 << i.ri_get_source()),
		}
	}

	pub fn branch_compare(i: u32) -> Self {
		Self::normalised(
			Self::REG_PC,
			(1 << i.ri_get_source()) | (1 << i.ri_get_source()),
		)
	}

	pub fn branch_read_s(i: u32) -> Self {
		Self::normalised(
			Self::REG_PC,
			1 << i.ri_get_source(),
		)
	}

	pub fn mul_div(i: u32) -> Self {
		Self::normalised(
			Self::REG_HI | Self::REG_LO,
			(1 << i.ri_get_source()) | (1 << i.ri_get_source()),
		)
	}

	pub fn no_req(_i: u32) -> Self {
		Default::default()
	}
}
