use crate::core::{
	constants::*,
	pipeline::*,
	EECore,
};
use super::instruction::Instruction;

const PC_HO_BITS: u32 = 0b1111 << 28;
const PC_ALIGNED_BITS: u32 = 0b11;

pub fn bne(cpu: &mut EECore, data: &OpCode) {
	// Compute condition here.
	let cond = cpu.read_register(data.ri_get_source()) != cpu.read_register(data.ri_get_target());
	cpu.branch(data, inner_bne as BranchAction, cond as u32);
}

fn inner_bne(cpu: &mut EECore, data: &BranchOpCode) -> BranchResult {
	// Add immediate to current PC value.
	if data.temp != 0 {
		cpu.pc_register = cpu.pc_register.wrapping_add((data.i_get_immediate() as u32) << 2);
		BranchResult::BRANCHED
	} else {
		BranchResult::empty()
	}
}

pub fn j(cpu: &mut EECore, data: &OpCode) {
	cpu.branch(data, inner_j as BranchAction, 0);
}

fn inner_j(cpu: &mut EECore, data: &BranchOpCode) -> BranchResult {
	cpu.pc_register = (cpu.pc_register & PC_HO_BITS) | (data.j_get_jump() << 2);

	BranchResult::BRANCHED
}

pub fn jal(cpu: &mut EECore, data: &OpCode) {
	// Store PC after BD-slot in R31.
	cpu.write_register(31, (cpu.pc_register + (OPCODE_LENGTH_BYTES * 2) as u32) as u64);

	cpu.branch(data, inner_j as BranchAction, 0);
}

pub fn jalr(cpu: &mut EECore, data: &OpCode) {
	let dest = cpu.read_register(data.ri_get_source()) as u32;

	cpu.write_register(
		data.r_get_destination(),
		(cpu.pc_register + (OPCODE_LENGTH_BYTES * 2) as u32) as u64,
	);

	if dest & PC_ALIGNED_BITS == 0 {
		cpu.branch(data, inner_jr as BranchAction, dest);
	} else {
		// FIXME: fire Address Error exception.
		cpu.fire_exception();
	}
}

pub fn jr(cpu: &mut EECore, data: &OpCode) {
	let dest = cpu.read_register(data.ri_get_source()) as u32;

	if dest & PC_ALIGNED_BITS == 0 {
		cpu.branch(data, inner_jr as BranchAction, dest);
	} else {
		// FIXME: fire Address Error exception.
		cpu.fire_exception();
	}
}

