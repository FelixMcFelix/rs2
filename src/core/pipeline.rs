use super::{
	ops::{
		self,
		instruction::Instruction,
	},
	EECore,
};

/// The function signature required by any CPU instruction.
///
/// Each is (internally) responsible for knowing/determining
/// the correct way to decode [`OpData`](enum.OpData.html).
pub type EEAction = fn(&mut EECore, &OpCode)->();

/// The queued form of a CPU instruction.
///
/// This contains the necessary data for execution, a function pointer,
/// and a delay. An action is held in the relevant pipeline until its delay
/// expires.
pub struct OpCode {
	pub raw: u32,
	pub action: &'static EEAction,
	pub delay: u8,
}

impl Default for OpCode {
	fn default() -> Self {
		Self {
			raw: 0,
			action: &(ops::nop as EEAction),
			delay: 0,
		}
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