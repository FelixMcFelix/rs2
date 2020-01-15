pub mod constants;

use constants::*;

pub struct Memory {
	bios: Vec<u8>,
	data: Vec<u8>,
}

impl Memory {
	pub fn new(bios: Vec<u8>) -> Self {
		Self {
			bios,
			data: vec![0; PHYSICAL_MEMORY_SIZE], 
		}
	}

	pub fn set_bios(&mut self, bios: Vec<u8>) {
		self.bios = bios;
	}

	/// Read a slice of the desired size from the specified physical address.
	pub fn read(&self, addr: usize, size: usize) -> &[u8] {
		match addr {
			0..=IO_REGISTERS_PHYSICAL => {
				&self.data[addr..addr+size]
			},
			BIOS_PHYSICAL..=0xFFFF_FFFF => {
				let bios_addr = addr - BIOS_PHYSICAL;
				&self.bios[bios_addr..bios_addr + size]
			}
			_ => &self.data[..]
		}
	}
}