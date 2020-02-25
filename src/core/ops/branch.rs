use crate::{
	core::{
		constants::*,
		exceptions::L1Exception,
		pipeline::*,
		EECore,
	},
	utils::*,
};
use super::instruction::Instruction;

const PC_HO_BITS: u32 = 0b1111 << 28;
const PC_ALIGNED_BITS: u32 = 0b11;

pub fn beq(cpu: &mut EECore, data: &OpCode) {
	// Compute condition here.
	let cond = cpu.read_register(data.ri_get_source()) == cpu.read_register(data.ri_get_target());
	cpu.branch(data, inner_bne as BranchAction, cond as u32);
}

pub fn beql(cpu: &mut EECore, data: &OpCode) {
	// Compute condition here.
	let cond = cpu.read_register(data.ri_get_source()) == cpu.read_register(data.ri_get_target());
	cpu.branch(data, inner_bnel as BranchAction, cond as u32);
}

pub fn bgez(cpu: &mut EECore, data: &OpCode) {
	// If GPR[rs]>=0, then apply offset to current PC as in BNE.
	let cond = cpu.read_register(data.ri_get_source()) as i64 >= 0;
	cpu.branch(data, inner_bne as BranchAction, cond as u32);
}

pub fn bltz(cpu: &mut EECore, data: &OpCode) {
	// If GPR[rs]<0, then apply offset to current PC as in BNE.
	let cond = (cpu.read_register(data.ri_get_source()) as i64) < 0;
	cpu.branch(data, inner_bne as BranchAction, cond as u32);
}

pub fn bne(cpu: &mut EECore, data: &OpCode) {
	// Compute condition here.
	let cond = cpu.read_register(data.ri_get_source()) != cpu.read_register(data.ri_get_target());
	cpu.branch(data, inner_bne as BranchAction, cond as u32);
}

fn inner_bne(cpu: &mut EECore, data: &BranchOpCode) -> BranchResult {
	// Add immediate to current PC value.
	if data.temp != 0 {
		let offset: u32 = data.i_get_immediate().s_ext();
		cpu.pc_register = cpu.pc_register.wrapping_add(offset << 2);
		BranchResult::BRANCHED
	} else {
		BranchResult::empty()
	}
}

pub fn bnel(cpu: &mut EECore, data: &OpCode) {
	// Compute condition here.
	let cond = cpu.read_register(data.ri_get_source()) != cpu.read_register(data.ri_get_target());
	cpu.branch(data, inner_bnel as BranchAction, cond as u32);
}

fn inner_bnel(cpu: &mut EECore, data: &BranchOpCode) -> BranchResult {
	// Add immediate to current PC value.
	if data.temp != 0 {
		let offset: u32 = data.i_get_immediate().s_ext();
		cpu.pc_register = cpu.pc_register.wrapping_add(offset << 2);
		BranchResult::BRANCHED
	} else {
		BranchResult::NULLIFIED
	}
}

pub fn break_i(cpu: &mut EECore, _data: &OpCode) {
	cpu.throw_l1_exception(L1Exception::Break);
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
	cpu.write_register(
		31,
		u64::from(cpu.pc_register.wrapping_add((OPCODE_LENGTH_BYTES * 2) as u32))
	);

	cpu.branch(data, inner_j as BranchAction, 0);
}

pub fn jalr(cpu: &mut EECore, data: &OpCode) {
	let dest = cpu.read_register(data.ri_get_source()) as u32;

	cpu.write_register(
		data.r_get_destination(),
		u64::from(cpu.pc_register.wrapping_add((OPCODE_LENGTH_BYTES * 2) as u32))
	);

	if dest & PC_ALIGNED_BITS == 0 {
		cpu.branch(data, inner_jr as BranchAction, dest);
	} else {
		cpu.throw_l1_exception(L1Exception::AddressErrorFetchLoad(dest));
	}
}

