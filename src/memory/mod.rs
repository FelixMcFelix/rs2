pub mod constants;
pub mod mmu;

use mmu::MmuAddress;

use constants::*;

pub struct Memory {
	bios: Vec<u8>,
	data: Vec<u8>,
	scratchpad: Vec<u8>,
}

impl Memory {
	pub fn new(bios: Vec<u8>) -> Self {
		Self {
			bios,
			data: vec![0; PHYSICAL_MEMORY_SIZE],
			scratchpad: vec![0; SPRAM_SIZE],
		}
	}

	pub fn set_bios(&mut self, bios: Vec<u8>) {
		self.bios = bios;
	}

	/// Read a slice of the desired size from the specified physical address.
	pub fn read(&self, addr: MmuAddress, size: usize) -> &[u8] {
		use MmuAddress::*;
		match addr {
			Address(a) => {
				match a {
					0..=IO_REGISTERS_PHYSICAL => {
						let u_addr = a as usize;
						&self.data[u_addr..u_addr+PHYSICAL_MEMORY_SIZE]
					},
					BIOS_PHYSICAL..=0xFFFF_FFFF => {
						let bios_addr = (a - BIOS_PHYSICAL) as usize;
						&self.bios[bios_addr..bios_addr + size]
					}
					_ => &self.data[..],
				}
			},
			Scratchpad(a) => &self.scratchpad[a as usize..size],
			_ => unreachable!(),
		}
	}

	/// Read a slice of the desired size from the specified physical address.
	pub fn read_mut(&mut self, addr: MmuAddress, size: usize) -> &mut [u8] {
		use MmuAddress::*;
		match addr {
			Address(a) => {
				match a {
					0..=IO_REGISTERS_PHYSICAL => {
						let u_addr = a as usize;
						&mut self.data[u_addr..u_addr+PHYSICAL_MEMORY_SIZE]
					},
					BIOS_PHYSICAL..=0xFFFF_FFFF => {
						let bios_addr = (a - BIOS_PHYSICAL) as usize;
						&mut self.bios[bios_addr..bios_addr + size]
					}
					_ => &mut self.data[..],
				}
			},
			Scratchpad(a) => &mut self.scratchpad[a as usize..size],
			_ => unreachable!(),
		}
	}

	pub fn write(&mut self, addr: MmuAddress, data: &[u8]) {
		let dest = self.read_mut(addr, data.len());
		println!("{:?} {:?}", data.len(), dest.len());
		dest.copy_from_slice(data);
	}
}