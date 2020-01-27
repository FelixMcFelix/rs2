pub mod cache;

use crate::core::{
	cop0::{
		Register,
		Status,
	},
	exceptions::L1Exception,
	pipeline::*,
	EECore,
};
use super::instruction::Instruction;

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

pub fn tlbwi(cpu: &mut EECore, data: &OpCode) {
	if !cop0_usable(cpu) {
		return;
	}

	// read the required registers, then update the mmu.
	cpu.mmu.write_index(
		cpu.read_cop0_direct(Register::PageMask as u8),
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
			cop0::Register,
			ops::{
				self,
				constants::*,
			},
		},
		memory::constants::*,
	};

	#[test]
	fn basic_mfc0() {
		let mut test_ee = EECore::default();

		// FIXME: design some cleaner way of creating COP0 codes.
		let mut instruction = 0;
		instruction.set_opcode(MipsOpcode::Cop0 as u8);
		instruction.ri_set_target(1);
		// FIXME: make constant for each COP0 register.
		instruction.r_set_destination(Register::PRId as u8);

		test_ee.execute(ops::process_instruction(instruction));

		assert_eq!(test_ee.read_register(1), EE_PRID as u64);
	}

	#[test]
	fn basic_mtc0() {
		unimplemented!()
	}

	#[test]
	fn basic_tlbwi() {
		unimplemented!()
	}

	#[test]
	fn cop0_always_usable_in_kernel() {
		unimplemented!()
	}

	#[test]
	fn cop0_needs_enabled_in_usermode() {
		unimplemented!()
	}
}