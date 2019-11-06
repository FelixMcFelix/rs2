extern crate proc_macro;

use quote::*;
use syn::{
	Expr,
	parse_macro_input,
};

/// Convert a list of OpCode definitions (R, I, J type) into varying
/// parts of associated machinery.
#[proc_macro]
pub fn ops(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let parsed_input = parse_macro_input!(input as Expr);

	let rs: Vec<u8> = match parsed_input {
		Expr::Array(a) => {
			eprintln!("Seeing {} children!", a.elems.len());
			assert!(a.elems.len() == 3);
			vec![]
		},
		_ => {
			eprintln!("Innards had another type...");
			vec![]
		},
	};

	// Step one: write a "do op" action.
	let op_fn = quote!{
		fn execute_instruction(cpu: &mut EECore, instruction: u32) {
			let op_code = instruction >> 26;

			match op_code {
				// R instructions
				0 => {
					let func = instruction        & 0b00111111;
					let sh = ((instruction >> 6)  & 0b00011111) as u8;
					let rd = ((instruction >> 11) & 0b00011111) as u8;
					let rt = ((instruction >> 16) & 0b00011111) as u8;
					let rs = ((instruction >> 21) & 0b00011111) as u8;

					match func {
						_ => eprintln!(
							"Unknown R-type instruction {:06b}: rs {}, rt {}, rd {}, sh {}.",
							func,
							rs,
							rt,
							rd,
							sh,
						),
					}
				},

				_ => eprintln!(
					"Unknown opcode {:06b}: data {:26b}.",
					op_code,
					(instruction << 6) >> 6,
				),
			}
		}
	};

	proc_macro::TokenStream::from(op_fn)
}


