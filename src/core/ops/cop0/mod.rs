pub mod cache;

use crate::{
	core::{
		cop0::{
			Register,
			Status,
		},
		exceptions::L1Exception,
		pipeline::*,
		EECore,
	},
	isa::mips::Instruction,
};

#[inline(always)]
fn cop0_usable(cpu: &mut EECore) -> bool {
	// We need EITHER:
	// * Kernel mode.
	// * COP0 Usable.
	// If neither, also throw a "Coprocessor Unusable" exception.
	let status = Status::from_bits_truncate(cpu.read_cop0_direct(Register::Status as u8));
	let non_k_usable = status.contains(Status::COP0_USABLE);

	let valid = non_k_usable || status.privilege_level().is_kernel();

	if !valid {
		cpu.throw_l1_exception(L1Exception::CoprocessorUnusable(0));
	}

	valid
}

pub fn mfc0(cpu: &mut EECore, data: &OpCode) {
	if !cop0_usable(cpu) {
		return;
	}

	// load sign extended value of COP0[rd] into rt.
	let v = cpu.read_cop0(data.r_get_destination()) as i32;
	cpu.write_register(data.ri_get_target(), v as u64);
}

pub fn mtc0(cpu: &mut EECore, data: &OpCode) {
	if !cop0_usable(cpu) {
		return;
	}

	// store 32 lsbs of GPR[rt] into COP0[rd]
	let v = cpu.read_register(data.ri_get_target()) as u32;
	cpu.write_cop0(data.r_get_destination(), v);
}

pub fn tlbwi(cpu: &mut EECore, _data: &OpCode) {
	if !cop0_usable(cpu) {
		return;
	}

	// read the required registers, then update the mmu.
	cpu.mmu.write_index(
		cpu.read_cop0_direct(Register::EntryHi as u8),
		cpu.read_cop0_direct(Register::EntryLo0 as u8),
		cpu.read_cop0_direct(Register::EntryLo1 as u8),
	);
}

pub fn tlbwr(cpu: &mut EECore, _data: &OpCode) {
	if !cop0_usable(cpu) {
		return;
	}

	// read the required registers, then update the mmu.
	cpu.mmu.write_random(
		cpu.read_cop0_direct(Register::Random as u8),
		cpu.read_cop0_direct(Register::EntryHi as u8),
		cpu.read_cop0_direct(Register::EntryLo0 as u8),
		cpu.read_cop0_direct(Register::EntryLo1 as u8),
	);
}

#[cfg(test)]
mod tests {
	use super::*;
	use byteorder::{
		ByteOrder,
		LittleEndian,
	};
	use crate::{
		core::{
			constants::*,
			cop0::{
				self,
				Register,
			},
			ops,
		},
		isa::mips::{
			self,
			ee::*,
			Function as MipsFunction,
			Instruction,
			Opcode as MipsOpcode,
			RegImmFunction,
			NOP,
		},
		memory::constants::*,
		utils::*,
	};

	#[test]
	fn basic_mfc0() {
		let mut test_ee = EECore::default();

		install_and_run_program(&mut test_ee, instructions_to_bytes(&vec![
			mips::build_op_register_custom(MipsOpcode::Cop0, 0, MF0, 1, Register::PRId as u8, 0),
		]));

		assert_eq!(test_ee.read_register(1), EE_PRID as u64);
	}

	#[test]
	fn basic_mtc0() {
		let test_val = 0x1234_5678;
		let mut test_ee = EECore::default();

		test_ee.write_register(1, test_val as u64);

		install_and_run_program(&mut test_ee, instructions_to_bytes(&vec![
			mips::build_op_register_custom(MipsOpcode::Cop0, 0, MT0, 1, Register::EntryHi as u8, 0),
		]));

		assert_eq!(test_ee.read_cop0_direct(Register::EntryHi as u8), test_val);
	}

	#[test]
	fn basic_tlbwi() {
		let test_val = 0x1234_5678;
		let mut test_ee = EECore::default();

		test_ee.write_register(1, test_val as u64);

		let vpn2 = 0;
		let asid = 0;
		let pfn = 320;
		let pfn2 = 160;
		let index = 4;

		test_ee.write_cop0(Register::Index as u8, index);
		test_ee.write_cop0(Register::PageMask as u8, 0);
		test_ee.write_cop0(Register::EntryHi as u8, cop0::entry_hi_from_parts(vpn2, asid));
		// scratchpad: bool, pfn: u32, cache_mode: u8, dirty: bool, valid: bool, global: bool{
		test_ee.write_cop0(Register::EntryLo0 as u8, cop0::entry_lo_from_parts(false, pfn, 2, true, true, true));
		test_ee.write_cop0(Register::EntryLo1 as u8, cop0::entry_lo_from_parts(false, pfn2, 2, true, true, true));

		install_and_run_program(&mut test_ee, instructions_to_bytes(&vec![
			mips::build_op_register_custom(MipsOpcode::Cop0, C0Function::TlbWI as u8, C0, 1, Register::EntryHi as u8, 0),
		]));

		let line = test_ee.mmu.tlb.lines[index as usize];

		assert_eq!(line.mask, 0);
		assert_eq!(line.virtual_page_number_half, 0);
		assert_eq!(line.global, true);
		assert_eq!(line.asid, asid);

		assert_eq!(line.even.page_frame_number, pfn);
		assert_eq!(line.odd.page_frame_number, pfn2);
	}

