use crate::{
	core::{
		pipeline::*,
		EECore,
	},
	utils::*,
};
use std::mem::size_of;
use super::instruction::Instruction;

pub fn mflo(cpu: &mut EECore, data: &OpCode) {
	// LO -> GPR[rd]
	cpu.write_register(data.r_get_destination(), cpu.read_lo());
}

pub fn lb(cpu: &mut EECore, data: &OpCode) {
	let offset: u32 = data.i_get_immediate_signed().s_ext();
	let v_addr = (cpu.read_register(data.ri_get_source()) as u32).wrapping_add(offset);

	trace!("I want to load byte from v_addr {:08x}",
		v_addr,
	);

	let loc = cpu.read_memory(v_addr as u32, size_of::<u8>())
		.map(|buf| buf[0]);

	if let Some(loc) = loc {
		cpu.write_register(data.ri_get_target(), loc.s_ext());
	}
}

pub fn lui(cpu: &mut EECore, data: &OpCode) {
	// load sign extended shifted value of immediate into rt.
	let v = i64::from(data.i_get_immediate());
	cpu.write_register(data.ri_get_target(), (v << 16) as u64);
}

#[cfg(test)]
mod test {
	use super::*;
	use byteorder::{
		ByteOrder,
		LittleEndian,
	};
	use crate::{
		core::ops::{
			self,
			constants::*,
		},
		memory::constants::*,
	};

	#[test]
	fn basic_mflo() {
		unimplemented!();
	}

	#[test]
	fn basic_lb() {
		unimplemented!();
	}

	#[test]
	fn basic_lui() {
		// Place a 16-bit value into bits 32..16.
		let in_1: i16 = 12345;

		let mut test_ee = EECore::new();

		let instruction = ops::build_op_immediate(MipsOpcode::LUI, 0, 1, in_1 as u16);

		test_ee.execute(ops::process_instruction(instruction));

		assert_eq!(test_ee.read_register(1) >> 16, in_1 as u64);
	}
}