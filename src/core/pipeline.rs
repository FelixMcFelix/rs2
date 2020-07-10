use bitflags::bitflags;
use crate::isa::mips::{Capability, Instruction, Requirement};
use std::cmp::{Ordering};
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

impl OpCode {
	pub fn make_live(self, time: u64) -> LiveAction {
		LiveAction {
			time: time + self.delay as u64 - 1,
			action: self,
		}
	}

	#[inline]
	pub fn needs_queue(&self) -> bool {
		self.delay != 1
	}

	#[inline]
	pub fn pipeline_fits(&self, cpu_cap: &Capability) -> Slot {
		self.requirements.pipeline_fits(cpu_cap)
	}
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

/// Instruction paired with an execution time.
///
/// Queued up internally for asynchronous execution where required.
pub struct LiveAction {
	pub time: u64,
	pub action: OpCode,
}

impl Eq for LiveAction {}

impl PartialEq for LiveAction {
	fn eq(&self, other: &Self) -> bool {
		self.time == other.time
	}
}

impl PartialOrd for LiveAction {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for LiveAction {
	fn cmp(&self, other: &Self) -> Ordering {
		self.time.cmp(&other.time)
	}
}

/// The second half of a branch instruction.
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

impl Requirement<Capability> {
	#[inline]
	pub fn pipeline_fits(&self, cpu_cap: &Capability) -> Slot {
		match self {
			Requirement::Joint(a) => {
				pipeline_capability_fits(cpu_cap, &a)
			},
			Requirement::Disjoint(a, b) => {
				let s1 = pipeline_capability_fits(cpu_cap, &a);
				let s2 = pipeline_capability_fits(cpu_cap, &b);

				s1.combine(s2)
			},
		}
	}
}

fn pipeline_capability_fits(cpu: &Capability, instr: &Capability) -> Slot {
	let read_mask = cpu.read & instr.read;
	let write_mask = cpu.write & instr.write;

	let valid = read_mask == instr.read && write_mask == instr.write;

	if valid {
		let pipes = write_mask >> Capability::PIPELINE_SHIFT;
		let fits_in_0 = (pipes & Pipe::LOGICAL0.bits()) == pipes;
		let fits_in_1 = (pipes & Pipe::LOGICAL1.bits()) == pipes;

		if fits_in_0 {
			if fits_in_1 {
				Slot::Either
			} else {
				Slot::Pipe0
			}
		} else if fits_in_1 {
			Slot::Pipe1
		} else {
			Slot::Both
		}
	} else {
		Slot::Neither
	}
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Slot {
	Neither,
	Pipe0,
	Pipe1,
	Either,
	Both,
}

impl Slot {
	pub fn combine(self, other: Self) -> Self {
		use Slot::*;

		match (self, other) {
			(Pipe0, Pipe1) | (Pipe1, Pipe0) => Either,
			(Neither, a) | (a, Neither) => a,
			a => {println!("{:?}",a);unreachable!()},
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::{
		core::{
			constants::requirements::*,
			cop0::Register,
		},
		isa::mips::{
			self,
			ee::*,
			Capability,
			Opcode as MipsOpcode,
		},
	};
	use super::*;

	#[test]
	fn ls_function_in_pipe_1() {
		assert_eq!(
			LS.fuse_registers(Default::default()).pipeline_fits(&Capability::all()),
			Slot::Pipe1,
		);
	}

	#[test]
	fn sync_function_in_pipe_1() {
		assert_eq!(
			SYNC.fuse_registers(Default::default()).pipeline_fits(&Capability::all()),
			Slot::Pipe1,
		);
	}

	#[test]
	fn lzc_function_in_pipe_1() {
		assert_eq!(
			LZC.fuse_registers(Default::default()).pipeline_fits(&Capability::all()),
			Slot::Pipe1,
		);
	}

	#[test]
	fn eret_function_in_pipe_1() {
		assert_eq!(
			ERET.fuse_registers(Default::default()).pipeline_fits(&Capability::all()),
			Slot::Pipe1,
		);
	}

	#[test]
	fn sa_function_in_pipe_0() {
		assert_eq!(
			SA.fuse_registers(Default::default()).pipeline_fits(&Capability::all()),
			Slot::Pipe0,
		);
	}

	#[test]
	fn cop0_function_in_pipe_1() {
		assert_eq!(
			COP0.fuse_registers(Default::default()).pipeline_fits(&Capability::all()),
			Slot::Pipe1,
		);
	}

	#[test]
	fn cop1_move_function_in_pipe_1() {
		assert_eq!(
			COP1_MOVE.fuse_registers(Default::default()).pipeline_fits(&Capability::all()),
			Slot::Pipe1,
		);
	}

	#[test]
	fn cop2_move_function_in_pipe_1() {
		assert_eq!(
			COP2_MOVE.fuse_registers(Default::default()).pipeline_fits(&Capability::all()),
			Slot::Pipe1,
		);
	}

	#[test]
	fn cop1_operate_function_in_pipe_0() {
		assert_eq!(
			COP1_OPERATE.fuse_registers(Default::default()).pipeline_fits(&Capability::all()),
			Slot::Pipe0,
		);
	}

	#[test]
	fn cop2_operate_function_in_pipe_0() {
		assert_eq!(
			COP2_OPERATE.fuse_registers(Default::default()).pipeline_fits(&Capability::all()),
			Slot::Pipe0,
		);
	}

	#[test]
	fn alu_function_in_either_slot() {
		assert_eq!(
			ALU.fuse_registers(Default::default()).pipeline_fits(&Capability::all()),
			Slot::Either,
		);
	}

	#[test]
	fn mac0_function_in_pipe_0() {
		assert_eq!(
			MAC0.fuse_registers(Default::default()).pipeline_fits(&Capability::all()),
			Slot::Pipe0,
		);
	}

	#[test]
	fn mac1_function_in_pipe_1() {
		assert_eq!(
			MAC1.fuse_registers(Default::default()).pipeline_fits(&Capability::all()),
			Slot::Pipe1,
		);
	}

	#[test]
	fn branch_function_in_either_slot() {
		assert_eq!(
			BRANCH.fuse_registers(Default::default()).pipeline_fits(&Capability::all()),
			Slot::Either,
		);
	}

	#[test]
	fn wide_function_in_both_slots() {
		assert_eq!(
			WIDE_OPERATE.fuse_registers(Default::default()).pipeline_fits(&Capability::all()),
			Slot::Both,
		);
	}

	#[test]
	fn alu_function_narrows() {
		// Suppose MAC0 or MAC1 are asyncronously holding up the I0/1 pipes.
		let mut mac0_in_use = Capability::all();
		mac0_in_use.write &= !(Pipe::I0.bits() << Capability::PIPELINE_SHIFT);

		assert_eq!(
			ALU.fuse_registers(Default::default()).pipeline_fits(&mac0_in_use),
			Slot::Pipe1,
		);

		let mut mac1_in_use = Capability::all();
		mac1_in_use.write &= !(Pipe::I1.bits() << Capability::PIPELINE_SHIFT);

		assert_eq!(
			ALU.fuse_registers(Default::default()).pipeline_fits(&mac1_in_use),
			Slot::Pipe0,
		);
	}

	#[test]
	fn shared_pipe_consumes_both() {
		let mut pipe_in_use = Capability::all();
		pipe_in_use.write &= !(Pipe::BR.bits() << Capability::PIPELINE_SHIFT);

		assert_eq!(
			BRANCH.fuse_registers(Default::default()).pipeline_fits(&pipe_in_use),
			Slot::Neither,
		);
	}

	#[test]
	fn mfc0_with_register_deps_in_pipe_1() {
		// Suppose MAC0 or MAC1 are asyncronously holding up the I0/1 pipes.
		let opcode = ops::process_instruction(
			mips::build_op_register_custom(
				MipsOpcode::Cop0, 0, MF0, 26, Register::PRId as u8, 0
			)
		);

		assert_eq!(
			opcode.pipeline_fits(&Capability::all()),
			Slot::Pipe1,
		);
	}
}
