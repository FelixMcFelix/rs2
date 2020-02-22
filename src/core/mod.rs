pub mod constants;
pub mod cop0;
pub mod exceptions;
pub mod mode;
pub mod ops;
pub mod pipeline;
#[cfg(test)]
mod tests;

use byteorder::{
	LittleEndian,
	ByteOrder,
};
use constants::*;
use cop0::*;
use enum_primitive::*;
use exceptions::{
	L1Exception,
	L2Exception,
};
use mode::PrivilegeLevel;
use pipeline::*;
use super::memory::{
	constants::*,
	mmu::{
		self,
		Mmu,
		MmuAddress,
	},
	Memory,
};

pub struct EECore {
	pub register_file: [u8; REGISTER_FILE_SIZE],
	pub cop0_register_file: [u8; COP0_REGISTER_FILE_SIZE],
	pub hi: [u8; REGISTER_WIDTH_BYTES],
	pub lo: [u8; REGISTER_WIDTH_BYTES],
	pub sa_register: u32,
	pub pc_register: u32,

	pub memory: Memory,
	pub mmu: Mmu,

	/// Code held by a Branch-type instruction.
	///
	/// This should be executed and cleared *before* any instruction
	/// passed via [`EECore::execute`](#method.execute).
	/// This is not the instruction *in* the branch delay slot,
	/// but instead indicates that the next call to [`execute`](#method.execute) is such.
	pub branch_delay_slot_active: Option<BranchOpCode>,

	/// Whether dual issue of instructions is enabled or disabled.
	pub dual_issue: bool,

	excepted_this_cycle: bool,
}

impl EECore {
	/// Create a new instance of an EE Core Processor, including any necessary state (pipelines, register file, etc.)
	pub fn new() -> Self {
		Self {
			register_file: [0u8; REGISTER_FILE_SIZE],
			cop0_register_file: [0u8; COP0_REGISTER_FILE_SIZE],
			hi: [0u8; REGISTER_WIDTH_BYTES],
			lo: [0u8; REGISTER_WIDTH_BYTES],
			sa_register: 0,
			pc_register: BIOS_START as u32,

			memory: Memory::new(vec![0;4]),
			mmu: Default::default(),

			branch_delay_slot_active: None,

			dual_issue: false,

			excepted_this_cycle: false,
		}
	}

