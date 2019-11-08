use byteorder::{
	LittleEndian,
	ReadBytesExt,
};
use std::{
	convert::TryInto,
	fs::File,
	io::Read,
};

pub mod core;

use crate::core::*;

fn main() {
	let mut ee_core = EECore::new();

	if let Ok(mut f) = File::open("bios/scph39001.bin") {
		let mut prog_buf = if let Ok(metadata) = f.metadata() {
			Vec::with_capacity(metadata.len().try_into().unwrap())
		} else {
			vec![]
		};

		if f.read_to_end(&mut prog_buf).is_ok() {
			// let active = true;
			// while active {
			loop {
				ee_core.cycle(&prog_buf);
			}
		}
	}
}
