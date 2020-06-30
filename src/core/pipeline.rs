use bitflags::bitflags;
use crate::isa::mips::Instruction;
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
}

impl Default for OpCode {
	fn default() -> Self {
		Self {
			raw: 0,
			action: ops::nop as EEAction,
			delay: 0,
		}
	}
}

impl std::fmt::Debug for OpCode {
	fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
		fmt.debug_struct("OpCode")
			.field("raw", &format!("{:032b}", self.raw))
			.field("delay", &self.delay)
			.field("action", &"<pointer>")
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

/// Currently unused.
pub enum Pipeline {
	Free,
	Reserved,
	Active(OpCode),
}