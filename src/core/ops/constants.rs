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
