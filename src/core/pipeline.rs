use bitflags::bitflags;
use crate::isa::mips::{Capability, Instruction, Requirement};
use super::{
	ops,
	EECore,
};

/// The function signature required by any CPU instruction.
///
/// Each is (internally) responsible for knowing/determining
/// the correct way to decode [`OpData`](enum.OpData.html).
pub type EEAction = fn(&mut EECore, &OpCode) -> ();

/// The function signature required by the second stage of jump instructions.
///
/// Each is (internally) responsible for knowing/determining
/// the correct way to decode [`OpData`](enum.OpData.html).
pub type BranchAction = fn(&mut EECore, &BranchOpCode) -> BranchResult;

bitflags!{
#[derive(Default)]
pub struct BranchResult: u8 {
	const NULLIFIED = 0b0000_0001;
	const BRANCHED  = 0b0000_0010;
}
}

/// The queued form of a CPU instruction.
///
/// This contains the necessary data for execution, a function pointer,
/// and a delay. An action is held in the relevant pipeline until its delay
/// expires.
#[derive(Clone, Copy)]
pub struct OpCode {
	pub raw: u32,
	pub action: EEAction,
	pub delay: u8,
	pub requirements: Requirement<Capability>,
}

impl Default for OpCode {
	fn default() -> Self {
		Self {
			raw: 0,
			action: ops::nop as EEAction,
			delay: 0,
			requirements: Default::default(),
		}
	}
}

impl std::fmt::Debug for OpCode {
	fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
		fmt.debug_struct("OpCode")
			.field("raw", &format!("{:032b}", self.raw))
			.field("delay", &self.delay)
			.field("action", &"<pointer>")
			.field("requirements", &self.requirements)
			.finish()
	}
}

impl Instruction for OpCode {
	#[inline]
	fn get_opcode(&self) -> u8 {
		self.raw.get_opcode()
	}

	#[inline]
	fn ri_get_source(&self) -> u8 {
		self.raw.ri_get_source()
	}
	#[inline]
	fn ri_get_target(&self) -> u8 {
		self.raw.ri_get_target()
	}
	#[inline]
	fn r_get_destination(&self) -> u8 {
		self.raw.r_get_destination()
	}
	#[inline]
	fn r_get_shift_amount(&self) -> u8 {
		self.raw.r_get_shift_amount()
	}
	#[inline]
	fn r_get_function(&self) -> u8 {
		self.raw.r_get_function()
	}

	#[inline]
	fn i_get_immediate(&self) -> u16 {
		self.raw.i_get_immediate()
	}

	#[inline]
	fn i_get_immediate_signed(&self) -> i16 {
		self.raw.i_get_immediate_signed()
	}

	#[inline]
	fn j_get_jump(&self) -> u32 {
		self.raw.j_get_jump()
	}

	#[inline]
	fn set_opcode(&mut self, v: u8) {
		self.raw.set_opcode(v);
	}

	#[inline]
	fn ri_set_source(&mut self, v: u8) {
		self.raw.ri_set_source(v);
	}
	#[inline]
	fn ri_set_target(&mut self, v: u8) {
		self.raw.ri_set_target(v);
	}
	#[inline]
	fn r_set_destination(&mut self, v: u8) {
		self.raw.r_set_destination(v);
	}
	#[inline]
	fn r_set_shift_amount(&mut self, v: u8) {
		self.raw.r_set_shift_amount(v);
	}
	#[inline]
	fn r_set_function(&mut self, v: u8) {
		self.raw.r_set_function(v);
	}

	#[inline]
	fn i_set_immediate(&mut self, v: u16) {
		self.raw.i_set_immediate(v);
	}

	#[inline]
	fn j_set_jump(&mut self, v: u32) {
		self.raw.j_set_jump(v);
	}
}

/// The queued form of a CPU instruction.
///
/// This contains the necessary data for execution, a function pointer,
/// and a delay. An action is held in the relevant pipeline until its delay
/// expires.
#[derive(Clone, Copy)]
pub struct BranchOpCode {
	pub raw: u32,
	pub action: BranchAction,
	pub temp: u32,
}

impl BranchOpCode {
	pub fn new(basis: &OpCode, action: BranchAction, temp: u32) -> Self {
		Self {
			raw: basis.raw,
			action,
			temp,
		}
	}
}

impl Instruction for BranchOpCode {
	#[inline]
	fn get_opcode(&self) -> u8 {
		self.raw.get_opcode()
	}

