use crate::{
	core::{
		exceptions::L1Exception,
		pipeline::*,
		EECore,
	},
	isa::mips::Instruction,
	utils::*,
};

pub fn add(cpu: &mut EECore, data: &OpCode) {
	// NOTE: do this work in signed space of proper size,
	// then convert to unsigned at end.
	// This achieves sign extension as required.
	let lhs = cpu.read_register(data.ri_get_source()) as i32;
	let rhs = cpu.read_register(data.ri_get_target()) as i32;

	if let Some(sum) = lhs.checked_add(rhs) {
		cpu.write_register(
			data.r_get_destination(),
			sum.s_ext(),
		);	
	} else {
		cpu.throw_l1_exception(L1Exception::Overflow);
	}
}

pub fn addi(cpu: &mut EECore, data: &OpCode) {
	let lhs = cpu.read_register(data.ri_get_source()) as i32;
	let rhs = i32::from(data.i_get_immediate_signed());

	if let Some(sum) = lhs.checked_add(rhs) {
		cpu.write_register(
			data.ri_get_target(),
			sum as u64,
		);	
	} else {
		cpu.throw_l1_exception(L1Exception::Overflow);
	}
}

pub fn addiu(cpu: &mut EECore, data: &OpCode) {
	let lhs = cpu.read_register(data.ri_get_source()) as u32;
	let rhs = data.i_get_immediate_signed().s_ext();

	cpu.write_register(
		data.ri_get_target(),
		lhs.wrapping_add(rhs).s_ext(),
	);
}

pub fn addu(cpu: &mut EECore, data: &OpCode) {
	let lhs = cpu.read_register(data.ri_get_source()) as u32;
	let rhs = cpu.read_register(data.ri_get_target()) as u32;

	cpu.write_register(
		data.r_get_destination(),
		lhs.wrapping_add(rhs).s_ext(),
	);
}

pub fn and(cpu: &mut EECore, data: &OpCode) {
	cpu.write_register(
		data.r_get_destination(),
		cpu.read_register(data.ri_get_source()) & cpu.read_register(data.ri_get_target()),
	);
}

pub fn andi(cpu: &mut EECore, data: &OpCode) {
	// rt <- rs & zero-ext(imm)
	let extd_imm = data.i_get_immediate() as u64;
	cpu.write_register(
		data.ri_get_target(),
		cpu.read_register(data.ri_get_source()) & extd_imm,
	);
}

pub fn daddu(cpu: &mut EECore, data: &OpCode) {
	// rs + rt -> rd
	let lhs = cpu.read_register(data.ri_get_source());
	let rhs = cpu.read_register(data.ri_get_target());

	cpu.write_register(
		data.r_get_destination(),
		lhs.wrapping_add(rhs),
	);
}

pub fn div(cpu: &mut EECore, data: &OpCode) {
	let lhs = cpu.read_register(data.ri_get_source()) as i32;
	let rhs = cpu.read_register(data.ri_get_target()) as i32;

	if rhs == 0 {
		return;
	}

	// may need some very... specific mods.

	let quotient = lhs.wrapping_div(rhs).s_ext();
	let remainder = lhs.wrapping_rem(rhs).s_ext();

	cpu.write_hi(remainder);
	cpu.write_lo(quotient);
}

pub fn divu(cpu: &mut EECore, data: &OpCode) {
	let lhs = cpu.read_register(data.ri_get_source()) as u32;
	let rhs = cpu.read_register(data.ri_get_target()) as u32;

	if rhs == 0 {
		return;
	}

	let quotient = (lhs / rhs).s_ext();
	let remainder = (lhs % rhs).s_ext();

	cpu.write_hi(remainder);
	cpu.write_lo(quotient);
}

pub fn movn(cpu: &mut EECore, data: &OpCode) {
	// if rt !=0, then rd <- rs
	if cpu.read_register(data.ri_get_target()) != 0 {
		cpu.write_register(
			data.r_get_destination(),
			cpu.read_register(data.ri_get_source()),
		);
	}
}

