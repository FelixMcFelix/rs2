use byteorder::{
	LittleEndian,
	ReadBytesExt,
};
use std::{
	fs::File,
};

mod core;

use crate::core::*;

fn main() {
	// Whst to do...
	// 1) Open file (maybe BIOS program?).
	// 2) iterate over each 32-bit word.
	// 3) print first 6 bits, last 6 bits if "special".
	let mut ee_core = EECore::new();

	if let Ok(mut f) = File::open("bios/scph39001.bin") {
		while let Ok(word) = f.read_u32::<LittleEndian>(){
			let special_code = word & 0b0000000_00111111;
			let op_code = word >> 26;

			if op_code == 0 {
				println!("Special: {:06b}", special_code);
			} else {
				println!("Op: {:06b}", op_code);

				let rs = ((word & 0b00000000_00000111_11000000) >> 6) as usize;
				let rt = ((word & 0b00000000_11111000_00000000) >> 11) as usize;
				let rd = ((word & 0b00011111_00000000_00000000) >> 16) as usize;
				if op_code == 1 {
					// Test out add.
					// Need to do three things: read rd, rs, rt. This logic can likely be generalised.
					// I.e., learn how to macro.

					println!("{:?}, {:?}, {:?}", rs, rt, rd);
					// Todo: make these work w/ 32-bit numbers...
					ee_core.register_file[rd * REGISTER_WIDTH_BYTES] = ee_core.register_file[rs * REGISTER_WIDTH_BYTES] + ee_core.register_file[rt * REGISTER_WIDTH_BYTES];
					println!("{:?}", ee_core.register_file[rd * REGISTER_WIDTH_BYTES]);
				}
			}
		}
	}
}