	#[inline]
	fn ri_get_source(&self) -> u8 {
		self.raw.ri_get_source()
	}
	#[inline]
	fn ri_get_target(&self) -> u8 {
		self.raw.ri_get_target()
	}
	#[inline]
	fn r_get_destination(&self) -> u8 {
		self.raw.r_get_destination()
	}
	#[inline]
	fn r_get_shift_amount(&self) -> u8 {
		self.raw.r_get_shift_amount()
	}
	#[inline]
	fn r_get_function(&self) -> u8 {
		self.raw.r_get_function()
	}

	#[inline]
	fn i_get_immediate(&self) -> u16 {
		self.raw.i_get_immediate()
	}

	#[inline]
	fn i_get_immediate_signed(&self) -> i16 {
		self.raw.i_get_immediate_signed()
	}

	#[inline]
	fn j_get_jump(&self) -> u32 {
		self.raw.j_get_jump()
	}

	#[inline]
	fn set_opcode(&mut self, v: u8) {
		self.raw.set_opcode(v);
	}

	#[inline]
	fn ri_set_source(&mut self, v: u8) {
		self.raw.ri_set_source(v);
	}
	#[inline]
	fn ri_set_target(&mut self, v: u8) {
		self.raw.ri_set_target(v);
	}
	#[inline]
	fn r_set_destination(&mut self, v: u8) {
		self.raw.r_set_destination(v);
	}
	#[inline]
	fn r_set_shift_amount(&mut self, v: u8) {
		self.raw.r_set_shift_amount(v);
	}
	#[inline]
	fn r_set_function(&mut self, v: u8) {
		self.raw.r_set_function(v);
	}

	#[inline]
	fn i_set_immediate(&mut self, v: u16) {
		self.raw.i_set_immediate(v);
	}

	#[inline]
	fn j_set_jump(&mut self, v: u32) {
		self.raw.j_set_jump(v);
	}
}

bitflags!{
/// Bitfield representing pipe dependencies for instruction issue.
///
/// This consists of several combinations:
///  * Physical pipes as the building blocks (I0, I1, ...).
///  * Pipe sets for joint physical requirements.
/// In the name of optimsation, aliases should be considered "joint".
#[derive(Default)]
pub struct Pipe: u64 {
	/// Physical Integer 0 pipe.
	const I0 = 0b00_0001;
	/// Physical Integer 1 pipe.
	const I1 = 0b00_0010;
	/// Physical Load/Store pipe.
	const LS = 0b00_0100;
	/// Physical Branch pipe.
	const BR = 0b00_1000;
	/// Physical COP1 pipe.
	const C1 = 0b01_0000;
	/// Physical COP2 pipe.
	const C2 = 0b10_0000;

	/// Multimedia (128-bit) instructions. Joint.
	const WIDE_OPERATE = Self::I0.bits
		| Self::I1.bits;

	/// Most floating-point commands (COP1). Joint.
	const COP1_OPERATE = Self::I0.bits
		| Self::C1.bits;

	/// Move, Load/Store to COP1 (FPC). Joint.
	const COP1_MOVE = Self::LS.bits
		| Self::C1.bits;

	/// VU Operations (COP2). Joint.
	const COP2_OPERATE = Self::I0.bits
		| Self::C2.bits;

	/// Move, Load/Store to COP2 (VU). Joint.
	const COP2_MOVE = Self::LS.bits
		| Self::C2.bits;

	/// Mask for all pipes compatible with the "first instruction
	/// slot" in dual issue mode.
	const LOGICAL0 = Self::I0.bits
		| Self::BR.bits
		| Self::C1.bits
		| Self::C2.bits;

	/// Mask for all pipes compatible with the "second instruction
	/// slot" in dual issue mode.
	const LOGICAL1 = Self::I1.bits
		| Self::LS.bits
		| Self::BR.bits
		| Self::C1.bits
		| Self::C2.bits;
}
}

impl Requirement<Pipe> {
	pub fn fuse_registers(self, mut register_list: Capability) -> Requirement<Capability> {
		use Requirement::*;
		match self {
			Joint(a) => {
				let pipes = a.bits();
				register_list.write |= pipes << Capability::PIPELINE_SHIFT;

				Joint(register_list)
			},
			Disjoint(a, b) => {
				let pipes = a.bits();
				let alt_pipes = b.bits();
				let mut alt_list = register_list;

				register_list.write |= pipes << Capability::PIPELINE_SHIFT;
				alt_list.write |= alt_pipes << Capability::PIPELINE_SHIFT;

				Disjoint(register_list, alt_list)
			},
		}
	}
}
