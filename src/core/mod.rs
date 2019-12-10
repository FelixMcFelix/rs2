mod constants;
mod ops;
pub mod pipeline;
#[cfg(test)]
mod tests;

use byteorder::{
	LittleEndian,
	ByteOrder,
};
use constants::*;
use ops::NOP;
use pipeline::*;
use super::memory::constants::*;

pub struct EECore {
	pub register_file: [u8; REGISTER_FILE_SIZE],
	pub hi: [u8; REGISTER_WIDTH_BYTES],
	pub lo: [u8; REGISTER_WIDTH_BYTES],
	pub sa_register: u32,
	pub pc_register: u32,

	/// Hack.
	///
	/// FIXME: exception logic should depend on COP0 state.
	pub exception: bool,

	/// Code held by a Branch-type instruction.
	///
	/// This should be executed and cleared *before* any instruction
	/// passed via [`EECore::execute`](#method.execute).
	/// This is not the instruction *in* the branch delay slot,
	/// but instead indicates that the next call to [`execute`](#method.execute) is such.
	pub branch_delay_slot_active: Option<BranchOpCode>,
}

impl EECore {
	/// Create a new instance of an EE Core Processor, including any necessary state (pipelines, register file, etc.)
	pub fn new() -> Self {
		Self {
			register_file: [0u8; REGISTER_FILE_SIZE],
			hi: [0u8; REGISTER_WIDTH_BYTES],
			lo: [0u8; REGISTER_WIDTH_BYTES],
			sa_register: 0,
			pc_register: KSEG1_START as u32,

			exception: false,

			branch_delay_slot_active: None,
		}
	}

	// So, The EE Core has 4 pipelines (6-stage) for instructions to be executed along.
	// Notably, the processor attempts to read and decode two instructions per clock cycle,
	// and performs handoff at the start. If both instructions fight over the same pipeline,
	// the second instruction is paused.

	// QUESTION: does each stage correspond to a clock signal? Particularly wrt. "phases".

	// NOTE: FPUs have different pipeline design (COP-0/COP-1)

	// Are I, Q, R stages shared between pipelines? i.e. specialisation occurs after R?

	// Figure 1-4 shows this clearly, there are two logical pipes. Stalled instructions are held between Q and R.
	// The physical pipes can belong to one or both logical pipes.
	// Moreover, some instructions need multiple PHYSICAL pipes (but one logical) (COP0/1 Move and operate)...
	// Aside from data-deps, some instruction pairs are illegal (branch+branch, branch+eret, and so on.)

	// Let me think... do these "six stages" all somehow take place sequentially in one clock cycle?
	// If stalling happens between Q -> R, ... oh.

	// ...except I don't CARE about queueing, branch prediction ect, which is what these are ALL ABOUT!

	// THINK bout register fetches (some need specificity over hi/lo bytes)

	// PS2 is a little-endian system. I suppose I need to care about this on the off chance that SOMEONE runs Big-Endian, somewhere...
	// Okay, how do Hi/Lo accesses work when handling different datatypes?
	// Loads + stores for datatypes demand struct-alignment in memory, otherwise an exception is thrown.
	// Misaligned data can be handled through specific instructions...

	/// Reads a value from the specified register.
	/// Note, register R0 will always return 0.
	pub fn read_register(&self, index: u8) -> u64 {
		trace!("Reading from register {}", index);
		let floor = ((index as usize) * REGISTER_WIDTH_BYTES) + HALF_REGISTER_WIDTH_BYTES;
		LittleEndian::read_u64(&self.register_file[floor..])
	}

	/// Write a value to the specified register.
	/// Writes to R0 will have NO effect.
	pub fn write_register(&mut self, index: u8, value: u64) -> Option<()> {
		trace!("Writing value {} to register {}", value, index);
		if index != 0 {
			let floor = ((index as usize) * REGISTER_WIDTH_BYTES) + HALF_REGISTER_WIDTH_BYTES;
			LittleEndian::write_u64(&mut self.register_file[floor..], value);
			Some(())
		} else {
			None
		}
	}

	/// Execute one cycle of the EE Core CPU.
	///
	/// This attempts to fetch and issue two instructions from memory.
	pub fn cycle(&mut self, program: &[u8]) {
		// Read and parse two instructions, put them into the pipeline.
		// FIXME: hack to avoid MMU during early BIOS testing.

		// FIXME: bound to 32-bit space.
		let pc: usize = self.pc_register.wrapping_sub(KSEG1_START as u32) as usize;
		trace!("{} <- {}",pc, self.pc_register);
		let next = self.pc_register.wrapping_add(OPCODE_LENGTH_BYTES as u32).wrapping_sub(KSEG1_START as u32) as usize;

		// FIXME: these checks will be slow/unnecessary once I move to real memory/mmu.
		// This is some ugly dupe, in the mean time.
		let i1 = if pc + OPCODE_LENGTH_BYTES <= program.len() {
			LittleEndian::read_u32(&program[pc..pc+OPCODE_LENGTH_BYTES])
		} else {
			NOP
		};
		let i2 = if next + OPCODE_LENGTH_BYTES <= program.len() {
			LittleEndian::read_u32(&program[next..next+OPCODE_LENGTH_BYTES])
		} else {
			NOP
		};

		// IDEA: Skip I, Q, lead in with R. Pass these off to the correct physical pipelines
		let p1 = ops::process_instruction(i1);
		self.execute(p1);
		// println!("{:?}, {:?}", p1.data, p1.delay);
		let p2 = ops::process_instruction(i2);
		self.execute(p2);
		// println!("{:?}, {:?}", p2.data, p2.delay);
	}

	pub fn execute(&mut self, instruction: OpCode) {
		let branch_result = if let Some(op) = self.branch_delay_slot_active.take() {
			(op.action)(self, &op)
		} else {
			BranchResult::empty()
		};

		if !branch_result.contains(BranchResult::NULLIFIED) {
			(instruction.action)(self, &instruction);
		}
		if !branch_result.contains(BranchResult::BRANCHED){
			self.pc_register = self.pc_register.wrapping_add(OPCODE_LENGTH_BYTES as u32);
		}
	}

	pub fn fire_exception(&mut self) {
		// Codes are on user's manual v6.0, p75/180.
		// FIXME: manipulate A LOT of COP0 state.
		self.exception = true;
	}

	pub fn in_exception(&mut self) -> bool {
		// FIXME: manipulate A LOT of COP0 state.
		self.exception
	}

	#[inline]
	pub fn branch(&mut self, op: &OpCode, new_action: &'static BranchAction, temp: u32) {
		let _ = self.branch_delay_slot_active.replace(BranchOpCode::new(
			op,
			new_action,
			temp,
		));
	}
}

impl Default for EECore {
	fn default() -> Self {
		Self::new()
	}
}
