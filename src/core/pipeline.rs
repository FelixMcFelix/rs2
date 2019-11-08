use crate::core::{
	ops,
	EECore,
};

pub type EEAction = fn(&mut EECore, &OpData)->();

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

pub type JumpOpData = u32;

pub enum Pipeline {
	Free,
	Reserved,
	Active(OpCode),
}