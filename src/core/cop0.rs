use bitflags::bitflags;
use enum_primitive::*;
use super::mode::*;

enum_from_primitive!{
/// Names of COP0 Registers, defined according to page 64 of the EE Core User's Manual.
/// Values which cannot be converted map to "Reserved" registers (7, 17–22, 26–27, 31).
#[derive(Debug, PartialEq)]
pub enum Register {
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


bitflags!{
/// Flags contained within COP0's status register.
/// These are defined within the *EE Core User's Manual 6.0*, pp.73.
pub struct Status: u32 {
	/// Enable/disable (`1`/`0`) all interrupts.
	const INTERRUPT_ENABLE   = 0b0000_0000_0000_0000_0000_0000_0000_0001;

	/// Set by all exceptions, except reset, NMI, perf counter, and debug.
	const EXCEPTION_LEVEL    = 0b0000_0000_0000_0000_0000_0000_0000_0010;

	/// Set by reset, NMI, perf counter, and debug.
	const ERROR_LEVEL        = 0b0000_0000_0000_0000_0000_0000_0000_0100;

	const SUPERVISOR_MODE    = 0b0000_0000_0000_0000_0000_0000_0000_1000;
	const USER_MODE          = 0b0000_0000_0000_0000_0000_0000_0001_0000;

	/// Int[0] Interrupt mask.
	const INTERRUPT_MASK_2   = 0b0000_0000_0000_0000_0000_0100_0000_0000;

	/// Int[0] Interrupt mask.
	const INTERRUPT_MASK_3   = 0b0000_0000_0000_0000_0000_1000_0000_0000;

	/// Internal Timer Interrupt mask.
	const INTERRUPT_MASK_7   = 0b0000_0000_0000_0000_1000_0000_0000_0000;

	/// Signals (`0`) or masks (`1`) a bus error.
	const BUS_ERROR_MASK     = 0b0000_0000_0000_0000_0001_0000_0000_0000;

	/// Overrides [`INTERRUPT_ENABLE`](#associatedconstant.INTERRUPT_ENABLE), where `0` => ignore, and
	/// `1` => respect the field.
	///
	/// This flag is controlled by the COP0 `DI` and `EI` instructions.
	const ENABLE_IE          = 0b0000_0000_0000_0001_0000_0000_0000_0000;

	/// Allow (`1`) or disallow (`1`) the `EI` and `DI` COP0 instructions,
	/// which control [`ENABLE_IE`](#associatedconstant.ENABLE_IE).
	const ENABLE_EDI         = 0b0000_0000_0000_0010_0000_0000_0000_0000;

	/// Status of the must recent data cache invalidate instructions:
	/// `0` => miss, `1` => hit.
	const CACHE_HIT          = 0b0000_0000_0000_0100_0000_0000_0000_0000;

	/// Address of the "B" (?) exception vector. `1` => bootstrap vector.
	const B_EXCEPTION_VECTOR = 0b0000_0000_0100_0000_0000_0000_0000_0000;

	/// Address of the Debug/Perf Counter exception vector. `1` => bootstrap vector.
	const D_EXCEPTION_VECTOR = 0b0000_0000_1000_0000_0000_0000_0000_0000;

	const COP0_USABLE        = 0b0001_0000_0000_0000_0000_0000_0000_0000;
	const COP1_USABLE        = 0b0010_0000_0000_0000_0000_0000_0000_0000;
	const COP2_USABLE        = 0b0100_0000_0000_0000_0000_0000_0000_0000;
	const COP3_USABLE        = 0b1000_0000_0000_0000_0000_0000_0000_0000;

	/// 3-bit field denoting whether the interrupts
	/// [`INTERRUPT_MASK_2`](#associatedconstant.INTERRUPT_MASK_2),
	/// [`INTERRUPT_MASK_3`](#associatedconstant.INTERRUPT_MASK_3),
	/// and [`INTERRUPT_MASK_7`](#associatedconstant.INTERRUPT_MASK_7)
	/// are enabled (`1`) or disabled (`0`).
	const INTERRUPT_MASK = Self::INTERRUPT_MASK_2.bits
		| Self::INTERRUPT_MASK_3.bits
		| Self::INTERRUPT_MASK_7.bits;

	/// 2-bit field denoting the current privilege level.
	///
	/// This has the pattern:
	/// * `00` => *Kernel Mode*
	/// * `01` => *Supervisor Mode*
	/// * `10` => *User Mode*
	/// * `00` => *Undefined*
	///
	/// In theory then, just using the [`SUPERVISOR_MODE`](#associatedconstant.SUPERVISOR_MODE) and
	/// [`USER_MODE`](#associatedconstant.USER_MODE) flags should suffice.
	const PRIVILEGE_LEVEL = Self::SUPERVISOR_MODE.bits
		| Self::USER_MODE.bits;

	/// 4-bit field denoting which coprocessors are currently enabled: these
	/// are individually accessed via
	/// [`COP0_USABLE`](#associatedconstant.COP0_USABLE),
	/// [`COP1_USABLE`](#associatedconstant.COP1_USABLE),
	/// [`COP2_USABLE`](#associatedconstant.COP2_USABLE), and
	/// [`COP3_USABLE`](#associatedconstant.COP3_USABLE).
	///
	/// A value of `1` allows use outside of kernel mode.
	const COPS_USABLE = Self::COP0_USABLE.bits
		| Self::COP1_USABLE.bits
		| Self::COP1_USABLE.bits
		| Self::COP1_USABLE.bits;
}
}

impl Default for Status {
	fn default() -> Self {
		// Enable the bits defined in pp.73: i.e., Error level and BEV.
		// These are the only flags whose initial values are defined.
		Self::ERROR_LEVEL | Self::B_EXCEPTION_VECTOR
	}
}

impl Status {
	#[inline]
	pub fn privilege_level(self) -> PrivilegeLevel {
		// Exception levels trigger Kernel status: check for these, before considering
		// the privilege flags.
		if self.contains(Self::ERROR_LEVEL) {
			PrivilegeLevel::Kernel(ExceptionLevel::Level2)
		} else if self.contains(Self::EXCEPTION_LEVEL) {
			PrivilegeLevel::Kernel(ExceptionLevel::Level1)
		} else if self.contains(Self::USER_MODE) {
			PrivilegeLevel::User
		} else if self.contains(Self::SUPERVISOR_MODE) {
			PrivilegeLevel::Supervisor
		} else {
			PrivilegeLevel::Kernel(ExceptionLevel::NoException)
		}
	}
}