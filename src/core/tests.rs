use crate::{
	core::ops::{
		self,
		constants::{MipsOpcode, MipsFunction},
		NOP,
	},
	utils::*,
};
use super::{
	*,
	constants::*,
};
use std::convert::TryInto;

#[test]
fn simple_register_read_and_write() {
	let mut test_ee = EECore::new();
	let mut in_vals = [0u64; REGISTER_COUNT];

	// Place a different value in each register.
	// NOTE: R0 is special, don't touch.
	for i in 1..REGISTER_COUNT {
		let reg = i.try_into().unwrap();
		in_vals[i] = i.try_into().unwrap();

		test_ee.write_register(reg, in_vals[i]);
	}

	// Read each register a considerable amount of time after it was last written to.
	// Ensure that outputs match the original inputs.
	for i in 1..REGISTER_COUNT {
		let reg = i.try_into().unwrap();
		let expected = in_vals[i];
		let observed = test_ee.read_register(reg);

		assert_eq!(expected, observed);
	}
}

#[test]
fn write_reg_0_nop() {
	let mut test_ee = EECore::new();
	
	// Pre-condition: R0 == 0
	assert_eq!(0, test_ee.read_register(0));
	// Write should fail.
	assert_eq!(None, test_ee.write_register(0, 123456));
	// Post-condition: R0 == 0
	assert_eq!(0, test_ee.read_register(0));
}

#[test]
fn physical_address_mapped_by_kseg_0_1() {
	let mut test_ee = EECore::default();

	for offset in (0..=0x1fff_ffff).step_by(32) {
		assert_eq!(test_ee.translate_virtual_address(KSEG0_START + offset, true), Some(MmuAddress::Address(offset)));
		assert_eq!(test_ee.translate_virtual_address(KSEG1_START + offset, true), Some(MmuAddress::Address(offset)));
	}
}

#[test]
fn branch_delay_active_with_dual_issue_makes_two_reads() {
	// NOP <- fires     C1
	// J <- fires       C1
	// NOP <- fires     C2
	// LUI <- not fires
	// ...
	// LUI <- fires     C2 (jump target)
	// ORI not seen     C3
	let touched_register = 8;
	let untouched_register = 8;

	let offset_base = ((BIOS_START << 3) >> 5) as u32;

	let mut test_ee = EECore::new();
	test_ee.dual_issue = true;

	install_and_run_program_for(&mut test_ee, instructions_to_bytes(&vec![
		NOP,
		ops::build_op_jump(MipsOpcode::J, offset_base + 8),

		// + 2
		// Fires
		NOP,
		// Doesn't
		ops::build_op_immediate(MipsOpcode::LUI, 0, untouched_register, 1),

		// + 4
		NOP,
		NOP,

		// + 6
		NOP,
		NOP,

		// + 8
		ops::build_op_immediate(MipsOpcode::LUI, 0, touched_register, 0xabcd),
		// PC points here at end (before OrI).
		ops::build_op_immediate(MipsOpcode::OrI, touched_register, touched_register, 0xef12),
	]), 2);

	assert_eq!(format!("{:x}", test_ee.pc_register), format!("{:x}", BIOS_START + (9 << 2)));
	assert_eq!(test_ee.read_register(untouched_register), 0);
	assert_eq!(format!("{:016x}", test_ee.read_register(touched_register)), format!("{:016x}", 0xabcd_0000u32.s_ext()));
}
