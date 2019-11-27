use super::{
	ops,
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

/// Currently unused.
pub enum Pipeline {
	Free,
	Reserved,
	Active(OpCode),
}