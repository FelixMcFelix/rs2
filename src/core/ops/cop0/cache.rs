use crate::{
	core::{
		cop0::Register,
		ops::instruction::Instruction,
		pipeline::*,
		EECore,
	},
	utils::*,
};

pub fn ixltg(cpu: &mut EECore, data: &OpCode) {
	trace!("FIXME: IXLTG is basically NOP atm.");

	if !super::cop0_usable(cpu) {
		return;
	}
	// Read from *instruction cache entries*.

	// Use GPR[rs] as a base pointer, w/ immediate offset.
	// This computes a virtual address,
	// which is ysed to populate bitfields of COP0

	// sign extend offset, add base pointer.
	let _v_addr = v_addr_with_offset(cpu, data);

	// FIXME: Need to perform address translation (which may trigger page fault etc.)

	// FIXME: Need to get tag associated with this element.

	let _taglo = cpu.read_cop0(Register::TagLo as u8);
}

#[cfg(test)]
mod tests {
	use super::*;
	use byteorder::{
		ByteOrder,
		LittleEndian,
	};
	use crate::{
		core::{
			constants::*,
			cop0::Register,
			ops::{
				self,
				constants::*,
			},
		},
		memory::constants::*,
	};

	#[test]
	fn basic_ixltg() {
		unimplemented!();
	}
}