pub fn jr(cpu: &mut EECore, data: &OpCode) {
	let dest = cpu.read_register(data.ri_get_source()) as u32;

	if dest & PC_ALIGNED_BITS == 0 {
		cpu.branch(data, inner_jr as BranchAction, dest);
	} else {
		cpu.throw_l1_exception(L1Exception::AddressErrorFetchLoad(dest));
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

		let mut test_ee = EECore::new();

		install_and_run_program(&mut test_ee, instructions_to_bytes(&vec![
			NOP,
			ops::build_op_jump(MipsOpcode::J, jump_offset),
		]));

		assert_ne!(test_ee.pc_register, jump_target);
	}

	#[test]
	fn basic_beq() {
		// Execute a jump instruction and a NOP. PC changes by relative amount.
		// PC only changes if target registers do not match.
		let jump_offset: u16 = 0x00_f0;
		let jump_target = (BIOS_START as u32) + 4 + ((jump_offset as u32) << 2);

		let program = instructions_to_bytes(&vec![
			ops::build_op_immediate(MipsOpcode::BEq, 1, 2, jump_offset),
			NOP,
		]);
		
		let mut staying_ee = EECore::new();
		let mut jumping_ee = EECore::new();

		staying_ee.write_register(1, 1234);
		staying_ee.write_register(2, 1235);
		install_and_run_program(&mut staying_ee, program.clone());

		jumping_ee.write_register(1, 1234);
		jumping_ee.write_register(2, 1234);
		install_and_run_program(&mut jumping_ee, program);

		assert_eq!(staying_ee.pc_register, (BIOS_START as u32) + 8);
		assert_eq!(jumping_ee.pc_register, jump_target);
	}

	#[test]
	fn basic_bne() {
		// Execute a jump instruction and a NOP. PC changes by relative amount.
		// PC only changes if target registers do not match.
		let jump_offset: u16 = 0x00_f0;
		let jump_target = (BIOS_START as u32) + 4 + ((jump_offset as u32) << 2);

		let program = instructions_to_bytes(&vec![
			ops::build_op_immediate(MipsOpcode::BNE, 1, 2, jump_offset),
			NOP,
		]);
		
		let mut staying_ee = EECore::new();
		let mut jumping_ee = EECore::new();

		staying_ee.write_register(1, 1234);
		staying_ee.write_register(2, 1234);
		install_and_run_program(&mut staying_ee, program.clone());

		jumping_ee.write_register(1, 1234);
		jumping_ee.write_register(2, 1235);
		install_and_run_program(&mut jumping_ee, program);

		assert_eq!(staying_ee.pc_register, (BIOS_START as u32) + 8);
		assert_eq!(jumping_ee.pc_register, jump_target);
	}

	#[test]
	fn bne_negative_offset() {
		// Go backwards by 5 instructions from Jump's PC.
		let jump_offset: i16 = -5;
		let jump_target = (BIOS_START as u32) + 4 - 20;

		let program = instructions_to_bytes(&vec![
			ops::build_op_immediate(MipsOpcode::BNE, 1, 2, jump_offset as u16),
			NOP,
		]);

		let mut jumping_ee = EECore::new();

		jumping_ee.write_register(1, 1234);
		jumping_ee.write_register(2, 1235);
		install_and_run_program(&mut jumping_ee, program);

		assert_eq!(jumping_ee.pc_register, jump_target);
	}

	#[test]
	fn basic_bgez() {
		let jump_offset: u16 = 0x00_f0;
		let jump_target = (BIOS_START as u32) + 4 + ((jump_offset as u32) << 2);

		let program = instructions_to_bytes(&vec![
			ops::build_op_immediate(MipsOpcode::RegImm, 1, RegImmFunction::BGEZ as u8, jump_offset),
			NOP,
		]);
		
		let mut staying_ee = EECore::new();
		let mut jumping_ee = EECore::new();
		let mut jumping_z_ee = EECore::new();

		let staying_val = -123;

		staying_ee.write_register(1, staying_val.s_ext());
		install_and_run_program(&mut staying_ee, program.clone());

		jumping_ee.write_register(1, 1234);
		install_and_run_program(&mut jumping_ee, program.clone());

		jumping_z_ee.write_register(1, 0);
		install_and_run_program(&mut jumping_z_ee, program);

		assert_eq!(staying_ee.pc_register, (BIOS_START as u32) + 8);
		assert_eq!(jumping_ee.pc_register, jump_target);
		assert_eq!(jumping_z_ee.pc_register, jump_target);
	}

	#[test]
	fn basic_bltz() {
		let jump_offset: u16 = 0x00_f0;
		let jump_target = (BIOS_START as u32) + 4 + ((jump_offset as u32) << 2);

		let program = instructions_to_bytes(&vec![
			ops::build_op_immediate(MipsOpcode::RegImm, 1, RegImmFunction::BLTZ as u8, jump_offset),
			NOP,
		]);
		
		let mut jumping_ee = EECore::new();
		let mut stay_z_ee = EECore::new();
		let mut stay_gz_ee = EECore::new();

		let jumping_val = -123;

		jumping_ee.write_register(1, jumping_val.s_ext());
		install_and_run_program(&mut jumping_ee, program.clone());

		stay_z_ee.write_register(1, 0);
		install_and_run_program(&mut stay_z_ee, program.clone());

		stay_gz_ee.write_register(1, 1234);
		install_and_run_program(&mut stay_gz_ee, program);

		assert_eq!(jumping_ee.pc_register, jump_target);
		assert_eq!(stay_z_ee.pc_register, (BIOS_START as u32) + 8);
		assert_eq!(stay_gz_ee.pc_register, (BIOS_START as u32) + 8);
	}

	#[test]
	fn basic_break_i() {
		unimplemented!();
	}

	#[test]
	fn basic_beql() {
		unimplemented!();
	}

	#[test]
	fn basic_bnel() {
		unimplemented!();
	}

	#[test]
	fn beql_bnel_skip_delay_slot() {
		unimplemented!();
	}

	#[test]
	fn basic_jump() {
		// Execute a jump instruction and a NOP. PC changes to new target.
		// NOTE: EE starts in uncached BIOS region (KSEG1).
		let jump_offset: u32 = 0x12_34_56;
		let jump_target = (BIOS_START as u32) & PC_HO_BITS | (jump_offset << 2);

		let mut test_ee = EECore::new();
		install_and_run_program(&mut test_ee, instructions_to_bytes(&vec![
			ops::build_op_jump(MipsOpcode::J, jump_offset),
			NOP,
		]));

		assert_eq!(test_ee.pc_register, jump_target);
	}

	#[test]
	fn basic_jr() {
		// Execute a jump instruction and a NOP. PC changes to new target.
		let jump_dest: u32 = 0x1234_5678;

		let mut test_ee = EECore::new();
		test_ee.write_register(1, jump_dest as u64);
		install_and_run_program(&mut test_ee, instructions_to_bytes(&vec![
			ops::build_op_register(MipsFunction::JR, 1, 0, 0, 0),
			NOP,
		]));

		assert_eq!(test_ee.pc_register, jump_dest);
	}

	#[test]
	fn basic_jalr() {
		// Execute a jump instruction and a NOP. PC changes to new target.
		// Old PC appears in arbitrary register.
		let jump_dest: u32 = 0x1234_5678;

		let mut test_ee = EECore::new();
		test_ee.write_register(1, jump_dest as u64);
		install_and_run_program(&mut test_ee, instructions_to_bytes(&vec![
			ops::build_op_register(MipsFunction::JaLR, 1, 0, 5, 0),
			NOP,
		]));

		assert_eq!(test_ee.read_register(5) as u32, (BIOS_START as u32) + 8);
		assert_eq!(test_ee.pc_register, jump_dest);
	}

	#[test]
	fn basic_jal() {
		// Execute a jump instruction and a NOP. PC changes to new target.
		// Old PC appears in register 31.
		let jump_offset: u32 = 0x12_34_56;
		let jump_target = (BIOS_START as u32) & PC_HO_BITS | (jump_offset << 2);

		let mut test_ee = EECore::new();
		let old_pc = test_ee.pc_register;
		install_and_run_program(&mut test_ee, instructions_to_bytes(&vec![
			ops::build_op_jump(MipsOpcode::JaL, jump_offset),
			NOP,
		]));

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
		let jump_dest: u32 = (BIOS_START as u32) + 0x0000_1235;

		let mut test_ee = EECore::new();
		test_ee.write_register(1, jump_dest as u64);
		install_and_run_program(&mut test_ee, instructions_to_bytes(&vec![
			ops::build_op_register(MipsFunction::JR, 1, 0, 0, 0),
			NOP,
		]));

		assert_ne!(test_ee.pc_register, jump_dest);
		assert!(test_ee.in_exception());
	}

	#[test]
	fn jump_delay_slot_fires() {
		// Execute a jump instruction and an ADD. ADD should have taken effect.
		let jump_offset: u32 = 0x12_34_56;
		let jump_target = (BIOS_START as u32) & PC_HO_BITS | (jump_offset << 2);

		let mut test_ee = EECore::new();
		let in_1 = 111;
		let in_2 = 256;
		test_ee.write_register(1, in_1);
		test_ee.write_register(2, in_2);

		install_and_run_program(&mut test_ee, instructions_to_bytes(&vec![
			ops::build_op_jump(MipsOpcode::J, jump_offset),
			ops::build_op_register(MipsFunction::Add, 1, 2, 3, 0),
		]));

		assert_eq!(test_ee.read_register(3), in_1 + in_2);
		assert_eq!(test_ee.pc_register, jump_target);
	}
}