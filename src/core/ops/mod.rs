mod arithmetic;

use crate::core::{
	pipeline::*,
	EECore,
};

rs2_macro::ops!([
	[(ADD, arithmetic::add, 0b100000, 1)],
	[],
	[],
]);

pub fn nop(_cpu: &mut EECore, _data: &OpData) {
	// No Op.
}