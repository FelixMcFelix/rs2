//! Various constants pertaining to the EE Core.

pub const OPCODE_LENGTH_BYTES: usize = 4;
pub const REGISTER_WIDTH: usize = 128;
pub const REGISTER_WIDTH_BYTES: usize = REGISTER_WIDTH / 8;
pub const HALF_REGISTER_WIDTH_BYTES: usize = REGISTER_WIDTH_BYTES / 2;
pub const REGISTER_COUNT: usize = 32;
pub const REGISTER_FILE_SIZE: usize = REGISTER_COUNT * REGISTER_WIDTH_BYTES;

pub const COP0_REGISTER_WIDTH: usize = 32;
pub const COP0_REGISTER_WIDTH_BYTES: usize = COP0_REGISTER_WIDTH / 8;
pub const COP0_REGISTER_FILE_SIZE: usize = REGISTER_COUNT * COP0_REGISTER_WIDTH_BYTES;

/// Implementation numbers and their relation to PRId are discussed in the
/// *EE Core User's Manual 6.0*, pp.77.
pub const EE_IMPL: u32  = 0x2e;
pub const IOP_IMPL: u32 = 0x00;

pub const EE_PRID: u32 = EE_IMPL << 8;
pub const IOP_PRID: u32 = IOP_IMPL << 8;

pub mod timings {
	//! Timings relating to operations and instructions.
	//! These are informed by the *EE Core User's Manual 6.0*,
	//! page 58.

	pub const INTEGER_SUM_LOGIC_DELAY: u8 = 1;
	pub const INTEGER_HI_LO_TRANSFER_DELAY: u8 = 1;
	pub const INTEGER_SHIFT_LUI_DELAY: u8 = 1;
	pub const INTEGER_BRANCH_JUMP_DELAY: u8 = 1;
	pub const INTEGER_CONDITIONAL_MOVE_DELAY: u8 = 1;
	pub const INTEGER_MULT_DELAY: u8 = 4;
	pub const INTEGER_DIV_DELAY: u8 = 37;
	pub const INTEGER_MADD_DELAY: u8 = 4;
	pub const INTEGER_LOAD_STORE_DELAY: u8 = 1;

	pub const FLOAT_MTC1_DELAY: u8 = 2;
	pub const FLOAT_ADD_NEG_COND_DELAY: u8 = 4;
	pub const FLOAT_CVT_DELAY: u8 = 4;
	pub const FLOAT_MUL_DELAY: u8 = 4;
	pub const FLOAT_MFC1_DELAY: u8 = 2;
	pub const FLOAT_MOVE_DELAY: u8 = 4;
	pub const FLOAT_DIV_DELAY: u8 = 8;
	pub const FLOAT_SQRT_DELAY: u8 = 8;
	pub const FLOAT_RSQRT_DELAY: u8 = 14;
	pub const FLOAT_MADD_DELAY: u8 = 4;
	pub const FLOAT_LWC1_DELAY: u8 = 2;
}

pub mod requirements {
	//! Constants containing the pipeline requirements for different
	//! instruction classes on the EE Core.

	use crate::{
		core::pipeline::Pipe,
		isa::mips::Requirement,
	};

	/// Pipeline requirement for Load/store, 128-bit load-store, prefetch, cache.
	pub const LS: Requirement<Pipe> = Requirement::Joint(Pipe::LS);

	/// Pipeline requirement for synchronisation.
	pub const SYNC: Requirement<Pipe> = Requirement::Joint(Pipe::I1);

	/// Pipeline requirement for Leading-Zero Count.
	pub const LZC: Requirement<Pipe> = Requirement::Joint(Pipe::I1);

	/// Pipeline requirement for exception return.
	pub const ERET: Requirement<Pipe> = Requirement::Joint(Pipe::I1);

	/// Pipeline requirement for moves to/from EE's SA register.
	pub const SA: Requirement<Pipe> = Requirement::Joint(Pipe::I0);

	/// Pipeline requirement for COP0 move/operate.
	pub const COP0: Requirement<Pipe> = Requirement::Joint(Pipe::LS);

	/// Pipeline requirement for COP1 (FPU) Loads/Stores.
	pub const COP1_MOVE: Requirement<Pipe> = Requirement::Joint(Pipe::COP1_MOVE);

	/// Pipeline requirement for COP1 (FPU) Operations.
	pub const COP1_OPERATE: Requirement<Pipe> = Requirement::Joint(Pipe::COP1_OPERATE);

	/// Pipeline requirement for COP1 (VU) Loads/Stores.
	pub const COP2_MOVE: Requirement<Pipe> = Requirement::Joint(Pipe::COP2_MOVE);

	/// Pipeline requirement for COP0 (VU) Operations.
	pub const COP2_OPERATE: Requirement<Pipe> = Requirement::Joint(Pipe::COP2_OPERATE);

	/// Pipeline requirement for Arithmetic, Shift, Logical, Trap, Syscall, Break.
	pub const ALU: Requirement<Pipe> = Requirement::Disjoint(Pipe::I0, Pipe::I1);

	/// Pipeline requirement for Multiply(-accumulate) and move for HI/LO.
	pub const MAC0: Requirement<Pipe> = Requirement::Joint(Pipe::I0);

	/// Pipeline requirement for Multiply(-accumulate) and move for HI1/LO1.
	pub const MAC1: Requirement<Pipe> = Requirement::Joint(Pipe::I1);

	/// Pipeline requirement for branch instructions.
	pub const BRANCH: Requirement<Pipe> = Requirement::Joint(Pipe::BR);

	/// Pipeline requirement for Multimedia (128-bit) instructions.
	pub const WIDE_OPERATE: Requirement<Pipe> = Requirement::Joint(Pipe::WIDE_OPERATE);
}
