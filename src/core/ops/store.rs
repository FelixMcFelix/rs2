use byteorder::{ByteOrder, LittleEndian};
use crate::{
	core::{
		exceptions::L1Exception,
		pipeline::*,
		EECore,
	},
	utils::*,
};
use std::mem::size_of;
use super::instruction::Instruction;

pub fn sw(cpu: &mut EECore, data: &OpCode) {
	// mem[GPR[rs] + signed(imm)] <- (GPR[rt] as 32)
	let to_store = cpu.read_register(data.ri_get_target()) as u32;
	let offset: u32 = data.i_get_immediate_signed().s_ext();
	let v_addr = (cpu.read_register(data.ri_get_source()) as u32).wrapping_add(offset);

	// FIXME: make size info part of address resolution.
	if v_addr & 0b11 != 0 {
		cpu.throw_l1_exception(L1Exception::AddressErrorStore(v_addr));
		return;
	}

	trace!("I want to store {} in v_addr {:08x}",
		to_store,
		v_addr,
	);

	if let Some(loc) = cpu.read_memory_mut(v_addr as u32, size_of::<u32>()) {
		LittleEndian::write_u32(loc, to_store);
	}

	// should except if ttarget addr is badly aligned.
	// unimplemented!()
}

pub fn sd(cpu: &mut EECore, data: &OpCode) {
	// mem[GPR[rs] + signed(imm)] <- (GPR[rt] as 64)
	let to_store = cpu.read_register(data.ri_get_target());
	let offset: u32 = data.i_get_immediate_signed().s_ext();
	let v_addr = (cpu.read_register(data.ri_get_source()) as u32).wrapping_add(offset);

	// FIXME: make size info part of address resolution.
	if v_addr & 0b111 != 0 {
		cpu.throw_l1_exception(L1Exception::AddressErrorStore(v_addr));
		return;
	}

	trace!("I want to store {} in v_addr {:08x}",
		to_store,
		v_addr,
	);

	if let Some(loc) = cpu.read_memory_mut(v_addr as u32, size_of::<u64>()) {
		LittleEndian::write_u64(loc, to_store);
	}
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
		utils::*,
	};

	#[test]
	fn basic_sw() {
		let stored_data: u32 = 0x1234_5678;

		let mut test_ee = EECore::new();

		test_ee.write_register(1, KSEG1_START.z_ext());
		test_ee.write_register(2, stored_data.s_ext());

		install_and_run_program(&mut test_ee, instructions_to_bytes(&vec![
			ops::build_op_immediate(MipsOpcode::SW, 1, 2, 0),
		]));

		assert_eq!(test_ee.read_memory(KSEG1_START, 4).map(|d| LittleEndian::read_u32(d)), Some(stored_data));
	}

	#[test]
	fn sw_minus_offset() {
		let stored_data: u32 = 0x1234_5678;
		let base_pointer = KSEG1_START + 128;

		let mut test_ee = EECore::default();

		test_ee.write_register(1, base_pointer.z_ext());
		test_ee.write_register(2, stored_data.s_ext());

		let offset = -20;

		install_and_run_program(&mut test_ee, instructions_to_bytes(&vec![
			ops::build_op_immediate(MipsOpcode::SW, 1, 2, offset as u16),
		]));

		assert_eq!(test_ee.read_memory(base_pointer - 20, 4).map(|d| LittleEndian::read_u32(d)), Some(stored_data));
	}

	#[test]
	fn sw_plus_offset() {
		let stored_data: u32 = 0x1234_5678;
		let base_pointer = KSEG1_START + 128;

		let mut test_ee = EECore::default();

		test_ee.write_register(1, base_pointer.z_ext());
		test_ee.write_register(2, stored_data.s_ext());

		install_and_run_program(&mut test_ee, instructions_to_bytes(&vec![
			ops::build_op_immediate(MipsOpcode::SW, 1, 2, 256),
		]));

		assert_eq!(test_ee.read_memory(base_pointer + 256, 4).map(|d| LittleEndian::read_u32(d)), Some(stored_data));
	}

	#[test]
	fn sd_4_byte_aligned() {
		let stored_data = 0x1234_5678;
		let base_pointer = KSEG1_START + 126;

		let mut test_ee = EECore::default();

		test_ee.write_register(1, base_pointer.z_ext());
		test_ee.write_register(2, stored_data);

		install_and_run_program(&mut test_ee, instructions_to_bytes(&vec![
			ops::build_op_immediate(MipsOpcode::SW, 1, 2, 0),
		]));

		assert!(test_ee.in_exception());
		assert_eq!(test_ee.read_memory(base_pointer, 4).map(|d| LittleEndian::read_u32(d)), Some(0));
	}

	#[test]
	fn basic_sd() {
		let stored_data: u64 = 0x1234_5678_9abc_def0;

		let mut test_ee = EECore::default();

		test_ee.write_register(1, KSEG1_START.z_ext());
		test_ee.write_register(2, stored_data);

		install_and_run_program(&mut test_ee, instructions_to_bytes(&vec![
			ops::build_op_immediate(MipsOpcode::SD, 1, 2, 0),
		]));

		assert_eq!(test_ee.read_memory(KSEG1_START, 8).map(|d| LittleEndian::read_u64(d)), Some(stored_data));
	}

	#[test]
	fn sd_plus_offset() {
		let stored_data: u64 = 0x1234_5678_9abc_def0;
		let base_pointer = KSEG1_START;

		let mut test_ee = EECore::default();

		test_ee.write_register(1, base_pointer.z_ext());
		test_ee.write_register(2, stored_data);

		install_and_run_program(&mut test_ee, instructions_to_bytes(&vec![
			ops::build_op_immediate(MipsOpcode::SD, 1, 2, 256),
		]));

		assert_eq!(test_ee.read_memory(base_pointer + 256, 8).map(|d| LittleEndian::read_u64(d)), Some(stored_data));
	}

	#[test]
	fn sd_minus_offset() {
		let stored_data: u64 = 0x1234_5678_9abc_def0;
		let base_pointer = KSEG1_START + 128;

		let mut test_ee = EECore::default();

		test_ee.write_register(1, base_pointer.z_ext());
		test_ee.write_register(2, stored_data);

		let offset: i16 = -24;

		install_and_run_program(&mut test_ee, instructions_to_bytes(&vec![
			ops::build_op_immediate(MipsOpcode::SD, 1, 2, offset as u16),
		]));

		assert_eq!(test_ee.read_memory(base_pointer - 24, 8).map(|d| LittleEndian::read_u64(d)), Some(stored_data));
	}

	#[test]
	fn sd_8_byte_aligned() {
		let stored_data: u64 = 0x1234_5678_9abc_def0;
		let base_pointer = KSEG1_START + 124;

		let mut test_ee = EECore::default();

		test_ee.write_register(1, base_pointer.z_ext());
		test_ee.write_register(2, stored_data);

		install_and_run_program(&mut test_ee, instructions_to_bytes(&vec![
			ops::build_op_immediate(MipsOpcode::SD, 1, 2, 0),
		]));

		assert!(test_ee.in_exception());
		assert_eq!(test_ee.read_memory(base_pointer, 8).map(|d| LittleEndian::read_u64(d)), Some(0));
	}
}