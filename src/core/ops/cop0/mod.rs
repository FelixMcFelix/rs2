pub mod cache;

use crate::core::{
	cop0::{
		Register,
		Status,
	},
	pipeline::*,
	EECore,
};
use super::instruction::Instruction;

fn cop0_usable(cpu: &mut EECore) -> bool {
	// We need EITHER:
	// * Kernel mode.
	// * COP0 Usable.
	// If neither, also throw a "Coprocessor Unusable" exception.
	let non_k_usable = Status::from_bits_truncate(cpu.read_cop0(Register::Status as u8))
		.contains(Status::COP0_USABLE);

	let valid = non_k_usable || cpu.get_current_privilege().is_kernel();

	if !valid {
		// FIXME: this must be a "Coprocessor Unusable" Exception.
		cpu.fire_exception();
	}

	valid
}

pub fn mfc0(cpu: &mut EECore, data: &OpCode) {
	// load sign extended value of COP0[rd] into rt.
	let v = cpu.read_cop0(data.r_get_destination()) as i32;
	cpu.write_register(data.ri_get_target(), v as u64);

	// FIXME: should except if COP0 unusable.
}

pub fn mtc0(cpu: &mut EECore, data: &OpCode) {
	// store 32 lsbs of GPR[rt] into COP0[rd]
	let v = cpu.read_register(data.ri_get_target()) as u32;
	cpu.write_cop0(data.r_get_destination(), v);

	// FIXME: should except if COP0 unusable.
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
		// On EE Core, this should be 0x2e.
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
}