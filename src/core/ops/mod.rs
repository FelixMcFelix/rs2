mod arithmetic;
mod branch;
pub mod constants;
mod cop0;
mod cop1;
mod load;
mod store;

pub use constants::*;
use crate::{
	core::{
		pipeline::*,
		EECore,
	},
	isa::mips::{
		ee::{CacheFunction, Cop0Function, Cop1Function},
		Function as MipsFunction,
		Instruction,
		Opcode as MipsOpcode,
		RegImmFunction,
	},
};
use enum_primitive::FromPrimitive;
use timings::*;

rs2_macro::mips_ops!([
	[
		(MipsOpcode::Special, "R", MipsFunction::decode, [
			(ADD, arithmetic::add, MipsFunction::Add, INTEGER_SUM_LOGIC_DELAY),
			(ADDU, arithmetic::addu, MipsFunction::AddU, INTEGER_SUM_LOGIC_DELAY),
			(AND, arithmetic::and, MipsFunction::And, INTEGER_SUM_LOGIC_DELAY),
			(BREAK, branch::break_i, MipsFunction::Break, INTEGER_BRANCH_JUMP_DELAY),
			(DADDU, arithmetic::daddu, MipsFunction::DAddU, INTEGER_SUM_LOGIC_DELAY),
			(DIV, arithmetic::div, MipsFunction::Div, INTEGER_DIV_DELAY),
			(DIVU, arithmetic::divu, MipsFunction::DivU, INTEGER_DIV_DELAY),
			(MFHI, load::mfhi, MipsFunction::MFHi, INTEGER_HI_LO_TRANSFER_DELAY),
			(MFLO, load::mflo, MipsFunction::MFLo, INTEGER_HI_LO_TRANSFER_DELAY),
			(JALR, branch::jalr, MipsFunction::JaLR, INTEGER_BRANCH_JUMP_DELAY),
			(JR, branch::jr, MipsFunction::JR, INTEGER_BRANCH_JUMP_DELAY),
			(MOVN, arithmetic::movn, MipsFunction::MovN, INTEGER_CONDITIONAL_MOVE_DELAY),
			(MULT, arithmetic::mult, MipsFunction::Mult, INTEGER_MULT_DELAY),
			(OR, arithmetic::or, MipsFunction::Or, INTEGER_SUM_LOGIC_DELAY),
			(SLL, arithmetic::sll, MipsFunction::SLL, INTEGER_SHIFT_LUI_DELAY),
			(SLT, arithmetic::slt, MipsFunction::SLT, INTEGER_SUM_LOGIC_DELAY),
			(SLTU, arithmetic::sltu, MipsFunction::SLTU, INTEGER_SUM_LOGIC_DELAY),
			(SRA, arithmetic::sra, MipsFunction::SRA, INTEGER_SHIFT_LUI_DELAY),
			(SRL, arithmetic::srl, MipsFunction::SRL, INTEGER_SHIFT_LUI_DELAY),
			(SUBU, arithmetic::subu, MipsFunction::SubU, INTEGER_SUM_LOGIC_DELAY),
			(SYNC, nop, MipsFunction::Sync, INTEGER_SHIFT_LUI_DELAY),
		]),
		(MipsOpcode::Cache, "CACHE", CacheFunction::decode, [
			(BFH, nop, CacheFunction::BFH, INTEGER_LOAD_STORE_DELAY),
			(BHINBT, nop, CacheFunction::BHINBT, INTEGER_LOAD_STORE_DELAY),
			(BXLBT, nop, CacheFunction::BXLBT, INTEGER_LOAD_STORE_DELAY),
			(BXSBT, nop, CacheFunction::BXSBT, INTEGER_LOAD_STORE_DELAY),
			(DHIN, nop, CacheFunction::DHIN, INTEGER_LOAD_STORE_DELAY),
			(DHWBIN, nop, CacheFunction::DHWBIN, INTEGER_LOAD_STORE_DELAY),
			(DHWOIN, nop, CacheFunction::DHWOIN, INTEGER_LOAD_STORE_DELAY),
			(DXIN, nop, CacheFunction::DXIN, INTEGER_LOAD_STORE_DELAY),
			(DXLDT, nop, CacheFunction::DXLDT, INTEGER_LOAD_STORE_DELAY),
			(DXLTG, nop, CacheFunction::DXLTG, INTEGER_LOAD_STORE_DELAY),
			(DXSDT, nop, CacheFunction::DXSDT, INTEGER_LOAD_STORE_DELAY),
			(DXSTG, nop, CacheFunction::DXSTG, INTEGER_LOAD_STORE_DELAY),
			(DXWBIN, nop, CacheFunction::DXWBIN, INTEGER_LOAD_STORE_DELAY),
			(IFL, nop, CacheFunction::IFL, INTEGER_LOAD_STORE_DELAY),
			(IHIN, nop, CacheFunction::IHIN, INTEGER_LOAD_STORE_DELAY),
			(IXIN, nop, CacheFunction::IXIN, INTEGER_LOAD_STORE_DELAY),
			(IXLDT, nop, CacheFunction::IXLDT, INTEGER_LOAD_STORE_DELAY),
			(IXLTG, cop0::cache::ixltg, CacheFunction::IXLTG, INTEGER_LOAD_STORE_DELAY),
			(IXSDT, nop, CacheFunction::IXSDT, INTEGER_LOAD_STORE_DELAY),
			(IXSTG, nop, CacheFunction::IXSTG, INTEGER_LOAD_STORE_DELAY),
		]),
		(MipsOpcode::Cop0, "COP0", Cop0Function::decode, [
			(MFBPC, cop0::mfc0, Cop0Function::MFBPC, INTEGER_LOAD_STORE_DELAY),
			(MFC0, cop0::mfc0, Cop0Function::MFC0, INTEGER_LOAD_STORE_DELAY),
			(MTBPC, cop0::mtc0, Cop0Function::MTBPC, INTEGER_LOAD_STORE_DELAY),
			(MTC0, cop0::mtc0, Cop0Function::MTC0, INTEGER_LOAD_STORE_DELAY),
			(TLBWI, cop0::tlbwi, Cop0Function::TlbWI, INTEGER_LOAD_STORE_DELAY),
			(TLBWR, cop0::tlbwr, Cop0Function::TlbWR, INTEGER_LOAD_STORE_DELAY),
		]),
		(MipsOpcode::Cop1, "COP1", Cop1Function::decode, [
			// N/A
		]),
		(MipsOpcode::RegImm, "REGIMM", RegImmFunction::decode, [
			(BGEZ, branch::bgez, RegImmFunction::BGEZ, INTEGER_BRANCH_JUMP_DELAY),
			(BLTZ, branch::bltz, RegImmFunction::BLTZ, INTEGER_BRANCH_JUMP_DELAY),
		]),
	],
	[
		(ADDI, arithmetic::addi, MipsOpcode::AddI, INTEGER_SUM_LOGIC_DELAY),
		(ADDIU, arithmetic::addiu, MipsOpcode::AddIU, INTEGER_SUM_LOGIC_DELAY),
		(ANDI, arithmetic::andi, MipsOpcode::AndI, INTEGER_SUM_LOGIC_DELAY),
		(BEQ, branch::beq, MipsOpcode::BEq, INTEGER_BRANCH_JUMP_DELAY),
		(BEQL, branch::beql, MipsOpcode::BEqL, INTEGER_BRANCH_JUMP_DELAY),
		(BGTZ, branch::bgtz, MipsOpcode::BGTZ, INTEGER_BRANCH_JUMP_DELAY),
		(BLEZ, branch::blez, MipsOpcode::BLEZ, INTEGER_BRANCH_JUMP_DELAY),
		(BNE, branch::bne, MipsOpcode::BNE, INTEGER_BRANCH_JUMP_DELAY),
		(BNEL, branch::bnel, MipsOpcode::BNEL, INTEGER_BRANCH_JUMP_DELAY),
		(J, branch::j, MipsOpcode::J, INTEGER_BRANCH_JUMP_DELAY),
		(JAL, branch::jal, MipsOpcode::JaL, INTEGER_BRANCH_JUMP_DELAY),
		(LB, load::lb, MipsOpcode::LB, INTEGER_LOAD_STORE_DELAY),
		(LBU, load::lbu, MipsOpcode::LBU, INTEGER_LOAD_STORE_DELAY),
		(LD, load::ld, MipsOpcode::LD, INTEGER_LOAD_STORE_DELAY),
		(LHU, load::lhu, MipsOpcode::LHU, INTEGER_LOAD_STORE_DELAY),
		(LUI, load::lui, MipsOpcode::LUI, INTEGER_SHIFT_LUI_DELAY),
		(LW, load::lw, MipsOpcode::LW, INTEGER_LOAD_STORE_DELAY),
		(ORI, arithmetic::ori, MipsOpcode::OrI, INTEGER_SUM_LOGIC_DELAY),
		(SB, store::sb, MipsOpcode::SB, INTEGER_LOAD_STORE_DELAY),
		(SD, store::sd, MipsOpcode::SD, INTEGER_LOAD_STORE_DELAY),
		(SLTI, arithmetic::slti, MipsOpcode::SLTI, INTEGER_SUM_LOGIC_DELAY),
		(SLTIU, arithmetic::sltiu, MipsOpcode::SLTIU, INTEGER_SUM_LOGIC_DELAY),
		(SW, store::sw, MipsOpcode::SW, INTEGER_LOAD_STORE_DELAY),
		(SWC1, cop1::swc1, MipsOpcode::SWC1, FLOAT_MFC1_DELAY),
	],
]);

pub fn nop(_cpu: &mut EECore, _data: &OpCode) {
	// No Op.
	trace!("NOP FIRED");
}
