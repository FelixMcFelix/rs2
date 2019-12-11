//! Various constants pertaining to the EE Core.

use enum_primitive::*;

pub const OPCODE_LENGTH_BYTES: usize = 4;
pub const REGISTER_WIDTH: usize = 128;
pub const REGISTER_WIDTH_BYTES: usize = REGISTER_WIDTH / 8;
pub const HALF_REGISTER_WIDTH_BYTES: usize = REGISTER_WIDTH_BYTES / 2;
pub const REGISTER_COUNT: usize = 32;
pub const REGISTER_FILE_SIZE: usize = REGISTER_COUNT * REGISTER_WIDTH_BYTES;

pub const COP0_REGISTER_WIDTH: usize = 32;
pub const COP0_REGISTER_WIDTH_BYTES: usize = COP0_REGISTER_WIDTH / 8;
pub const COP0_REGISTER_FILE_SIZE: usize = REGISTER_COUNT * COP0_REGISTER_WIDTH_BYTES;

pub const IOP_PRID: u32 = 0x00;
pub const EE_PRID: u32  = 0x2e;

enum_from_primitive!{
/// Names of COP0 Registers, defined according to page 64 of the EE Core User's Manual.
/// Values which cannot be converted map to "Reserved" registers (7, 17–22, 26–27, 31).
#[derive(Debug, PartialEq)]
pub enum EECop0Register {
	/// MMU Register—TLB Entry for read/write.
	Index = 0,

	/// MMU Register—PRNG Index for replacement in TLB.
	Random,

	/// MMU Register—TLB Entry low half (even PFN).
	EntryLo0,

	/// MMU Register—TLB Entry low half (odd PFN)
	EntryLo1,

	/// MMU Register—PTE table address.
	Context,

	/// MMU Register—Page size mask: MSBs of TLB entry.
	PageMask,

	/// MMU Register—Wired TLB entry count.
	Wired,

	/// Exception Register—Bad virtual address.
	BadVAddr = 8,

	/// MMU Register—Timer Comparison.
	Count,

	/// MMU Register—VPN, ASID of TLB Entry. High half.
	EntryHi,

	/// Exception Register—Timer reference.
	Compare,

	/// Exception Register—Status of processor.
	Status,

	/// Exception Register—Result of last exception.
	Cause,

	/// Exception Register—Exception PC.
	EPC,

	/// MMU Register—Processor Revision. See [`IOP_PRID`](constant.IOP_PRID.html),
	/// [`EE_PRID`](constant.EE_PRID.html).
	PRId,

	/// MMU Register—Configuration.
	Config,

	/// Exception Register—Bad Physical Address.
	BadPAddr = 23,

	/// Debug Registers.
	/// There are supposedly 7 such registers behind this.
	Debug,

	/// Performance counters and control register.
	/// There are supposedly 3 such registers behind this.
	Perf,

	/// Cache Register—Low bits of tag.
	TagLo = 28,

	/// Cache Register—High bits of tag.
	TagHi,

	/// Exception Register—Error Exception PC.
	ErrorEPC,
}
}
