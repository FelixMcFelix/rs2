use enum_primitive::*;

enum_from_primitive!{
#[derive(Debug, PartialEq)]
pub enum MipsOpcode {
	Special = 0b00_0000,
	AddI    = 0b00_1000,
	AddIU   = 0b00_1001,
	J       = 0b00_0010,
}
}

enum_from_primitive!{
#[derive(Debug, PartialEq)]
pub enum MipsFunction {
	Add  = 0b10_0000,
	AddU = 0b10_0001,
	And  = 0b10_0100,
}
}