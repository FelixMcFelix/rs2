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