pub fn mult(cpu: &mut EECore, data: &OpCode) {
	// multiply rs and rt in signed space.
	// result will be 64-bit. Place into hi and lo (sign-extended).
	let lhs = cpu.read_register(data.ri_get_source()) as i64;
	let rhs = cpu.read_register(data.ri_get_target()) as i64;
	let result = lhs.wrapping_mul(rhs);

	cpu.write_hi((result >> 32) as u64);
	let lo_part = (result as i32).s_ext();
	cpu.write_lo(lo_part);

	// EE-core specific modification (RRR).
	let dest = data.r_get_destination();
	if dest != 0 {
		cpu.write_register(dest, lo_part);
	}
}

pub fn or(cpu: &mut EECore, data: &OpCode) {
	cpu.write_register(
		data.r_get_destination(),
		cpu.read_register(data.ri_get_source()) | cpu.read_register(data.ri_get_target()),
	);
}

pub fn ori(cpu: &mut EECore, data: &OpCode) {
	// rt <- rs | zero-ext(imm)
	let extd_imm = data.i_get_immediate() as u64;
	cpu.write_register(
		data.ri_get_target(),
		cpu.read_register(data.ri_get_source()) | extd_imm,
	);
}

pub fn sll(cpu: &mut EECore, data: &OpCode) {
	cpu.write_register(
		data.r_get_destination(),
		((cpu.read_register(data.ri_get_target()) as u32) << data.r_get_shift_amount()).s_ext(),
	);
}

pub fn slt(cpu: &mut EECore, data: &OpCode) {
	let lhs = cpu.read_register(data.ri_get_source()) as i64;
	let rhs = cpu.read_register(data.ri_get_target()) as i64;
	cpu.write_register(
		data.r_get_destination(),
		if lhs < rhs { 1 } else { 0 },
	);
}

pub fn slti(cpu: &mut EECore, data: &OpCode) {
	let lhs = cpu.read_register(data.ri_get_source()) as i64;
	let rhs = i64::from(data.i_get_immediate());
	cpu.write_register(
		data.ri_get_target(),
		if lhs < rhs { 1 } else { 0 },
	);
}

pub fn sltiu(cpu: &mut EECore, data: &OpCode) {
	let lhs = cpu.read_register(data.ri_get_source());
	let rhs = data.i_get_immediate().s_ext();
	cpu.write_register(
		data.ri_get_target(),
		if lhs < rhs { 1 } else { 0 },
	);
}

pub fn sltu(cpu: &mut EECore, data: &OpCode) {
	let lhs = cpu.read_register(data.ri_get_source());
	let rhs = cpu.read_register(data.ri_get_target());
	cpu.write_register(
		data.r_get_destination(),
		if lhs < rhs { 1 } else { 0 },
	);
}

pub fn sra(cpu: &mut EECore, data: &OpCode) {
	cpu.write_register(
		data.r_get_destination(),
		(cpu.read_register(data.ri_get_target()) as i32 >> data.r_get_shift_amount()).s_ext(),
	);
}

pub fn srl(cpu: &mut EECore, data: &OpCode) {
	cpu.write_register(
		data.r_get_destination(),
		(cpu.read_register(data.ri_get_target()) as u32 >> data.r_get_shift_amount()).s_ext(),
	);
}

