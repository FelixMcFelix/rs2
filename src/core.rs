use byteorder::{
	BigEndian,
	LittleEndian,
	ReadBytesExt,
};
use std::{
	fs::File,
	io::Read,
};

pub const REGISTER_WIDTH: usize = 128;
pub const REGISTER_WIDTH_BYTES: usize = REGISTER_WIDTH / 8;
pub const REGISTER_COUNT: usize = 32;
pub const REGISTER_FILE_SIZE: usize = REGISTER_COUNT * REGISTER_WIDTH_BYTES;

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

	/// 
	pub fn read_register(&self, index: usize) -> u64 {
		0
	}

	/// Write a value to the specified register. Writes to R0 will have NO effect.
	pub fn write_register(&mut self, index: usize) -> Option<()> {
		Some(())
	}

	pub fn cycle(&mut self, program: &[u8]) {
		// Read and parse two instructions, put them into the pipeline.

		&program[pc..pc+OPCODE_LENGTH_BYTES].read_u32::<LittleEndian>();
		&program[pc+OPCODE_LENGTH_BYTES..pc+OPCODE_LENGTH_BYTES+OPCODE_LENGTH_BYTES].read_u32::<LittleEndian>();

		// IDEA: Skip I, Q, lead in with R. Pass these off to the correct physical pipelines
	}

	// pub fn read_instruction

}

// PLAN: decode and read two-at-a-time, then place in pipelines like nothing happened.
// Store action with latency or at least a wake-up time. Main issue is then capturing register state at the time that these things should be entering e.g. the ALU.

const OPCODE_LENGTH_BYTES: usize = 4;

struct OpCode {
	op: OpCodeType,
	action: &'static fn()->(),
}

enum OpCodeType {
	Immediate(ImmediateOpCode),
	Register(RegisterOpCode),
	Jump(JumpOpcode),
}

struct ImmediateOpCode{
	source: u8,
	target: u8,
	immediate: u16,
}

struct RegisterOpCode {
	source: u8,
	target: u8,
	destination: u8,
	shift_amount: u8,
}

type JumpOpcode = u32;

enum Pipeline {
	Free,
	Reserved,
	Active(OpCode),
}