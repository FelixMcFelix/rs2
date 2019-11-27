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
	let mut ij_type_matches_tokens = None;

	match parsed_input {
		Expr::Array(outer_list) => {
			assert!(outer_list.elems.len() == 2);

			for (i, el) in outer_list.elems.iter().enumerate() {
				if let Expr::Array(instruction_list) = el {
					match i {
						0 => {
							r_type_matches_tokens = Some(r_type_matches(instruction_list));
						},
						1 => {
							ij_type_matches_tokens = Some(ij_type_matches(instruction_list));
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
		///
		/// NOTE: You must import [`Instruction`](pipeline/trait.Instruction.html) in the same scope
		/// as calling this macro and `FromPrimitive`.
		pub fn process_instruction(instruction: u32) -> crate::core::pipeline::OpCode {
			let mut out = crate::core::pipeline::OpCode::default();
			out.raw = instruction;

			let raw_opcode = instruction.get_opcode();
			let opcode = crate::core::ops::constants::MipsOpcode::from_u8(raw_opcode);

			match opcode {
				// R instructions
				Some(crate::core::ops::constants::MipsOpcode::Special) => {
					let raw_func = instruction.r_get_function();
					let func = crate::core::ops::constants::MipsFunction::from_u8(raw_func);

					match func {
						#r_type_matches_tokens
						_ => debug!(
							"Unknown R-type instruction {:06b}: data {:020b}.",
							raw_func,
							(instruction << 6) >> 12,
						),
					}
				},

				// I, J instructions
				#ij_type_matches_tokens

				_ => debug!(
					"Unknown opcode {:06b}: data {:026b}.",
					raw_opcode,
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
				Some(#func_code) => {
					let lname = stringify!(#op_name);
					trace!("{}; {:?}", lname, (instruction << 6) >> 12);
					out.action = &(#func_name as crate::core::pipeline::EEAction);
					out.delay = #func_delay;
				},
			});
		}
	}

	quote!{#(#match_parts)*}
}

fn ij_type_matches(instructions: &ExprArray) -> proc_macro2::TokenStream {
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
				Some(#func_code) => {
					let lname = stringify!(#op_name);
					trace!("{}; {:?}", lname, (instruction << 6) >> 6);
					out.action = &(#func_name as crate::core::pipeline::EEAction);
					out.delay = #func_delay;
				},
			});
		}
	}

	quote!{#(#match_parts)*}
}