pub fn subu(cpu: &mut EECore, data: &OpCode) {
	let lhs = cpu.read_register(data.ri_get_source()) as u32;
	let rhs = cpu.read_register(data.ri_get_target()) as u32;
	cpu.write_register(
		data.r_get_destination(),
		(lhs - rhs).s_ext(),
	);	
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{
		core::ops,
		isa::mips::{
			self,
			ee::{CacheFunction, Cop0Function, Cop1Function},
			Function as MipsFunction,
			Instruction,
			Opcode as MipsOpcode,
			RegImmFunction,
		},
	};

	#[test]
	fn basic_add() {
		// Place a value into registers 1 and 2, store their sum in register 3.
		let in_1 = 36;
		let in_2 = 19;

		let mut test_ee = EECore::new();
		test_ee.write_register(1, in_1);
		test_ee.write_register(2, in_2);

		let instruction = mips::build_op_register(MipsFunction::Add, 1, 2, 3, 0);

		test_ee.execute(ops::process_instruction(instruction));

		assert_eq!(test_ee.read_register(3), in_1 + in_2);
	}

	#[test]
	fn add_uses_sign() {
		// Place a value into registers 1 and 2, store their sum in register 3.
		let in_1 = 36;
		let in_2 = -19;

		let mut test_ee = EECore::new();
		test_ee.write_register(1, in_1);
		test_ee.write_register(2, in_2 as u64);

		let instruction = mips::build_op_register(MipsFunction::Add, 1, 2, 3, 0);

		test_ee.execute(ops::process_instruction(instruction));

		assert_eq!(test_ee.read_register(3) as i64, (in_1 as i64) + in_2);
	}

	#[test]
	fn add_overflow_exception() {
		// 32-bit signed overflow should trap.
		// Destination register should be unaffected.
		let in_1 = std::i32::MAX;
		let in_2 = 1;

		let mut test_ee = EECore::new();
		test_ee.write_register(1, in_1 as u64);
		test_ee.write_register(2, in_2);

		let instruction = mips::build_op_register(MipsFunction::Add, 1, 2, 3, 0);

		test_ee.execute(ops::process_instruction(instruction));

		assert!(test_ee.in_exception());
		assert_eq!(test_ee.read_register(3) as i64, 0);
	}

	#[test]
	fn basic_addi() {
		// Place a value into register 1, store their sum in register 2.
		let in_1 = 36;
		let in_2 = 19;

		let mut test_ee = EECore::new();
		test_ee.write_register(1, in_1);

		let instruction = mips::build_op_immediate(MipsOpcode::AddI, 1, 2, in_2);

		test_ee.execute(ops::process_instruction(instruction));

		assert_eq!(test_ee.read_register(2), in_1 + u64::from(in_2));
	}

	#[test]
	fn addi_uses_sign() {
		// Place a value into register 1, store their sum in register 2.
		let in_1 = 36;
		let in_2 = -19;

		let mut test_ee = EECore::new();
		test_ee.write_register(1, in_1);

		let instruction = mips::build_op_immediate(MipsOpcode::AddI, 1, 2, in_2 as u16);

		test_ee.execute(ops::process_instruction(instruction));

		assert_eq!(test_ee.read_register(2) as i32, (in_1 as i32) + (in_2 as i32));
	}

	#[test]
	fn addi_overflow_exception() {
		// 32-bit signed overflow should trap.
		// Destination register should be unaffected.
		let in_1 = std::i32::MAX;
		let in_2 = 1;

		let mut test_ee = EECore::new();
		test_ee.write_register(1, in_1 as u64);

		let instruction = mips::build_op_immediate(MipsOpcode::AddI, 1, 2, in_2 as u16);

		test_ee.execute(ops::process_instruction(instruction));

		assert!(test_ee.in_exception());
		assert_eq!(test_ee.read_register(3) as i64, 0);
	}

	#[test]
	fn basic_addu() {
		// Place a value into registers 1 and 2, store their sum in register 3.
		let in_1 = 23467;
		let in_2 = 34578;

		let mut test_ee = EECore::new();
		test_ee.write_register(1, in_1);
		test_ee.write_register(2, in_2);

		let instruction = mips::build_op_register(MipsFunction::AddU, 1, 2, 3, 0);

		test_ee.execute(ops::process_instruction(instruction));

		assert_eq!(test_ee.read_register(3), in_1 + in_2);
	}

	#[test]
	fn addu_no_overflow_exception() {
		// Signed overflow SHOULD be allowed.
		// The value is the result of performing the addition.
		let in_1 = std::i32::MAX;
		let in_2 = 1;

		let mut test_ee = EECore::new();
		test_ee.write_register(1, in_1 as u64);
		test_ee.write_register(2, in_2);

		let instruction = mips::build_op_register(MipsFunction::AddU, 1, 2, 3, 0);

		test_ee.execute(ops::process_instruction(instruction));

		assert!(!test_ee.in_exception());
		assert_eq!(test_ee.read_register(3) as i64, std::i32::MIN as i64);
	}

	#[test]
	fn basic_addiu() {
		// Place a value into register 1, store their sum in register 2.
		let in_1 = 23467;
		let in_2 = 34578;

		let mut test_ee = EECore::new();
		test_ee.write_register(1, in_1);

		let instruction = mips::build_op_immediate(MipsOpcode::AddIU, 1, 2, in_2);

		test_ee.execute(ops::process_instruction(instruction));

		assert_eq!(test_ee.read_register(2), ((in_1 as i32) + (in_2 as i16) as i32) as u64);
	}

	#[test]
	fn addiu_no_overflow_exception() {
		// Signed overflow SHOULD be allowed.
		let in_1 = std::i32::MAX;
		let in_2 = 1;

		let mut test_ee = EECore::new();
		test_ee.write_register(1, in_1 as u64);

		let instruction = mips::build_op_immediate(MipsOpcode::AddIU, 1, 2, in_2);

		test_ee.execute(ops::process_instruction(instruction));

		assert!(!test_ee.in_exception());
		assert_eq!(test_ee.read_register(2) as i64, std::i32::MIN as i64);
	}

	#[test]
	fn addiu_form_address() {
		// Lui + ORI + ADDIU ahould define an address together.
		let base_address_upper = 0x7000;
		let base_address_lower = 0x3ff0;
		let address_offset     = 0xffd0;

		let mut test_ee = EECore::new();

		install_and_run_program(&mut test_ee, instructions_to_bytes(&vec![
			mips::build_op_immediate(MipsOpcode::LUI, 0, 1, base_address_upper),
			mips::build_op_immediate(MipsOpcode::OrI, 1, 1, base_address_lower),
			mips::build_op_immediate(MipsOpcode::AddIU, 1, 1, address_offset),
		]));

		assert_eq!(test_ee.read_register(1), 0x7000_3fc0);
	}

	#[test]
	fn basic_and() {
		// Need to ensure this works on full 64-bit width.
		let in_1 = 0b1000_0000_0000_0000_0000_0000_0100_1111_0010_0000_0000_0010_0000_0000_1111_0001;
		let in_2 = 0b1000_1111_0000_0000_0100_0000_0100_1111_0010_0000_0000_0000_0000_0000_0000_0001;

		let mut test_ee = EECore::new();
		test_ee.write_register(1, in_1);
		test_ee.write_register(2, in_2);

		let instruction = mips::build_op_register(MipsFunction::And, 1, 2, 3, 0);

		test_ee.execute(ops::process_instruction(instruction));

		assert_eq!(test_ee.read_register(3), in_1 & in_2);
	}

	#[test]
	fn basic_andi() {
		// Need to ensure this works on full 64-bit width.
		let in_1 = 0b1000_0000_0000_0000_0000_0000_0100_1111_0010_0000_0000_0010_0000_0000_1111_0001;
		let in_2 = 0b1111_0000_0110_1000;

		let extended_in_2: u64 = in_2.z_ext();

		let mut test_ee = EECore::new();
		test_ee.write_register(1, in_1);

		let instruction = mips::build_op_immediate(MipsOpcode::AndI, 1, 2, in_2);

		test_ee.execute(ops::process_instruction(instruction));

		assert_eq!(test_ee.read_register(2), in_1 & extended_in_2);
	}

	#[test]
	fn basic_daddu() {
		let in_1 = std::u32::MAX as u64;
		let in_2 = 34578;

		let mut test_ee = EECore::new();
		test_ee.write_register(1, in_1);
		test_ee.write_register(2, in_2);

		let instruction = mips::build_op_register(MipsFunction::DAddU, 1, 2, 3, 0);

		test_ee.execute(ops::process_instruction(instruction));

		assert_eq!(test_ee.read_register(3), in_1 + in_2);
	}

	#[test]
	fn daddu_no_overflow_exception() {
		// Signed overflow SHOULD be allowed.
		let in_1 = (std::i64::MAX) as u64;
		let in_2 = 1;

		let mut test_ee = EECore::new();
		test_ee.write_register(1, in_1);
		test_ee.write_register(2, in_2);

		let instruction = mips::build_op_register(MipsFunction::DAddU, 1, 2, 3, 0);

		test_ee.execute(ops::process_instruction(instruction));

		assert!(!test_ee.in_exception());
		assert_eq!(test_ee.read_register(3), in_1.wrapping_add(in_2));
	}

	#[test]
	fn basic_div() {
		let in_1: i32 = -200;
		let in_2: i32 = -6;

		let mut test_ee = EECore::new();
		test_ee.write_register(1, in_1.s_ext());
		test_ee.write_register(2, in_2.s_ext());

		let instruction = mips::build_op_register(MipsFunction::Div, 1, 2, 0, 0);

		test_ee.execute(ops::process_instruction(instruction));

		assert_eq!(test_ee.read_lo() as i32, in_1 / in_2);
		assert_eq!(test_ee.read_hi() as i32, in_1 % in_2);
	}

	#[test]
	fn div_sign_table() {
		// DIV must follow the sign table described in pp.51
		let pos: i32 = 10;
		let pos_2: i32 = 7;
		let neg: i32 = -11;
		let neg_2: i32 = -3;

		let mut test_ee = EECore::new();

		// Mult has no dest register.
		let instruction = mips::build_op_register(MipsFunction::Div, 1, 2, 0, 0);

		// POS / POS
		test_ee.write_register(1, pos.s_ext());
		test_ee.write_register(2, pos_2.s_ext());
		test_ee.execute(ops::process_instruction(instruction));
		println!("{:?} {:?}", test_ee.read_lo() as i32, test_ee.read_hi() as i32);
		assert!((test_ee.read_lo() as i32) > 0);
		assert!((test_ee.read_hi() as i32) > 0);

		// POS / NEG
		test_ee.write_register(1, pos.s_ext());
		test_ee.write_register(2, neg_2.s_ext());
		test_ee.execute(ops::process_instruction(instruction));
		println!("{:?} {:?}", test_ee.read_lo() as i32, test_ee.read_hi() as i32);
		assert!((test_ee.read_lo() as i32) < 0);
		assert!((test_ee.read_hi() as i32) > 0);

		// NEG / POS
		test_ee.write_register(1, neg.s_ext());
		test_ee.write_register(2, pos_2.s_ext());
		test_ee.execute(ops::process_instruction(instruction));
		println!("{:?} {:?}", test_ee.read_lo() as i32, test_ee.read_hi() as i32);
		assert!((test_ee.read_lo() as i32) < 0);
		assert!((test_ee.read_hi() as i32) < 0);

		// NEG / NEG
		test_ee.write_register(1, neg.s_ext());
		test_ee.write_register(2, neg_2.s_ext());
		test_ee.execute(ops::process_instruction(instruction));
		println!("{:?} {:?}", test_ee.read_lo() as i32, test_ee.read_hi() as i32);
		assert!((test_ee.read_lo() as i32) > 0);
		assert!((test_ee.read_hi() as i32) < 0);
	}

	#[test]
	fn div_min_over_minus_one() {
		// DIV must not overflow and return a specific result for i32::MIN / -1
		let in_1: i32 = std::i32::MIN;
		let in_2: i32 = -1;

		let mut test_ee = EECore::new();
		test_ee.write_register(1, in_1.s_ext());
		test_ee.write_register(2, in_2.s_ext());

		let instruction = mips::build_op_register(MipsFunction::Div, 1, 2, 0, 0);

		test_ee.execute(ops::process_instruction(instruction));

		assert_eq!(test_ee.read_lo() as i32, std::i32::MIN);
		assert_eq!(test_ee.read_hi() as i32, 0);
	}

	#[test]
	fn div_by_zero_valid() {
		// DIV must not overflow and return a specific result for i32::MIN / -1
		let in_1: i32 = 646;
		let in_2: i32 = 0;

		let mut test_ee = EECore::new();
		test_ee.write_register(1, in_1.s_ext());
		test_ee.write_register(2, in_2.s_ext());

		let instruction = mips::build_op_register(MipsFunction::Div, 1, 2, 0, 0);

		test_ee.execute(ops::process_instruction(instruction));

		assert!(!test_ee.in_exception());
	}

	#[test]
	fn basic_divu() {
		let in_1 = std::u32::MAX.z_ext();
		let in_2 = 5;

		let mut test_ee = EECore::new();
		test_ee.write_register(1, in_1);
		test_ee.write_register(2, in_2);

		let instruction = mips::build_op_register(MipsFunction::DivU, 1, 2, 0, 0);

		test_ee.execute(ops::process_instruction(instruction));

		assert_eq!(test_ee.read_lo(), in_1 / in_2);
		assert_eq!(test_ee.read_hi(), in_1 % in_2);
	}

	#[test]
	fn divu_by_zero_valid() {
		// DIV must not overflow and return a specific result for i32::MIN / -1
		let in_1: i32 = 646;
		let in_2: i32 = 0;

		let mut test_ee = EECore::new();
		test_ee.write_register(1, in_1.s_ext());
		test_ee.write_register(2, in_2.s_ext());

		let instruction = mips::build_op_register(MipsFunction::DivU, 1, 2, 0, 0);

		test_ee.execute(ops::process_instruction(instruction));

		assert!(!test_ee.in_exception());
	}

	#[test]
	fn basic_movn() {
		let base_address_upper = 0x1234;
		let base_address_lower = 0x5678;

		let src = 1;
		let dest = 2;
		let cond_reg = 3;

		// positive case
		let mut test_ee = EECore::new();

		install_and_run_program(&mut test_ee, instructions_to_bytes(&vec![
			mips::build_op_immediate(MipsOpcode::LUI, 0, src, base_address_upper),
			mips::build_op_immediate(MipsOpcode::OrI, src, src, base_address_lower),

			mips::build_op_immediate(MipsOpcode::LUI, 0, cond_reg, 1),

			mips::build_op_register(MipsFunction::MovN, src, cond_reg, dest, 0),
		]));

		assert_eq!(test_ee.read_register(src), test_ee.read_register(dest));

		// negative case
		let mut test_ee = EECore::new();

		install_and_run_program(&mut test_ee, instructions_to_bytes(&vec![
			mips::build_op_immediate(MipsOpcode::LUI, 0, src, base_address_upper),
			mips::build_op_immediate(MipsOpcode::OrI, src, src, base_address_lower),

			mips::build_op_immediate(MipsOpcode::LUI, 0, cond_reg, 0),

			mips::build_op_register(MipsFunction::MovN, src, cond_reg, dest, 0),
		]));

		assert_ne!(test_ee.read_register(src), test_ee.read_register(dest));
	}

	#[test]
	fn basic_mult() {
		let in_1 = std::u32::MAX.s_ext();
		let in_2 = 2;

		let mut test_ee = EECore::new();
		test_ee.write_register(1, in_1);
		test_ee.write_register(2, in_2);

		// Mult has no dest register.
		let instruction = mips::build_op_register(MipsFunction::Mult, 1, 2, 0, 0);

		test_ee.execute(ops::process_instruction(instruction));

		let mult_result = in_1.wrapping_mul(in_2);

		assert_eq!(test_ee.read_hi(), ((mult_result >> 32) as u32).s_ext());
		assert_eq!(test_ee.read_lo(), (mult_result as u32).s_ext());
	}

	#[test]
	fn mult_lo_in_rd() {
		let in_1 = std::u32::MAX.s_ext();
		let in_2 = 2;

		let mut test_ee = EECore::new();
		test_ee.write_register(1, in_1);
		test_ee.write_register(2, in_2);

		// EE's mult has a dest register...
		let instruction = mips::build_op_register(MipsFunction::Mult, 1, 2, 3, 0);

		test_ee.execute(ops::process_instruction(instruction));

		let mult_result = in_1.wrapping_mul(in_2);

		assert_eq!(test_ee.read_register(3), (mult_result as u32).s_ext());
	}

	#[test]
	fn basic_or() {
		// Need to ensure this works on full 64-bit width.
		let in_1 = 0b1000_0000_0000_0000_0000_0000_0100_1111_0010_0000_0000_0010_0000_0000_1111_0001;
		let in_2 = 0b1000_1111_0000_0000_0100_0000_0100_1111_0010_0000_0000_0000_0000_0000_0000_0001;

		let mut test_ee = EECore::new();
		test_ee.write_register(1, in_1);
		test_ee.write_register(2, in_2);

		let instruction = mips::build_op_register(MipsFunction::Or, 1, 2, 3, 0);

		test_ee.execute(ops::process_instruction(instruction));

		assert_eq!(test_ee.read_register(3), in_1 | in_2);
	}

	#[test]
	fn basic_ori() {
		// Need to ensure this works on full 64-bit width.
		// NOTE: ORI zero-extends.
		let in_1 = 0b1000_0000_0000_0000_0000_0000_0100_1111_0010_0000_0000_0010_0000_0000_1111_0001;
		let in_2 = 0b1111_0000_0110_1000;

		let extended_in_2: u64 = in_2.z_ext();

		let mut test_ee = EECore::new();
		test_ee.write_register(1, in_1);

		let instruction = mips::build_op_immediate(MipsOpcode::OrI, 1, 2, in_2);

		test_ee.execute(ops::process_instruction(instruction));

		assert_eq!(test_ee.read_register(2), in_1 | extended_in_2);
	}

	#[test]
	fn ori_not_sign_extended() {
		// Contrary to the official documentation...
		// This can lead to some particularly bad address loads.
		let base_address_upper = 0x7000;
		let base_address_lower = 0xfff0;

		let mut test_ee = EECore::new();

		install_and_run_program(&mut test_ee, instructions_to_bytes(&vec![
			mips::build_op_immediate(MipsOpcode::LUI, 0, 1, base_address_upper),
			mips::build_op_immediate(MipsOpcode::OrI, 1, 1, base_address_lower),
		]));

		assert_eq!(test_ee.read_register(1) as i64, 0x7000_fff0);
	}

	#[test]
	fn basic_sll() {
		let input: u32 = 0b11 << 30;
		let shift_amount = 1;
		let mut test_ee = EECore::new();

		test_ee.write_register(1, input.s_ext());

		let instruction = mips::build_op_register(MipsFunction::SLL, 0, 1, 2, shift_amount);
		test_ee.execute(ops::process_instruction(instruction));

		assert_eq!(test_ee.read_register(2), 0xffff_ffff_8000_0000);
	}

	#[test]
	fn basic_slt() {
		let in_1 = -1;
		let in_2 = 256;
		let in_3 = 512;

		let lhs_r = 1;
		let rhs_r = 2;
		let dest = 3;

		let mut test_ee = EECore::new();
		test_ee.write_register(lhs_r, in_1.s_ext());
		test_ee.write_register(rhs_r, in_2.s_ext());

		// Test positive case.
		let instruction = mips::build_op_register(MipsFunction::SLT, lhs_r, rhs_r, dest, 0);

		test_ee.execute(ops::process_instruction(instruction));

		assert_eq!(test_ee.read_register(dest), 1);

		// Test negative case.
		test_ee.write_register(lhs_r, in_3.s_ext());

		test_ee.execute(ops::process_instruction(instruction));

		assert_eq!(test_ee.read_register(dest), 0);
	}

	#[test]
	fn basic_sltu() {
		let in_1 = u64::MAX - 2;
		let in_2 = u64::MAX - 1;
		let in_3 = u64::MAX;

		let lhs_r = 1;
		let rhs_r = 2;
		let dest = 3;

		let mut test_ee = EECore::new();
		test_ee.write_register(lhs_r, in_1);
		test_ee.write_register(rhs_r, in_2);

		// Test positive case.
		let instruction = mips::build_op_register(MipsFunction::SLTU, lhs_r, rhs_r, dest, 0);

		test_ee.execute(ops::process_instruction(instruction));

		assert_eq!(test_ee.read_register(dest), 1);

		// Test negative case.
		test_ee.write_register(lhs_r, in_3);

		test_ee.execute(ops::process_instruction(instruction));

		assert_eq!(test_ee.read_register(dest), 0);
	}

	#[test]
	fn basic_slti() {
		let in_1 = 23467;
		let in_2: u16 = 32760;

		let mut test_ee = EECore::new();
		test_ee.write_register(1, in_1);

		// Test positive case.
		let instruction = mips::build_op_immediate(MipsOpcode::SLTI, 1, 2, in_2);

		test_ee.execute(ops::process_instruction(instruction));

		assert_eq!(test_ee.read_register(2), 1);

		// Test negative case.
		test_ee.write_register(1, in_2.s_ext());

		test_ee.execute(ops::process_instruction(instruction));

		assert_eq!(test_ee.read_register(2), 0);
	}

	#[test]
	fn slti_sign_used() {
		let in_1 = -1;
		let in_2 = 0;

		let mut test_ee = EECore::new();
		test_ee.write_register(1, in_1 as u64);

		let instruction = mips::build_op_immediate(MipsOpcode::SLTI, 1, 2, in_2);

		test_ee.execute(ops::process_instruction(instruction));

		assert_eq!(test_ee.read_register(2), 1);
	}

	#[test]
	fn basic_sltiu() {
		let in_1 = 23467;
		let in_2 = 34578;

		let mut test_ee = EECore::new();
		test_ee.write_register(1, in_1);

		// Test positive case.
		let instruction = mips::build_op_immediate(MipsOpcode::SLTIU, 1, 2, in_2);

		test_ee.execute(ops::process_instruction(instruction));

		assert_eq!(test_ee.read_register(2), 1);

		// Test negative case.
		test_ee.write_register(1, in_2.s_ext());

		test_ee.execute(ops::process_instruction(instruction));

		assert_eq!(test_ee.read_register(2), 0);
	}

	#[test]
	fn sltiu_sign_unused() {
		// Place a value into register 1, store their sum in register 2.
		let in_1 = -1;
		let in_2 = 0;

		let mut test_ee = EECore::new();
		test_ee.write_register(1, in_1.s_ext());

		let instruction = mips::build_op_immediate(MipsOpcode::SLTIU, 1, 2, in_2);

		test_ee.execute(ops::process_instruction(instruction));

		assert_eq!(test_ee.read_register(2), 0);
	}

	#[test]
	fn basic_sra() {
		let input: u32 = 0b1 << 31;
		let shift_amount = 1;
		let mut test_ee = EECore::new();

		test_ee.write_register(1, input.s_ext());

		let instruction = mips::build_op_register(MipsFunction::SRA, 0, 1, 2, shift_amount);
		test_ee.execute(ops::process_instruction(instruction));

		assert_eq!(test_ee.read_register(2), 0xffff_ffff_c000_0000);
	}

	#[test]
	fn basic_srl() {
		let input: u32 = 0b1 << 31;
		let shift_amount = 1;
		let mut test_ee = EECore::new();

		test_ee.write_register(1, input.s_ext());

		let instruction = mips::build_op_register(MipsFunction::SRL, 0, 1, 2, shift_amount);
		test_ee.execute(ops::process_instruction(instruction));

		assert_eq!(test_ee.read_register(2), 0x0000_0000_4000_0000);
	}

	#[test]
	fn basic_subu() {
		let i1 = (u32::MAX as u64) + 100;
		let i2 = (u32::MAX as u64) + 57;

		let mut test_ee = EECore::new();

		test_ee.write_register(1, i1);
		test_ee.write_register(2, i2);

		let instruction = mips::build_op_register(MipsFunction::SubU, 1, 2, 3, 0);
		test_ee.execute(ops::process_instruction(instruction));

		assert_eq!(test_ee.read_register(3), i1 - i2);
	}
}