	pub fn set_bios(&mut self, mut bios: Vec<u8>) {
		bios.resize(BIOS_LEN as usize, 0);
		self.memory.set_bios(bios);
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

	/// Reads the value of the HI register.
	pub fn read_hi(&self) -> u64 {
		trace!("Reading from HI");

		LittleEndian::read_u64(&self.hi[..])
	}

	/// Write a value to the HI register.
	pub fn write_hi(&mut self, value: u64) {
		trace!("Writing value {} to HI", value);

		LittleEndian::write_u64(&mut self.hi[..], value);
	}

	/// Reads half of the HI register,
	/// where `index` is `0` or `1`.
	pub fn read_hi_half(&self, index: u8) -> u32 {
		trace!("Reading from HI{}", index);

		let offset = if index == 0 {
			HALF_REGISTER_WIDTH_BYTES
		} else {
			0
		};

		LittleEndian::read_u32(&self.hi[offset..])
	}

	/// Write a value to half of the HI register,
	/// where `index` is `0` or `1`.
	pub fn write_hi_half(&mut self, index: u8, value: u32) {
		trace!("Writing value {} to HI{}", value, index);

		let offset = if index == 0 {
			HALF_REGISTER_WIDTH_BYTES
		} else {
			0
		};

		LittleEndian::write_u32(&mut self.hi[offset..], value);
	}

	/// Reads the value of the LO register.
	pub fn read_lo(&self) -> u64 {
		trace!("Reading from LO");

		LittleEndian::read_u64(&self.lo[..])
	}

	/// Write a value to the LO register.
	pub fn write_lo(&mut self, value: u64) {
		trace!("Writing value {} to LO", value);

		LittleEndian::write_u64(&mut self.lo[..], value);
	}

	/// Reads half of the LO register,
	/// where `index` is `0` or `1`.
	pub fn read_lo_half(&self, index: u8) -> u32 {
		trace!("Reading from LO{}", index);

		let offset = if index == 0 {
			HALF_REGISTER_WIDTH_BYTES
		} else {
			0
		};

		LittleEndian::read_u32(&self.lo[offset..])
	}

	/// Write a value to half of the LO register,
	/// where `index` is `0` or `1`.
	pub fn write_lo_half(&mut self, index: u8, value: u32) {
		trace!("Writing value {} to LO{}", value, index);

		let offset = if index == 0 {
			HALF_REGISTER_WIDTH_BYTES
		} else {
			0
		};

		LittleEndian::write_u32(&mut self.lo[offset..], value);
	}

	/// Reads a value from the specified register of COP0.
	pub fn read_cop0(&self, index: u8) -> u32 {
		let value = self.read_cop0_direct(index);

		#[cfg(debug_assertions)]
		{
			let printable = format_cop0(value, index);
			trace!("Reading {:?} from COP0 register {} ({:?})", printable, index, Register::from_u8(index));
		}

		value
	}

	/// Reads a value from the specified register of COP0.
	#[inline]
	pub fn read_cop0_direct(&self, index: u8) -> u32 {
		let floor = (index as usize) * COP0_REGISTER_WIDTH_BYTES;
		LittleEndian::read_u32(&self.cop0_register_file[floor..])
	}

	/// Write a value to the specified register of COP0.
	pub fn write_cop0(&mut self, index: u8, value: u32) {
		let bmask = cop0::get_writable_bitmask(index);
		let value = (value & bmask) | (self.read_cop0_direct(index) & !bmask);
		#[cfg(debug_assertions)]
		{
			let printable = format_cop0(value, index);
			trace!("Writing value {:?} ({:032b}) to COP0 register {} ({:?})", printable, value, index, Register::from_u8(index));
		}
		self.write_cop0_direct(index, value);
	}

	/// Write a value to the specified register of COP0.
	#[inline]
	pub fn write_cop0_direct(&mut self, index: u8, value: u32) {
		let floor = (index as usize) * COP0_REGISTER_WIDTH_BYTES;
		LittleEndian::write_u32(&mut self.cop0_register_file[floor..], value);

		match Register::from_u8(index) {
			Some(Register::Config) => self.update_config(value),
			Some(Register::Index) => self.mmu.index = value as u8,
			Some(Register::PageMask) => self.mmu.page_mask = value,
			Some(Register::Wired) => {
				self.mmu.wired = value as u8;
				self.write_cop0_direct(Register::Random as u8, RANDOM_DEFAULT);
			},
			_ => {},
		}
	}

	fn update_config(&mut self, value: u32) {
		let config = Config::from_bits_truncate(value);

		self.dual_issue = config.contains(Config::ENABLE_DUAL_ISSUE);
	}

	pub fn init_as_ee(&mut self) {
		self.write_cop0_direct(Register::Wired as u8, WIRED_DEFAULT);
		self.write_cop0_direct(Register::Random as u8, RANDOM_DEFAULT);
		self.write_cop0_direct(Register::PRId as u8, EE_PRID);
		self.write_cop0_direct(Register::Status as u8, Status::default().bits());
		self.write_cop0_direct(Register::Config as u8, Config::default().bits());
	}

	pub fn init_as_iop(&mut self) {
		
	}

	pub fn read_memory(&mut self, v_addr: u32, size: usize) -> Option<&[u8]> {
		if self.access_virtual_address(v_addr, true) {
			let p_addr = self.translate_virtual_address(v_addr, true);
			p_addr.map(move |real_p| self.memory.read(real_p, size))
		} else {
			None
		}
	}

	pub fn read_memory_mut(&mut self, v_addr: u32, size: usize) -> Option<&mut [u8]> {
		if self.access_virtual_address(v_addr, false) {
			let p_addr = self.translate_virtual_address(v_addr, false);
			p_addr.map(move |real_p| self.memory.read_mut(real_p, size))
		} else {
			None
		}
	}

	pub fn write_memory(&mut self, v_addr: u32, data: &[u8]) {
		if self.access_virtual_address(v_addr, false) {
			let p_addr = self.translate_virtual_address(v_addr, false);
			if let Some(p_addr) = p_addr {
				self.memory.write(p_addr, data);
			}
		}
	}

	pub fn translate_virtual_address(&mut self, v_addr: u32, load: bool) -> Option<MmuAddress> {
		match v_addr {
			KSEG0_START..=KSEG0_END => Some(MmuAddress::Address(v_addr - (KSEG0_START as u32))),
			KSEG1_START..=KSEG1_END => Some(MmuAddress::Address(v_addr - (KSEG1_START as u32))),
			_ => match self.mmu.translate_address(v_addr, load) {
				MmuAddress::Exception(e) => {
					self.throw_l1_exception(e);
					None
				},
				a@_ => Some(a),
			},
		}
	}

	/// Determine whether an address can be served in the current processor mode.
	///
	/// This will fire address sxceptions if access violations occur.
	#[inline]
	pub fn access_virtual_address(&mut self, v_addr: u32, load: bool) -> bool {
		let out = match v_addr {
			USEG_START..=USEG_END => true,
			SSEG_START..=SSEG_END => self.get_current_privilege() != PrivilegeLevel::User,
			_ => self.get_current_privilege().is_kernel(),
		};

		if !out {
			if load {
				self.throw_l1_exception(L1Exception::AddressErrorFetchLoad(v_addr as u32));
			} else {
				self.throw_l1_exception(L1Exception::AddressErrorStore(v_addr as u32));
			}
		}

		out
	}

	/// Execute one cycle of the EE Core CPU.
	///
	/// This attempts to fetch and issue two instructions from memory.
	pub fn cycle(&mut self) {
		// Timer interrupt.
		// FIXME: does this happen befpre or after execution?
		let count = self.read_cop0_direct(Register::Count as u8).wrapping_add(1);
		self.write_cop0_direct(Register::Count as u8, count);
		if count == self.read_cop0_direct(Register::Compare as u8) {
			// FIXME: magic number
			// FIXME: needs to be masked.
			self.throw_l1_exception(L1Exception::Interrupt(7))
		}

		// Read and parse two instructions, put them into the pipeline.
		// FIXME: hack to avoid MMU during early BIOS testing.

		// FIXME: bound to 32-bit space.
		let pc = self.pc_register;
		trace!("PC: {:08x}", self.pc_register);

		let ops = self.read_memory(pc, 2 * OPCODE_LENGTH_BYTES).unwrap();

		// FIXME: these checks will be slow/unnecessary once I move to real memory/mmu.
		// This is some ugly dupe, in the mean time.
		let i1 = LittleEndian::read_u32(&ops);
		let i2 = LittleEndian::read_u32(&ops[OPCODE_LENGTH_BYTES..]);

		// p1 may accidentally enable this...
		let dual_issue = self.dual_issue;

		let p1 = ops::process_instruction(i1);
		self.execute(p1);

		if dual_issue {
			let p2 = ops::process_instruction(i2);
			self.execute(p2);
		}

		self.excepted_this_cycle = false;

		// Decrement TLB's random destination once per cycle w/ instruction execution.
		// FIXME: only if one or more fired...
		let old_random = self.read_cop0_direct(Register::Random as u8);
		let wired = self.mmu.wired as u32;
		let new_random = if old_random == wired {RANDOM_DEFAULT} else {old_random - 1};
		self.write_cop0_direct(Register::Random as u8, new_random);
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
		if !(branch_result.contains(BranchResult::BRANCHED) || self.excepted_this_cycle) {
			self.pc_register = self.pc_register.wrapping_add(OPCODE_LENGTH_BYTES as u32);
		}
	}

	pub fn in_exception(&mut self) -> bool {
		self.get_current_privilege().is_in_exception()
	}

	pub fn throw_l1_exception(&mut self, ex: L1Exception) {
		self.excepted_this_cycle = true;

		trace!("L1 Exception: {:?}", ex);

		// Switch to kernel mode (EXL).
		// Save addresses (put PC into Cop0::EPC, set cause.bd if necessary)
		//  Don't save if in handler already.
		//  Put PC of triggering instruction into EPC if not BD, else put preceding instruction.
		// Set exception cause codes mandated by each exception.
		// Jump to the vector.
		let status = self.read_cop0_direct(Register::Status as u8);
		let mut status = Status::from_bits_truncate(status);

		// Set exception code.
		let mut cause = self.read_cop0_direct(Register::Cause as u8);
		cause &= !Cause::EXCEPTION_CODE_L1.bits();
		cause |= (ex.to_exception_code() as u32) << 2;
		let mut cause = Cause::from_bits_truncate(cause);

		// NOTE: official docs make it unclear whether COMMON is taken for all
		// in EX1, or just for the TLB instructions.
		// Pseudocode suggests all, text suggests only TLB.
		if !status.contains(Status::EXCEPTION_LEVEL) {
			status.insert(Status::EXCEPTION_LEVEL);

			let saved_addr = if self.branch_delay_slot_active.is_none() {
				cause.insert(Cause::BRANCH_DELAY_1);
				self.pc_register
			} else {
				cause.remove(Cause::BRANCH_DELAY_1);
				self.pc_register - OPCODE_LENGTH_BYTES as u32
			};

			trace!("placing 0x{:08x} in EPC", saved_addr);
			self.write_cop0_direct(Register::EPC as u8, saved_addr);
			self.write_cop0_direct(Register::Status as u8, status.bits());
		}

		ex.specific_handling(self, &mut cause);
		self.write_cop0_direct(Register::Cause as u8, cause.bits());
		self.pc_register = ex.to_exception_vector(status);
	}

	pub fn throw_l2_exception(&mut self, ex: L2Exception) {
		self.excepted_this_cycle = true;

		trace!("L2 Exception: {:?}", ex);

		let status = self.read_cop0_direct(Register::Status as u8);
		let mut status = Status::from_bits_truncate(status);

		// FIRST: set code.
		let mut cause = self.read_cop0_direct(Register::Cause as u8);
		cause &= !Cause::EXCEPTION_CODE_L2.bits();
		cause |= (ex as u32) << 16;
		let mut cause = Cause::from_bits_truncate(cause);

		// NOTE: official docs make it unclear whether COMMON is taken for all
		// in EX1, or just for the TLB instructions.
		// Pseudocode suggests all, text suggests only TLB.
		if !status.contains(Status::ERROR_LEVEL) {
			status.insert(Status::ERROR_LEVEL);

			let saved_addr = if self.branch_delay_slot_active.is_none() {
				cause.insert(Cause::BRANCH_DELAY_2);
				self.pc_register
			} else {
				cause.remove(Cause::BRANCH_DELAY_2);
				self.pc_register - OPCODE_LENGTH_BYTES as u32
			};

			self.write_cop0_direct(Register::ErrorEPC as u8, saved_addr);
		}

		// write back status, cause.
		self.write_cop0_direct(Register::Cause as u8, cause.bits());

		self.pc_register = ex.to_exception_vector(status);

		ex.specific_handling(self, &mut status);
		self.write_cop0_direct(Register::Status as u8, status.bits());
	}

	#[inline]
	pub fn branch(&mut self, op: &OpCode, new_action: BranchAction, temp: u32) {
		let _ = self.branch_delay_slot_active.replace(BranchOpCode::new(
			op,
			new_action,
			temp,
		));
	}

	pub fn get_current_privilege(&self) -> PrivilegeLevel {
		Status::from_bits_truncate(
			self.read_cop0_direct(Register::Status as u8)
		).privilege_level()
	}
}

fn format_cop0(value: u32, register: u8) -> Box<dyn core::fmt::Debug> {
	match Register::from_u8(register) {
		Some(Register::Config) => Box::new(Config::from_bits_truncate(value)),
		Some(Register::Status) => Box::new(Status::from_bits_truncate(value)),
		Some(Register::Cause) => Box::new(Cause::from_bits_truncate(value)),
		Some(Register::EntryHi) => Box::new(
			format!("asid: {}, vpn2: {:05x}", value.get_asid(), EntryHi::get_vpn2(value))
		),
		Some(Register::Context) => Box::new(
			format!("pte_base: {:03x}, vpn2: {:05x}", value.get_pte_base(), Context::get_vpn2(value))
		),
		Some(Register::EntryLo0) => Box::new(
			format!("scratchpad: {}, pfn: {:05x}, cache_mode: {}, d: {}, v: {}, g: {}",
				value.is_scratchpad(), value.get_pfn(), value.get_cache_mode(),
				value.is_dirty() ,value.is_valid(), value.is_global(),
			)
		),
		Some(Register::EntryLo1) => Box::new(
			format!("pfn: {:05x}, cache_mode: {}, d: {}, v: {}, g: {}",
				value.get_pfn(), value.get_cache_mode(),
				value.is_dirty() ,value.is_valid(), value.is_global(),
			)
		),
		Some(Register::PageMask) => Box::new(
			format!("page_size: {}", mmu::page_mask_size(value))
		),
		_ => Box::new(value),
	}
}

impl Default for EECore {
	fn default() -> Self {
		let mut out = Self::new();
		out.init_as_ee();

		out
	}
}
