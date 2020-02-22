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
			cop0::Register,
			ops::{
				self,
				constants::*,
			},
		},
		memory::constants::*,
		utils::*,
	};

	#[test]
	fn basic_mfc0() {
		let mut test_ee = EECore::default();

		install_and_run_program(&mut test_ee, instructions_to_bytes(&vec![
			ops::build_op_register_custom(MipsOpcode::Cop0, MipsFunction::SLL, MF0, 1, Register::PRId as u8, 0),
		]));

		assert_eq!(test_ee.read_register(1), EE_PRID as u64);
	}

	#[test]
	fn basic_mtc0() {
		let test_val = 0x1234_5678;
		let mut test_ee = EECore::default();

		test_ee.write_register(1, test_val as u64);

		install_and_run_program(&mut test_ee, instructions_to_bytes(&vec![
			ops::build_op_register_custom(MipsOpcode::Cop0, MipsFunction::SLL, MT0, 1, Register::EntryHi as u8, 0),
		]));

		assert_eq!(test_ee.read_cop0_direct(Register::EntryHi as u8), test_val);
	}

	#[test]
	fn basic_tlbwi() {
		unimplemented!()
	}

	#[test]
	fn basic_tlbwr() {
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