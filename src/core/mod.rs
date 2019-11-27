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
use pipeline::*;

pub struct EECore {
	pub register_file: [u8; REGISTER_FILE_SIZE],
	pub hi: [u8; REGISTER_WIDTH_BYTES],
	pub lo: [u8; REGISTER_WIDTH_BYTES],
	pub sa_register: u32,
	pub pc_register: u32,
}

impl EECore {
	/// Create a new instance of an EE Core Processor, including any necessary state (pipelines, register file, etc.)
	pub fn new() -> Self {
		Self {
			register_file: [0u8; REGISTER_FILE_SIZE],
			hi: [0u8; REGISTER_WIDTH_BYTES],
			lo: [0u8; REGISTER_WIDTH_BYTES],
			sa_register: 0,
			pc_register: 0,
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

	pub fn cycle(&mut self, program: &[u8]) {
		// Read and parse two instructions, put them into the pipeline.
		let pc = self.pc_register as usize;

		let i1 = LittleEndian::read_u32(&program[pc..pc+OPCODE_LENGTH_BYTES]);
		let i2 = LittleEndian::read_u32(&program[pc+OPCODE_LENGTH_BYTES..pc+OPCODE_LENGTH_BYTES+OPCODE_LENGTH_BYTES]);

		// IDEA: Skip I, Q, lead in with R. Pass these off to the correct physical pipelines
		let p1 = ops::process_instruction(i1);
		self.execute(p1);
		// println!("{:?}, {:?}", p1.data, p1.delay);
		let p2 = ops::process_instruction(i2);
		self.execute(p2);
		// println!("{:?}, {:?}", p2.data, p2.delay);

		self.pc_register += (OPCODE_LENGTH_BYTES as u32) << 1;
	}

	pub fn execute(&mut self, instruction: OpCode) {
		(instruction.action)(self, &instruction);
	}
}

impl Default for EECore {
	fn default() -> Self {
		Self::new()
	}
}
