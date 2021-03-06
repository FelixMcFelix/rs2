#[macro_use] extern crate log;
use std::{
	convert::TryInto,
	fs::File,
	io::{
		self,
		Read,
	},
};

pub mod core;
pub mod debugger;
pub mod memory;
pub mod isa;
pub mod utils;

use crate::core::*;

fn main() {
	env_logger::init();
	
	let mut ee_core = EECore::default();

	// if let Ok(mut f) = File::open("bios/scph39001.bin") {
	if let Ok(mut f) = File::open("bios/scph10000.bin") {
		let mut prog_buf = if let Ok(metadata) = f.metadata() {
			Vec::with_capacity(metadata.len().try_into().unwrap())
		} else {
			vec![]
		};

		if f.read_to_end(&mut prog_buf).is_ok() {
			ee_core.set_bios(prog_buf);

			let stdin = io::stdin();
			let mut s = String::new();
			println!("Stepped execution: press enter to cycle.");
			loop {
				// let _ = stdin.read_line(&mut s);
				ee_core.cycle();
			}
		}
	}
}
