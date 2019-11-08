use crate::core::{
	pipeline::*,
	EECore,
};

pub fn add(cpu: &mut EECore, data: &OpData) {
	if let OpData::Register(d) = data {
		println!("Calling add");
		cpu.write_register(d.destination, cpu.read_register(d.source) + cpu.read_register(d.target));
	}
}