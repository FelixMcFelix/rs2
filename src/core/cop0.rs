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

	/// Int[1] Interrupt mask.
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
		| Self::COP2_USABLE.bits
		| Self::COP3_USABLE.bits;
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

bitflags!{
/// Flags contained within COP0's cause register.
/// These are defined within the *EE Core User's Manual 6.0*, pp.75.
pub struct Cause: u32 {
	const EXCEPTION_CODE_L1_B0    = 0b0000_0000_0000_0000_0000_0000_0000_0100;
	const EXCEPTION_CODE_L1_B1    = 0b0000_0000_0000_0000_0000_0000_0000_1000;
	const EXCEPTION_CODE_L1_B2    = 0b0000_0000_0000_0000_0000_0000_0001_0000;
	const EXCEPTION_CODE_L1_B3    = 0b0000_0000_0000_0000_0000_0000_0010_0000;
	const EXCEPTION_CODE_L1_B4    = 0b0000_0000_0000_0000_0000_0000_0100_0000;

	/// 5-bit field determining the level 1 exception code.
	const EXCEPTION_CODE_L1 = Self::EXCEPTION_CODE_L1_B0.bits
		| Self::EXCEPTION_CODE_L1_B1.bits
		| Self::EXCEPTION_CODE_L1_B2.bits
		| Self::EXCEPTION_CODE_L1_B3.bits
		| Self::EXCEPTION_CODE_L1_B4.bits;

	/// Set when an I[0] interrupt is pending.
	const PENDING_INTERRUPT_I1    = 0b0000_0000_0000_0000_0000_0100_0000_0000;

	/// Set when an I[1] interrupt is pending.
	const PENDING_INTERRUPT_I0    = 0b0000_0000_0000_0000_0000_1000_0000_0000;

	/// Set when a timer interrupt is pending.
	const PENDING_INTERRUPT_TIMER = 0b0000_0000_0000_0000_1000_0000_0000_0000;

	const EXCEPTION_CODE_L2_B0    = 0b0000_0000_0000_0001_0000_0000_0000_0000;
	const EXCEPTION_CODE_L2_B1    = 0b0000_0000_0000_0010_0000_0000_0000_0000;
	const EXCEPTION_CODE_L2_B2    = 0b0000_0000_0000_0100_0000_0000_0000_0000;

	/// 3-bit field determining the level 2 exception code.
	const EXCEPTION_CODE_L2 = Self::EXCEPTION_CODE_L2_B0.bits
		| Self::EXCEPTION_CODE_L2_B1.bits
		| Self::EXCEPTION_CODE_L2_B2.bits;

	const COPROCESSOR_B0          = 0b0001_0000_0000_0000_0000_0000_0000_0000;
	const COPROCESSOR_B1          = 0b0010_0000_0000_0000_0000_0000_0000_0000;

	/// 2-bit field determining the Coprocessor responsible for a
	/// "Coprocessor Unusable" Exception.
	const COPROCESSOR_NUMBER = Self::COPROCESSOR_B0.bits
		| Self::COPROCESSOR_B1.bits;

	/// Set if a level 2 exception (not including reset) occurs from an instruction
	/// placed in the branch delay slot.
	const BRANCH_DELAY_2          = 0b0100_0000_0000_0000_0000_0000_0000_0000;

	/// Set if a level 1 exception  occurs from an instruction
	/// placed in the branch delay slot.
	const BRANCH_DELAY_1          = 0b1000_0000_0000_0000_0000_0000_0000_0000;
}
}


