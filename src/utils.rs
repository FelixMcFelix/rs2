use byteorder::{
	ByteOrder,
	LittleEndian,
};
use crate::core::EECore;

pub fn install_and_run_program(cpu: &mut EECore, program: Vec<u8>) {
	let duration = program.len();
	install_and_run_program_for(cpu, program, duration);
}

pub fn install_and_run_program_for(cpu: &mut EECore, program: Vec<u8>, duration: usize) {
	cpu.set_bios(program);
	for _ in 0..duration {
		cpu.cycle();
	}
}

pub fn instructions_to_bytes(program: &Vec<u32>) -> Vec<u8> {
	let mut program_bytes = vec![0u8; 4 * program.len()];
	LittleEndian::write_u32_into(&program[..], &mut program_bytes[..]);

	program_bytes
}

pub trait SignExtend<T> {
	fn s_ext(self) -> T;
}

impl SignExtend<u32> for i16 {
	fn s_ext(self) -> u32 {
		self as u32
	}
}

impl SignExtend<u32> for u16 {
	fn s_ext(self) -> u32 {
		(self as i16).s_ext()
	}
}

impl SignExtend<u64> for i16 {
	fn s_ext(self) -> u64 {
		self as u64
	}
}

impl SignExtend<u64> for u16 {
	fn s_ext(self) -> u64 {
		(self as i16).s_ext()
	}
}

impl SignExtend<u64> for i32 {
	fn s_ext(self) -> u64 {
		self as u64
	}
}

impl SignExtend<u64> for u32 {
	fn s_ext(self) -> u64 {
		(self as i32).s_ext()
	}
}

pub trait ZeroExtend<T> {
	fn z_ext(self) -> T;
}

impl ZeroExtend<u32> for i16 {
	fn z_ext(self) -> u32 {
		(self as u16).z_ext()
	}
}

impl ZeroExtend<u32> for u16 {
	fn z_ext(self) -> u32 {
		self as u32
	}
}

impl ZeroExtend<u64> for i16 {
	fn z_ext(self) -> u64 {
		(self as u16).z_ext()
	}
}

impl ZeroExtend<u64> for u16 {
	fn z_ext(self) -> u64 {
		self as u64
	}
}

impl ZeroExtend<u64> for i32 {
	fn z_ext(self) -> u64 {
		(self as u32).z_ext()
	}
}

impl ZeroExtend<u64> for u32 {
	fn z_ext(self) -> u64 {
		self as u64
	}
}