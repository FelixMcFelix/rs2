use crate::core::{
	pipeline::*,
	EECore,
};
use super::instruction::Instruction;

pub fn mfc0(cpu: &mut EECore, data: &OpCode) {
	// load sign extended value of COP0[rd] into rt.
	let v = cpu.read_cop0(data.r_get_destination()) as i32;
	cpu.write_register(data.ri_get_target(), v as u64);

	// FIXME: should except if unusable.
}

#[cfg(test)]
mod test {
	use super::*;
	use byteorder::{
		ByteOrder,
		LittleEndian,
	};
	use crate::{
		core::{
			constants::*,
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
		instruction.r_set_destination(15);

		test_ee.execute(ops::process_instruction(instruction));

		assert_eq!(test_ee.read_register(1), EE_PRID as u64);
	}
}