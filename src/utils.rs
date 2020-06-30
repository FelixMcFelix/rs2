use byteorder::{
	ByteOrder,
	LittleEndian,
};
use crate::{
	core::{
		pipeline::*,
		EECore,
	},
	isa::mips::{
		ee::{CacheFunction, Cop0Function, Cop1Function},
		Function as MipsFunction,
		Instruction,
		Opcode as MipsOpcode,
		RegImmFunction,
	},
};

pub fn install_and_run_program(cpu: &mut EECore, program: Vec<u8>) {
	let duration = program.len() / 4;
	install_and_run_program_for(cpu, program, duration);
}

pub fn install_and_run_program_for(cpu: &mut EECore, program: Vec<u8>, duration: usize) {
	cpu.set_bios(program);
	for _ in 0..duration {
		cpu.cycle();
	}
}

pub fn instructions_to_bytes(program: &[u32]) -> Vec<u8> {
	let mut program_bytes = vec![0u8; 4 * program.len()];
	LittleEndian::write_u32_into(&program[..], &mut program_bytes[..]);

	program_bytes
}

#[inline(always)]
pub fn v_addr_with_offset(cpu: &EECore, data: &OpCode) -> u32 {
	let offset: u32 = data.i_get_immediate_signed().s_ext();
	(cpu.read_register(data.ri_get_source()) as u32).wrapping_add(offset)
}

#[inline(always)]
pub fn v_addr_with_offset_branch(cpu: &EECore, data: &BranchOpCode) -> u32 {
	let offset: u32 = data.i_get_immediate_signed().s_ext();
	cpu.pc_register.wrapping_add(offset << 2)
}

pub trait SignExtend<T> {
	fn s_ext(self) -> T;
}

impl SignExtend<u16> for i8 {
	fn s_ext(self) -> u16 {
		self as u16
	}
}

impl SignExtend<u16> for u8 {
	fn s_ext(self) -> u16 {
		(self as i8).s_ext()
	}
}

impl SignExtend<u32> for i8 {
	fn s_ext(self) -> u32 {
		self as u32
	}
}

impl SignExtend<u32> for u8 {
	fn s_ext(self) -> u32 {
		(self as i8).s_ext()
	}
}

impl SignExtend<u64> for i8 {
	fn s_ext(self) -> u64 {
		self as u64
	}
}

impl SignExtend<u64> for u8 {
	fn s_ext(self) -> u64 {
		(self as i8).s_ext()
	}
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

impl ZeroExtend<u16> for i8 {
	fn z_ext(self) -> u16 {
		(self as u8).z_ext()
	}
}

impl ZeroExtend<u16> for u8 {
	fn z_ext(self) -> u16 {
		self as u16
	}
}

impl ZeroExtend<u32> for i8 {
	fn z_ext(self) -> u32 {
		(self as u8).z_ext()
	}
}

impl ZeroExtend<u32> for u8 {
	fn z_ext(self) -> u32 {
		self as u32
	}
}

impl ZeroExtend<u64> for i8 {
	fn z_ext(self) -> u64 {
		(self as u8).z_ext()
	}
}

impl ZeroExtend<u64> for u8 {
	fn z_ext(self) -> u64 {
		self as u64
	}
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