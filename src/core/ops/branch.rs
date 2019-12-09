use crate::core::{
	constants::*,
	pipeline::*,
	EECore,
};
use super::instruction::Instruction;

const PC_HO_BITS: u32 = 0b1111 << 28;
const PC_ALIGNED_BITS: u64 = 0b11;

pub fn j(cpu: &mut EECore, data: &OpCode) {
	let mut new_op = *data;
	new_op.action = &(inner_j as EEAction);

	let _ = cpu.branch_delay_slot.replace(new_op);
}

fn inner_j(cpu: &mut EECore, data: &OpCode) {
	cpu.pc_register = (cpu.pc_register & PC_HO_BITS) | (data.j_get_jump() << 2);
}

pub fn jal(cpu: &mut EECore, data: &OpCode) {
	// Store PC after BD-slot in R31.
	cpu.write_register(31, (cpu.pc_register + (OPCODE_LENGTH_BYTES * 2) as u32) as u64);

	let mut new_op = *data;
	new_op.action = &(inner_j as EEAction);

	let _ = cpu.branch_delay_slot.replace(new_op);
}

pub fn jalr(cpu: &mut EECore, data: &OpCode) {
	unimplemented!()
}

pub fn jr(cpu: &mut EECore, data: &OpCode) {
	let dest = cpu.read_register(data.ri_get_source());

	if dest & PC_ALIGNED_BITS == 0 {
		let mut new_op = *data;
		new_op.action = &(inner_jr as EEAction);

		let _ = cpu.branch_delay_slot.replace(new_op);
	} else {
		// FIXME: fire Address Error exception.
	}
}

fn inner_jr(cpu: &mut EECore, data: &OpCode) {
	cpu.pc_register = cpu.read_register(data.ri_get_source()) as u32;
}

#[cfg(test)]
mod tests {
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
	fn jump_not_instant() {
		// Execute a jump instruction with no followup. No change to PC.
		let jump_offset = 0x12_34_56;
		let jump_target = (KSEG1_START as u32) | (jump_offset << 2);

		let program = vec![
			NOP,
			ops::build_op_jump(MipsOpcode::J, jump_offset),
		];
		let mut program_bytes = vec![0u8; 4 * program.len()];
		LittleEndian::write_u32_into(&program[..], &mut program_bytes[..]);

		let mut test_ee = EECore::new();

		test_ee.cycle(&program_bytes[..]);

		assert_ne!(test_ee.pc_register, jump_target);
	}

	#[test]
	fn basic_jump() {
		// Execute a jump instruction and a NOP. PC changes to new target.
		// NOTE: EE starts in uncached BIOS region (KSEG1).
		let jump_offset: u32 = 0x12_34_56;
		let jump_target = (KSEG1_START as u32) | (jump_offset << 2);

		let program = vec![
			ops::build_op_jump(MipsOpcode::J, jump_offset),
			NOP,
		];
		let mut program_bytes = vec![0u8; 4 * program.len()];
		LittleEndian::write_u32_into(&program[..], &mut program_bytes[..]);

		let mut test_ee = EECore::new();

		test_ee.cycle(&program_bytes[..]);

		assert_eq!(test_ee.pc_register, jump_target);
	}

	#[test]
	fn jump_delay_slot_fires() {
		// Execute a jump instruction and an ADD. ADD should have taken effect.
		let jump_offset: u32 = 0x12_34_56;
		let jump_target = (KSEG1_START as u32) | (jump_offset << 2);

		let program = vec![
			ops::build_op_jump(MipsOpcode::J, jump_offset),
			ops::build_op_register(MipsFunction::Add, 1, 2, 3, 0),
		];
		let mut program_bytes = vec![0u8; 4 * program.len()];
		LittleEndian::write_u32_into(&program[..], &mut program_bytes[..]);

		let mut test_ee = EECore::new();

		let in_1 = 111;
		let in_2 = 256;
		test_ee.write_register(1, in_1);
		test_ee.write_register(2, in_2);

		test_ee.cycle(&program_bytes[..]);

		assert_eq!(test_ee.read_register(3), in_1 + in_2);
		assert_eq!(test_ee.pc_register, jump_target);
	}
}