fn inner_jr(cpu: &mut EECore, data: &BranchOpCode) -> BranchResult {
	cpu.pc_register = data.temp;

	BranchResult::BRANCHED
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
		let jump_target = (BIOS_START as u32) | (jump_offset << 2);

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
	fn basic_bne() {
		// Execute a jump instruction and a NOP. PC changes by relative amount.
		// PC only changes if target registers do not match.
		let jump_offset: u16 = 0x12_34;
		let jump_target = (BIOS_START as u32) + 4 + ((jump_offset as u32) << 2);

		let program = vec![
			ops::build_op_immediate(MipsOpcode::BNE, 1, 2, jump_offset),
			NOP,
		];
		let mut program_bytes = vec![0u8; 4 * program.len()];
		LittleEndian::write_u32_into(&program[..], &mut program_bytes[..]);

		let mut staying_ee = EECore::new();
		let mut jumping_ee = EECore::new();

		staying_ee.write_register(1, 1234);
		staying_ee.write_register(2, 1234);
		staying_ee.cycle(&program_bytes[..]);

		jumping_ee.write_register(1, 1234);
		jumping_ee.write_register(2, 1235);
		jumping_ee.cycle(&program_bytes[..]);

		assert_eq!(staying_ee.pc_register, (BIOS_START as u32) + 8);
		assert_eq!(jumping_ee.pc_register, jump_target);
	}

	#[test]
	fn basic_jump() {
		// Execute a jump instruction and a NOP. PC changes to new target.
		// NOTE: EE starts in uncached BIOS region (KSEG1).
		let jump_offset: u32 = 0x12_34_56;
		let jump_target = (BIOS_START as u32) & PC_HO_BITS | (jump_offset << 2);

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
	fn basic_jr() {
		// Execute a jump instruction and a NOP. PC changes to new target.
		let jump_dest: u32 = 0x1234_5678;

		let program = vec![
			ops::build_op_register(MipsFunction::JR, 1, 0, 0, 0),
			NOP,
		];
		let mut program_bytes = vec![0u8; 4 * program.len()];
		LittleEndian::write_u32_into(&program[..], &mut program_bytes[..]);

		let mut test_ee = EECore::new();

		test_ee.write_register(1, jump_dest as u64);

		test_ee.cycle(&program_bytes[..]);

		assert_eq!(test_ee.pc_register, jump_dest);
	}

	#[test]
	fn basic_jalr() {
		// Execute a jump instruction and a NOP. PC changes to new target.
		// Old PC appears in arbitrary register.
		let jump_dest: u32 = 0x1234_5678;

		let program = vec![
			ops::build_op_register(MipsFunction::JaLR, 1, 0, 5, 0),
			NOP,
		];
		let mut program_bytes = vec![0u8; 4 * program.len()];
		LittleEndian::write_u32_into(&program[..], &mut program_bytes[..]);

		let mut test_ee = EECore::new();

		test_ee.write_register(1, jump_dest as u64);

		test_ee.cycle(&program_bytes[..]);

		assert_eq!(test_ee.pc_register, jump_dest);
		assert_eq!(test_ee.read_register(5) as u32, (BIOS_START as u32) + 8);
	}

	#[test]
	fn basic_jal() {
		// Execute a jump instruction and a NOP. PC changes to new target.
		// Old PC appears in register 31.
		let jump_offset: u32 = 0x12_34_56;
		let jump_target = (BIOS_START as u32) & PC_HO_BITS | (jump_offset << 2);

		let program = vec![
			ops::build_op_jump(MipsOpcode::JaL, jump_offset),
			NOP,
		];
		let mut program_bytes = vec![0u8; 4 * program.len()];
		LittleEndian::write_u32_into(&program[..], &mut program_bytes[..]);

		let mut test_ee = EECore::new();

		let old_pc = test_ee.pc_register;

		test_ee.cycle(&program_bytes[..]);

		assert_eq!(test_ee.pc_register, jump_target);
		assert_eq!(test_ee.read_register(31) as u32, (BIOS_START as u32) + 8);
	}

	#[test]
	fn jalr_unaligned_address_exception() {
		unimplemented!()
		// FIXME: left undone to remind myself that these exceptions should happen during address fetch...
		// Unclear how this affects PC.
	}

	#[test]
	fn jr_unaligned_address_exception() {
		// Execute a jump instruction and a NOP. PC changes to new target.
		let jump_dest: u32 = 0x1234_5679;

		let program = vec![
			ops::build_op_register(MipsFunction::JR, 1, 0, 0, 0),
			NOP,
		];
		let mut program_bytes = vec![0u8; 4 * program.len()];
		LittleEndian::write_u32_into(&program[..], &mut program_bytes[..]);

		let mut test_ee = EECore::new();

		test_ee.write_register(1, jump_dest as u64);

		test_ee.cycle(&program_bytes[..]);

		assert_ne!(test_ee.pc_register, jump_dest);
		assert!(test_ee.in_exception());
	}

	#[test]
	fn jump_delay_slot_fires() {
		// Execute a jump instruction and an ADD. ADD should have taken effect.
		let jump_offset: u32 = 0x12_34_56;
		let jump_target = (BIOS_START as u32) & PC_HO_BITS | (jump_offset << 2);

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