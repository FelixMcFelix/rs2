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
