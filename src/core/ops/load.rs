use byteorder::{
	ByteOrder,
	LittleEndian,
};
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

pub fn lb(cpu: &mut EECore, data: &OpCode) {
	// FIXME: break this code out of several functions...
	let offset: u32 = data.i_get_immediate_signed().s_ext();
	let v_addr = (cpu.read_register(data.ri_get_source()) as u32).wrapping_add(offset);

	let loc = cpu.read_memory(v_addr as u32, size_of::<u8>())
		.map(|buf| buf[0]);

	if let Some(loc) = loc {
		cpu.write_register(data.ri_get_target(), loc.s_ext());
	}
}

pub fn lbu(cpu: &mut EECore, data: &OpCode) {
	// FIXME: break this code out of several functions...
	let offset: u32 = data.i_get_immediate_signed().s_ext();
	let v_addr = (cpu.read_register(data.ri_get_source()) as u32).wrapping_add(offset);

	let loc = cpu.read_memory(v_addr as u32, size_of::<u8>())
		.map(|buf| buf[0]);

	if let Some(loc) = loc {
		cpu.write_register(data.ri_get_target(), loc.z_ext());
	}
}

pub fn ld(cpu: &mut EECore, data: &OpCode) {
	// FIXME: break this code out of several functions...
	let offset: u32 = data.i_get_immediate_signed().s_ext();
	let v_addr = (cpu.read_register(data.ri_get_source()) as u32).wrapping_add(offset);

	// FIXME: make size info part of address resolution.
	if v_addr & 0b111 != 0 {
		cpu.throw_l1_exception(L1Exception::AddressErrorFetchLoad(v_addr));
		return;
	}

	let loc = cpu.read_memory(v_addr as u32, size_of::<u64>())
		.map(LittleEndian::read_u64);

	if let Some(loc) = loc {
		cpu.write_register(data.ri_get_target(), loc);
	}
}

pub fn lhu(cpu: &mut EECore, data: &OpCode) {
	// FIXME: break this code out of several functions...
	let offset: u32 = data.i_get_immediate_signed().s_ext();
	let v_addr = (cpu.read_register(data.ri_get_source()) as u32).wrapping_add(offset);

	// FIXME: make size info part of address resolution.
	if v_addr & 0b1 != 0 {
		cpu.throw_l1_exception(L1Exception::AddressErrorFetchLoad(v_addr));
		return;
	}

	let loc = cpu.read_memory(v_addr as u32, size_of::<u16>())
		.map(LittleEndian::read_u16);

	if let Some(loc) = loc {
		cpu.write_register(data.ri_get_target(), loc.z_ext());
	}
}

pub fn lw(cpu: &mut EECore, data: &OpCode) {
	// FIXME: break this code out of several functions...
	let offset: u32 = data.i_get_immediate_signed().s_ext();
	let v_addr = (cpu.read_register(data.ri_get_source()) as u32).wrapping_add(offset);

	trace!("{:08x}", v_addr);

	// FIXME: make size info part of address resolution.
	if v_addr & 0b11 != 0 {
		cpu.throw_l1_exception(L1Exception::AddressErrorFetchLoad(v_addr));
		return;
	}

	let loc = cpu.read_memory(v_addr as u32, size_of::<u32>())
		.map(LittleEndian::read_u32);

	if let Some(loc) = loc {
		cpu.write_register(data.ri_get_target(), loc.s_ext());
	}
}

pub fn lui(cpu: &mut EECore, data: &OpCode) {
	// load sign extended shifted value of immediate into rt.
	let v = i64::from(data.i_get_immediate());
	cpu.write_register(data.ri_get_target(), (v << 16) as u64);
}

pub fn mfhi(cpu: &mut EECore, data: &OpCode) {
	// HI -> GPR[rd]
	cpu.write_register(data.r_get_destination(), cpu.read_hi());
}

pub fn mflo(cpu: &mut EECore, data: &OpCode) {
	// LO -> GPR[rd]
	cpu.write_register(data.r_get_destination(), cpu.read_lo());
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
	fn basic_lb() {
		let offset: i16 = 0;
		let read_val = 0xfa;

		let mut test_ee = EECore::new();

		test_ee.write_register(1, KSEG1_START.z_ext());
		test_ee.write_memory(KSEG1_START, &[read_val]);
		let instruction = ops::build_op_immediate(MipsOpcode::LB, 1, 2, offset as u16);

		test_ee.execute(ops::process_instruction(instruction));

		assert_eq!(test_ee.read_register(2), read_val.s_ext());
	}

	#[test]
	fn basic_lbu() {
		let offset: i16 = 0;
		let read_val = 0xfa;

		let mut test_ee = EECore::new();

		test_ee.write_register(1, KSEG1_START.z_ext());
		test_ee.write_memory(KSEG1_START, &[read_val]);
		let instruction = ops::build_op_immediate(MipsOpcode::LBU, 1, 2, offset as u16);

		test_ee.execute(ops::process_instruction(instruction));

		assert_eq!(test_ee.read_register(2), read_val.z_ext());
	}

	#[test]
	fn basic_ld() {
		let offset: i16 = 0;
		let read_val: u64 = 0x1234_5678_90ab_cdef;

		let mut test_ee = EECore::new();

		test_ee.write_register(1, KSEG1_START.z_ext());
		test_ee.write_memory(KSEG1_START, &read_val.to_le_bytes());
		let instruction = ops::build_op_immediate(MipsOpcode::LD, 1, 2, offset as u16);

		test_ee.execute(ops::process_instruction(instruction));

		assert_eq!(test_ee.read_register(2), read_val);
	}

	#[test]
	fn basic_lhu() {
		unimplemented!();
	}

	#[test]
	fn basic_lw() {
		let offset: i16 = 0;
		let read_val: u32 = 0x90ab_cdef;

		let mut test_ee = EECore::new();

		test_ee.write_register(1, KSEG1_START.z_ext());
		test_ee.write_memory(KSEG1_START, &read_val.to_le_bytes());
		let instruction = ops::build_op_immediate(MipsOpcode::LW, 1, 2, offset as u16);

		test_ee.execute(ops::process_instruction(instruction));

		assert_eq!(test_ee.read_register(2), read_val.s_ext());
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

	#[test]
	fn basic_mfhi() {
		let hi: u64 = 0x1234_5678_abcd_ef90;

		let mut test_ee = EECore::new();
		test_ee.write_hi(hi);

		let instruction = ops::build_op_register(MipsFunction::MFHi, 0, 0, 1, 0);

		test_ee.execute(ops::process_instruction(instruction));

		assert_eq!(test_ee.read_register(1), hi);
	}

	#[test]
	fn basic_mflo() {
		let lo: u64 = 0x1234_5678_abcd_ef90;

		let mut test_ee = EECore::new();
		test_ee.write_lo(lo);

		let instruction = ops::build_op_register(MipsFunction::MFLo, 0, 0, 1, 0);

		test_ee.execute(ops::process_instruction(instruction));

		assert_eq!(test_ee.read_register(1), lo);
	}
}