	#[test]
	fn basic_tlbwr() {
		let test_val = 0x1234_5678;
		let mut test_ee = EECore::default();

		test_ee.write_register(1, test_val as u64);

		let vpn2 = 0;
		let asid = 0;
		let pfn = 320;
		let pfn2 = 160;
		let index = 4;

		test_ee.write_cop0(Register::Index as u8, index);
		test_ee.write_cop0(Register::PageMask as u8, 0);
		test_ee.write_cop0(Register::EntryHi as u8, cop0::entry_hi_from_parts(vpn2, asid));
		// scratchpad: bool, pfn: u32, cache_mode: u8, dirty: bool, valid: bool, global: bool{
		test_ee.write_cop0(Register::EntryLo0 as u8, cop0::entry_lo_from_parts(false, pfn, 2, true, true, true));
		test_ee.write_cop0(Register::EntryLo1 as u8, cop0::entry_lo_from_parts(false, pfn2, 2, true, true, true));

		install_and_run_program(&mut test_ee, instructions_to_bytes(&vec![
			NOP,
			NOP,
			NOP,
			NOP,
			NOP,
			NOP,
			mips::build_op_register_custom(MipsOpcode::Cop0, C0Function::TlbWR as u8, C0, 1, Register::EntryHi as u8, 0),
		]));

		// NOTE: random changed after executing TlbWR
		let random = test_ee.read_cop0_direct(Register::Random as u8) + 1;

		let line = test_ee.mmu.tlb.lines[random as usize];

		assert_eq!(line.mask, 0);
		assert_eq!(line.virtual_page_number_half, 0);
		assert_eq!(line.global, true);
		assert_eq!(line.asid, asid);

		assert_eq!(line.even.page_frame_number, pfn);
		assert_eq!(line.odd.page_frame_number, pfn2);
	}

	#[test]
	fn cop0_always_usable_in_kernel() {
		let mut test_ee = EECore::new();

		let mut status = Status::from_bits_truncate(test_ee.read_cop0_direct(Register::Status as u8));
		status.remove(Status::COP0_USABLE);
		test_ee.write_cop0_direct(Register::Status as u8, status.bits());

		install_and_run_program(&mut test_ee, instructions_to_bytes(&vec![
			mips::build_op_register_custom(MipsOpcode::Cop0, 0, MF0, 1, Register::PRId as u8, 0),
		]));

		assert!(!test_ee.in_exception());
	}

	#[test]
	fn cop0_needs_enabled_in_usermode() {
		let mut allowed_ee = EECore::new();
		let mut forbidden_ee = EECore::new();

		let program = instructions_to_bytes(&vec![
			mips::build_op_register_custom(MipsOpcode::Cop0, 0, MF0, 1, Register::PRId as u8, 0),
		]);

		// Tricky test. Need to map some space in USEG to SPRAM, write a program there...
		for test_ee in [&mut allowed_ee, &mut forbidden_ee].iter_mut() {
			let mut status = Status::from_bits_truncate(test_ee.read_cop0_direct(Register::Status as u8));
			status.remove(Status::COP0_USABLE);
			status.insert(Status::USER_MODE);
			test_ee.write_cop0_direct(Register::Status as u8, status.bits());

			let hi = 0b0000_0000_0000_0000_0000_0000_0000_0000;
			let lo0 = cop0::entry_lo_from_parts(true, 0, 0, true, true, true);
			let lo1 = cop0::entry_lo_from_parts(false, 0, 0, true, true, true);
			test_ee.write_cop0(Register::PageMask as u8, 0);
			test_ee.write_cop0(Register::Index as u8, 0);
			test_ee.mmu.write_index(hi, lo0, lo1);

			// 16KiB from 0x0 has been mapped.
			test_ee.write_memory(0, &program);

			test_ee.pc_register = 0;
		}

		let mut status = Status::from_bits_truncate(allowed_ee.read_cop0_direct(Register::Status as u8));
		status.insert(Status::COP0_USABLE);
		allowed_ee.write_cop0_direct(Register::Status as u8, status.bits());

		for test_ee in [&mut allowed_ee, &mut forbidden_ee].iter_mut() {
			test_ee.cycle();
		}

		assert!(forbidden_ee.in_exception());
		assert!(!allowed_ee.in_exception());
	}
}