extern crate proc_macro;

use quote::*;
use syn::{
	Expr,
	ExprArray,
	parse_macro_input,
};

/// Convert a list of OpCode definitions (R, I, J type) into varying
/// parts of associated machinery.
///
/// Defining an instruction as `(NAME, fn, op, delay)`,
/// this takes 2 parameters:
/// * Switched instructions, a list of tuples (OpCode, name, decoder, list of instructions)
/// * Generic instructions, a list of instructions.
///
/// For instance, the following will register one R-type function, ADD:
/// ```rust
/// rs2_macro::ops!([
/// 	[
/// 		(MipsOpcode::Special, "R", MipsFunction::decode,
/// 			[(ADD, add, 0b100000, 1)],
/// 		),
/// 	]
/// 	[],
/// ]);
/// ```
#[proc_macro]
pub fn mips_ops(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
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

			if instruction == 0 {
				trace!("Explicit NOP");
				out.requirements = crate::core::constants::requirements::ALU.fuse_registers(Default::default());
				return out;
			}

			let raw_opcode = instruction.get_opcode();
			let opcode = crate::isa::mips::Opcode::from_u8(raw_opcode);

			match opcode {
				// R/switched instructions
				#r_type_matches_tokens

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

	for family in instructions.elems.clone().iter_mut() {
		if let Expr::Tuple(ref mut family_data) = family {
			let family_elems = &mut family_data.elems;

			assert!(family_elems.len() == 4);
			let funcs = family_elems.pop().unwrap().into_value();
			let op_codec = family_elems.pop().unwrap().into_value();
			let op_name = family_elems.pop().unwrap().into_value();
			let op_code = family_elems.pop().unwrap().into_value();

			let r_type_matches_tokens = if let Expr::Array(instruction_list) = funcs {
				Some(individual_r_type_matches(&instruction_list))
			} else {
				None
			};

			match_parts.push(quote!{
				Some(#op_code) => {
					let raw_func = instruction.r_get_function();
					let func = crate::isa::mips::Function::from_u8(raw_func);

					match #op_codec(instruction) {
						#r_type_matches_tokens
						_ => debug!(
							"Unknown {}-type instruction {:06b}: data {:020b}.",
							#op_name,
							raw_func,
							(instruction << 6) >> 12,
						),
					}
				},
			});
		}
	}

	quote!{#(#match_parts)*}
}

fn individual_r_type_matches(instructions: &ExprArray) -> proc_macro2::TokenStream {
	let mut match_parts = vec![];

	for instruction in instructions.elems.clone().iter_mut() {
		if let Expr::Tuple(ref mut instruction_data) = instruction {
			let elems = &mut instruction_data.elems;

			assert!(elems.len() == 6);
			let reg_access_func = elems.pop().unwrap().into_value();
			let pipes_set = elems.pop().unwrap().into_value();
			let func_delay = elems.pop().unwrap().into_value();
			let func_code = elems.pop().unwrap().into_value();
			let func_name = elems.pop().unwrap().into_value();
			let op_name = elems.pop().unwrap().into_value();

			match_parts.push(quote!{
				Some(#func_code) => {
					use crate::isa::mips::Requirement::*;

					let lname = stringify!(#op_name);
					trace!("{}; {:020b} {:06b}", lname, (instruction << 6) >> 12, instruction & 0b11_1111);

					let regs = #reg_access_func(instruction);
					let requirements = #pipes_set.fuse_registers(regs);

					out.action = #func_name as crate::core::pipeline::EEAction;
					out.delay = #func_delay;
					out.requirements = requirements;
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

			assert!(elems.len() == 6);
			let reg_access_func = elems.pop().unwrap().into_value();
			let pipes_set = elems.pop().unwrap().into_value();
			let func_delay = elems.pop().unwrap().into_value();
			let func_code = elems.pop().unwrap().into_value();
			let func_name = elems.pop().unwrap().into_value();
			let op_name = elems.pop().unwrap().into_value();

			match_parts.push(quote!{
				Some(#func_code) => {
					let lname = stringify!(#op_name);
					trace!("{}; {:026b}", lname, (instruction << 6) >> 6);

					let regs = #reg_access_func(instruction);
					let requirements = #pipes_set.fuse_registers(regs);

					out.action = #func_name as crate::core::pipeline::EEAction;
					out.delay = #func_delay;
					out.requirements = requirements;
				},
			});
		}
	}

	quote!{#(#match_parts)*}
}
