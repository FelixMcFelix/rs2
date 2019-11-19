use crate::core::{
	ops,
	EECore,
};

/// The function signature required by any CPU instruction.
///
/// Each is (internally) responsible for knowing/determining
/// the correct way to decode [`OpData`](enum.OpData.html).
pub type EEAction = fn(&mut EECore, &OpData)->();

/// The queued form of a CPU instruction.
///
/// This contains the necessary data for execution, a function pointer,
/// and a delay. An action is held in the relevant pipeline until its delay
/// expires.
pub struct OpCode {
	pub data: OpData,
	pub action: &'static EEAction,
	pub delay: u8,
}

impl Default for OpCode {
	fn default() -> Self {
		Self {
			data: OpData::NoData,
			action: &(ops::nop as EEAction),
			delay: 0,
		}
	}
}

/// The type of the instruction being executed, and any function parameters.
///
/// Callee [`EEAction`s](type.EEAction.html) are responsible for knowing how to properly decode
/// the internal data.
#[derive(Debug)]
pub enum OpData {
	NoData,
	Immediate(ImmediateOpData),
	Register(RegisterOpData),
	Jump(JumpOpData),
}

impl OpData {
	pub fn immediate(instruction: u32) -> Self {
		Self::Immediate(ImmediateOpData::new(instruction))
	}

	pub fn register(instruction: u32) -> Self {
		Self::Register(RegisterOpData::new(instruction))
	}
}

/// Datatype containing the fields needed to execute an I-type instruction.
#[derive(Debug)]
pub struct ImmediateOpData {
	pub source: u8,
	pub target: u8,
	pub immediate: u16,
}

impl ImmediateOpData {
	pub fn new(instruction: u32) -> Self {
		let immediate = (instruction & 0xFF_FF) as u16;
		let target = ((instruction >> 16) & 0b00011111) as u8;
		let source = ((instruction >> 21) & 0b00011111) as u8;

		Self {
			source,
			target,
			immediate,
		}
	}
}

/// Datatype containing the fields needed to execute an R-type instruction.
#[derive(Debug)]
pub struct RegisterOpData {
	pub source: u8,
	pub target: u8,
	pub destination: u8,
	pub shift_amount: u8,
}

impl RegisterOpData {
	pub fn new(instruction: u32) -> Self {
		let shift_amount = ((instruction >> 6)  & 0b00011111) as u8;
		let destination = ((instruction >> 11) & 0b00011111) as u8;
		let target = ((instruction >> 16) & 0b00011111) as u8;
		let source = ((instruction >> 21) & 0b00011111) as u8;

		Self {
			source,
			target,
			destination,
			shift_amount,
		}
	}
}

/// Datatype containing the fields needed to execute a J-type instruction.
pub type JumpOpData = u32;

/// Currently unused.
pub enum Pipeline {
	Free,
	Reserved,
	Active(OpCode),
}