bitflags!{
/// Flags contained within COP0's config register.
/// These are defined within the *EE Core User's Manual 6.0*, pp.78.
pub struct Config: u32 {
	const KSEG0_CACHE_MODE_B0      = 0b0000_0000_0000_0000_0000_0000_0000_0001;
	const KSEG0_CACHE_MODE_B1      = 0b0000_0000_0000_0000_0000_0000_0000_0010;
	const KSEG0_CACHE_MODE_B2      = 0b0000_0000_0000_0000_0000_0000_0000_0100;

	/// 3-bit field denoting the current cache mode of KSEG0.
	///
	/// This has the pattern:
	/// * `000` => *Cached w/o writeback & write allocate*
	/// * `010` => *Uncached*
	/// * `011` => *Cached w/ writeback & write allocate*
	/// * `111` => *Uncached, unaccelerated*
	const KSEG0_CACHE_MODE = Self::KSEG0_CACHE_MODE_B0.bits
		| Self::KSEG0_CACHE_MODE_B1.bits
		| Self::KSEG0_CACHE_MODE_B2.bits;

	const DATA_CACHE_B0            = 0b0000_0000_0000_0000_0000_0000_0100_0000;
	const DATA_CACHE_B1            = 0b0000_0000_0000_0000_0000_0000_1000_0000;
	const DATA_CACHE_B2            = 0b0000_0000_0000_0000_0000_0001_0000_0000;

	/// 3-bit field denoting data cache size (8kB blocks).
	///
	/// This should only be set to `001` (i.e., 8kB).
	const DATA_CACHE_SIZE = Self::DATA_CACHE_B0.bits
		| Self::DATA_CACHE_B1.bits
		| Self::DATA_CACHE_B2.bits;

	const INSTR_CACHE_B0           = 0b0000_0000_0000_0000_0000_0010_0000_0000;
	const INSTR_CACHE_B1           = 0b0000_0000_0000_0000_0000_0100_0000_0000;
	const INSTR_CACHE_B2           = 0b0000_0000_0000_0000_0000_1000_0000_0000;

	/// 3-bit field denoting instruction cache size (8kB blocks).
	///
	/// This should only be set to `010` (i.e., 16kB).
	const INSTR_CACHE_SIZE = Self::INSTR_CACHE_B0.bits
		| Self::INSTR_CACHE_B1.bits
		| Self::INSTR_CACHE_B2.bits;

	/// Enable branch prediction.
	const ENABLE_BRANCH_PREDICTION = 0b0000_0000_0000_0000_0001_0000_0000_0000;

	/// Enable non-blocking loads.
	const ENABLE_NB_LOAD           = 0b0000_0000_0000_0000_0010_0000_0000_0000;

	/// Enable data cache.
	const ENABLE_DATA_CACHE        = 0b0000_0000_0000_0001_0000_0000_0000_0000;

	/// Enable instruction cache.
	const ENABLE_INSTR_CACHE       = 0b0000_0000_0000_0010_0000_0000_0000_0000;

	/// Enable parallel issue of 2 instructions to pipelines.
	const ENABLE_DUAL_ISSUE        = 0b0000_0000_0000_0100_0000_0000_0000_0000;

	const BUS_CLOCK_RATIO_B0       = 0b0001_0000_0000_0000_0000_0000_0000_0000;
	const BUS_CLOCK_RATIO_B1       = 0b0010_0000_0000_0000_0000_0000_0000_0000;
	const BUS_CLOCK_RATIO_B2       = 0b0100_0000_0000_0000_0000_0000_0000_0000;

	/// 3-bit field denoting bus clock ratio.
	///
	/// Supposedly "value from dividing processor clock frequency by 2".
	const BUS_CLOCK_RATIO = Self::BUS_CLOCK_RATIO_B0.bits
		| Self::BUS_CLOCK_RATIO_B1.bits
		| Self::BUS_CLOCK_RATIO_B2.bits;
}
}

impl Default for Config {
	fn default() -> Self {
		Self::from_bits_truncate(0b0000_0000_0000_0000_0000_0100_0100_0000)
	}
}

/// Used to write-protect certain bits/fields
/// which should (from the perspecive of running code)
/// be immutable.
pub fn get_writable_bitmask(index: u8) -> u32 {
	match index {
		_ => 0xff_ff_ff_ff,
	}
}

pub const WIRED_DEFAULT: u32 = 0;

pub const RANDOM_DEFAULT: u32 = RANDOM_MAX;

// Note: these are the way they are because the tlb size is 48.
pub const RANDOM_MAX: u32 = 47;
pub const RANDOM_MOD: u32 = RANDOM_MAX + 1;

pub fn increment_random(random: u32) -> u32 {
	(random + 1) % RANDOM_MOD
}

pub trait EntryLo {
	fn is_scratchpad(self) -> bool;
	fn get_pfn(self) -> u32;
	fn get_cache_mode(self) -> u8;
	fn is_dirty(self) -> bool;
	fn is_valid(self) -> bool;
	fn is_global(self) -> bool;
}

impl EntryLo for u32 {
	fn is_scratchpad(self) -> bool {
		(self & 0x8000_0000) != 0
	}

	fn get_pfn(self) -> u32 {
		(self & 0x03ff_ffff) >> 6
	}

	fn get_cache_mode(self) -> u8 {
		((self & 0x0000_003f) >> 3) as u8
	}

	fn is_dirty(self) -> bool {
		(self & 0b100) != 0
	}

	fn is_valid(self) -> bool {
		(self & 0b010) != 0
	}

	fn is_global(self) -> bool {
		(self & 0b001) != 0
	}
}

pub fn entry_lo_from_parts(scratchpad: bool, pfn: u32, cache_mode: u8, dirty: bool, valid: bool, global: bool) -> u32 {
	((scratchpad as u32) << 31)
	| (pfn << 6)
	| ((cache_mode as u32) << 3)
	| ((dirty as u32) << 2)
	| ((valid as u32) << 1)
	| (global as u32)
}

pub trait Context {
	fn get_pte_base(self) -> u32;
	fn get_vpn2(self) -> u32;
}

impl Context for u32 {
	fn get_pte_base(self) -> u32 {
		self >> 23
	}

	fn get_vpn2(self) -> u32 {
		(self & 0x003f_ffff) >> 4
	}
}

pub fn context_from_parts(pte_base: u32, vpn2: u32) -> u32 {
	(pte_base << 23) | (vpn2 << 4)
}

pub trait EntryHi {
	fn get_asid(self) -> u8;
	fn get_vpn2(self) -> u32;
}

impl EntryHi for u32 {
	fn get_asid(self) -> u8 {
		self as u8
	}

	fn get_vpn2(self) -> u32 {
		self >> 13
	}
}

pub fn entry_hi_from_parts(vpn2: u32, asid: u8) -> u32 {
	(vpn2 << 13) | (asid as u32)
}