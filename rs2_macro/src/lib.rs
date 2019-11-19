extern crate proc_macro;

use quote::*;
use syn::{
	Expr,
	ExprArray,
	Macro,
	Path,
	parse_macro_input,
};

/// Convert a list of OpCode definitions (R, I, J type) into varying
/// parts of associated machinery.
///
/// Defining an instruction as `(NAME, fn, op, delay)`,
/// this takes 3 parameters, each a list of instructions:
/// * R instructions
/// * I instructions
/// * J instructions
///
/// For instance, the following will register one R-type function, ADD:
/// ```rust
/// rs2_macro::ops!([
/// 	[(ADD, add, 0b100000, 1)],
/// 	[],
/// 	[],
/// ]);
/// ```
#[proc_macro]
pub fn ops(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let parsed_input = parse_macro_input!(input as Expr);
	let mut r_type_matches_tokens = None;
	let mut i_type_matches_tokens = None;
	let mut j_type_matches_tokens = None;

	match parsed_input {
		Expr::Array(outer_list) => {
			assert!(outer_list.elems.len() == 3);

			for (i, el) in outer_list.elems.iter().enumerate() {
				if let Expr::Array(instruction_list) = el {
					match i {
						0 => {
							r_type_matches_tokens = Some(r_type_matches(instruction_list));
						},
						1 => {
							i_type_matches_tokens = Some(quote!{});
						},
						2 => {
							j_type_matches_tokens = Some(quote!{});
						},
						_ => panic!("There is no additional instruction family here to handle..."),
					}
				}
			}
		},
		_ => panic!("Params given to ops macro were not in a list!"),
	};

	let op_fn = quote!{
		/// Convert a 32-bit instruction into an [`OpCode`](pipeline/struct.OpCode.html), for later queueing/execution.
		pub fn process_instruction(instruction: u32) -> crate::core::pipeline::OpCode {
			let mut out = crate::core::pipeline::OpCode::default();
			let op_code = instruction >> 26;

			match op_code {
				// R instructions
				0 => {
					let func = instruction & 0b00111111;
					out.data = crate::core::pipeline::OpData::register(instruction);

					match func {
						#r_type_matches_tokens
						_ => debug!(
							"Unknown R-type instruction {:06b}: {:?}.",
							func,
							out.data,
						),
					}
				},

				// I, J instructions
				#i_type_matches_tokens
				#j_type_matches_tokens

				_ => debug!(
					"Unknown opcode {:06b}: data {:026b}.",
					op_code,
					(instruction << 6) >> 6,
				),
			}

			out
		}
	};

	proc_macro::TokenStream::from(op_fn)
}

fn r_type_matches(instructions: &ExprArray) -> proc_macro2::TokenStream {
	let mut match_parts = vec![];
	for instruction in instructions.elems.clone().iter_mut() {
		if let Expr::Tuple(ref mut instruction_data) = instruction {
			let elems = &mut instruction_data.elems;

			assert!(elems.len() == 4);
			let func_delay = elems.pop().unwrap().into_value();
			let func_code = elems.pop().unwrap().into_value();
			let func_name = elems.pop().unwrap().into_value();
			let op_name = elems.pop().unwrap().into_value();

			match_parts.push(quote!{
				#func_code => {
					let lname = stringify!(#op_name);
					trace!("{}; {:?}", lname, out.data);
					out.action = &(#func_name as crate::core::pipeline::EEAction);
					out.delay = #func_delay;
				},
			});
		}
	}

	quote!{#(#match_parts)*}
}
