pub mod vectors {
	pub const RESET_NMI: u32    = 0xBFC0_0000;
	pub const TLB_REFILL: u32   = 0x8000_0000;
	pub const TLB_REFILL_B: u32 = 0xBFC0_0200;
	pub const COUNTER: u32      = 0x8000_0080;
	pub const COUNTER_D: u32    = 0xBFC0_0280;
	pub const DEBUG: u32        = 0x8000_0100;
	pub const DEBUG_D: u32      = 0xBFC0_0300;
	pub const COMMON: u32       = 0x8000_0180;
	pub const COMMON_B: u32     = 0xBFC0_0380;
	pub const INTERRUPT: u32    = 0x8000_0200;
	pub const INTERRUPT_B: u32  = 0xBFC0_0400;
}

use enum_primitive::*;
use super::{
	cop0::{
		Cause,
		Register,
		Status,
	},
	mode::{
		ExceptionLevel,
		PrivilegeLevel,
	},
	EECore,
};
use vectors::*;

// See the below notes on why I can't just `enum_from_prmitive`.

/// Level 1 exception codes which can/will be thrown by instructions.
///
/// NOTE: several of these share the same exception code, and have
/// behaviour which is context/call-site sensitive.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum L1Exception {
	Interrupt(u8),
	TlbModified(u32),
	TlbFetchLoadRefill(u32), // 2
	TlbStoreRefill(u32), // 3
	TlbFetchLoadInvalid(u32), // 2
	TlbStoreInvalid(u32), // 3
	AddressErrorFetchLoad(u32),
	AddressErrorStore(u32),
	BusErrorFetch,
	BusErrorLoadStore,
	Systemcall,
	Break,
	ReservedInstruction,
	CoprocessorUnusable(u8),
	Overflow,
	Trap,
}

impl From<L1Exception> for u8 {
	fn from(v: L1Exception) -> u8 {
		use L1Exception::*;

		match v {
			Interrupt(_) => 0,
			TlbModified(_) => 1,
			TlbFetchLoadRefill(_) => 2,
			TlbStoreRefill(_) => 3,
			TlbFetchLoadInvalid(_) => 2,
			TlbStoreInvalid(_) => 3,
			AddressErrorFetchLoad(_) => 4,
			AddressErrorStore(_) => 5,
			BusErrorFetch => 6,
			BusErrorLoadStore => 7,
			Systemcall => 8,
			Break => 9,
			ReservedInstruction => 10,
			CoprocessorUnusable(_) => 11,
			Overflow => 12,
			Trap => 13,
		}
	}
}

impl L1Exception {
	pub fn to_exception_code(self) -> u8 {
		u8::from(self)
	}

	pub fn to_exception_vector(self, status: Status) -> u32 {
		use L1Exception::*;
		use PrivilegeLevel::*;
		use ExceptionLevel::*;

		let b_set = status.contains(Status::B_EXCEPTION_VECTOR);

		match self {
			Interrupt(_) => if b_set { INTERRUPT_B } else { INTERRUPT },
			TlbFetchLoadRefill(_) | TlbStoreRefill(_) => match status.privilege_level() {
				Kernel(Level1) => if b_set { COMMON_B } else { COMMON },
				_ => if b_set { TLB_REFILL_B } else { TLB_REFILL },
			},
			_ => if b_set { COMMON_B } else { COMMON },
		}
	}

	// pub fn prep_cop0(self, status)

	pub fn specific_handling(self, cpu: &mut EECore, cause: &mut Cause) {
		use L1Exception::*;

		match self {
			Interrupt(code) => {
				// NOTE: assumes this is well-formed.
				// i.e., one bit, 0--7.
				let interrupt_bits = 1 << (code + 8);
				cause.insert(Cause::from_bits_truncate(interrupt_bits));
			},
			TlbModified(addr) => {
				bad_v_addr(cpu, addr);
				fill_ctx_entryhi(cpu, addr);
			},
			TlbFetchLoadRefill(addr) | TlbStoreRefill(addr) |
			TlbFetchLoadInvalid(addr) | TlbStoreInvalid(addr) => {
				bad_v_addr(cpu, addr);
				fill_ctx_entryhi(cpu, addr);

				// TODO
				// Store new tlb entry/index in "Random" Cop0 register.
				unimplemented!()
			},
			AddressErrorStore(addr) | AddressErrorFetchLoad(addr) => {
				bad_v_addr(cpu, addr);
			},
			CoprocessorUnusable(cop_code) => {
				// Sets Cause::COPROCESSOR_NUMBER.
				let cop_code_bits = (cop_code as u32) << 28;
				cause.insert(Cause::from_bits_truncate(cop_code_bits));
			},
			_ => {},
		}
	}
}

#[inline]
fn bad_v_addr(cpu: &mut EECore, addr: u32) {
	cpu.write_cop0(Register::BadVAddr as u8, addr);
}

#[inline]
fn fill_ctx_entryhi(cpu: &mut EECore, addr: u32) {
	// Fill out COntext Register w/ 19 hi-order bits
	// and on page table address.

	// Fill out EntryHi using addr and current ASID.
	unimplemented!()
}

enum_from_primitive!{
/// Level 2 exception codes which can/will be thrown by instructions.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum L2Exception {
	Reset = 0,
	Nmi,
	PerformanceCounter,
	Debug = 4,
}
}

impl L2Exception {
	pub fn to_exception_code(self) -> u8 {
		self as u8
	}

	pub fn to_exception_vector(self, status: Status) -> u32 {
		use L2Exception::*;

		let d_set = status.contains(Status::D_EXCEPTION_VECTOR);

		match self {
			Reset | Nmi => RESET_NMI,
			PerformanceCounter => if d_set { COUNTER_D } else { COUNTER },
			Debug => if d_set { DEBUG_D } else { DEBUG },
		}
	}

	pub fn specific_handling(self, cpu: &mut EECore, status: &mut Status) {
		use L2Exception::*;

		match self {
			Reset => {
				status.insert(Status::B_EXCEPTION_VECTOR);
				status.remove(Status::BUS_ERROR_MASK);
				cpu.write_cop0(Register::Random as u8, 47);
				cpu.write_cop0(Register::Wired as u8, 0);

				// TODO
				// disable config die, ice, dce, nbe, bpe
				// set ccr.cte = 0
				// disable bpc iae, dre, dwe

				// set valid, dirty, lrf, lock bits of D$ to 0
				// set valid, lrf bits of I$ to t0
				unimplemented!()
			},
			Nmi => {
				status.insert(Status::B_EXCEPTION_VECTOR);
			},
			_ => {},
		}
	}
}