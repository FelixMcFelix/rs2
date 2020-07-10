mod arithmetic;
mod branch;
mod cop0;
mod cop1;
mod load;
mod store;

use crate::{
	core::{
		constants::{requirements as req, timings::*},
		pipeline::*,
		EECore,
	},
	isa::mips::{
		ee::{CacheFunction, Cop0Function, Cop1Function},
		Capability as Cap,
		Function as MipsFunction,
		Instruction,
		Opcode as MipsOpcode,
		RegImmFunction,
	},
};
use enum_primitive::FromPrimitive;

rs2_macro::mips_ops!([
	[
		(MipsOpcode::Special, "R", MipsFunction::decode, [
			(ADD, arithmetic::add, MipsFunction::Add, INTEGER_SUM_LOGIC_DELAY, req::ALU, Cap::write_d_read_ts),
			(ADDU, arithmetic::addu, MipsFunction::AddU, INTEGER_SUM_LOGIC_DELAY, req::ALU, Cap::write_d_read_ts),
			(AND, arithmetic::and, MipsFunction::And, INTEGER_SUM_LOGIC_DELAY, req::ALU, Cap::write_d_read_ts),
			(BREAK, branch::break_i, MipsFunction::Break, INTEGER_BRANCH_JUMP_DELAY, req::ALU, Cap::no_req),
			(DADDU, arithmetic::daddu, MipsFunction::DAddU, INTEGER_SUM_LOGIC_DELAY, req::ALU, Cap::write_d_read_ts),
			(DIV, arithmetic::div, MipsFunction::Div, INTEGER_DIV_DELAY, req::MAC0, Cap::mul_div),
			(DIVU, arithmetic::divu, MipsFunction::DivU, INTEGER_DIV_DELAY, req::MAC0, Cap::mul_div),
			(JALR, branch::jalr, MipsFunction::JaLR, INTEGER_BRANCH_JUMP_DELAY, req::BRANCH, Cap::jump_link_reg),
			(JR, branch::jr, MipsFunction::JR, INTEGER_BRANCH_JUMP_DELAY, req::BRANCH, Cap::jump_reg),
			(MFHI, load::mfhi, MipsFunction::MFHi, INTEGER_HI_LO_TRANSFER_DELAY, req::MAC0, Cap::write_d),
			(MFLO, load::mflo, MipsFunction::MFLo, INTEGER_HI_LO_TRANSFER_DELAY, req::MAC0, Cap::write_d),
			(MOVN, arithmetic::movn, MipsFunction::MovN, INTEGER_CONDITIONAL_MOVE_DELAY, req::ALU, Cap::write_d_read_ts),
			(MULT, arithmetic::mult, MipsFunction::Mult, INTEGER_MULT_DELAY, req::MAC0, Cap::mul_div),
			(OR, arithmetic::or, MipsFunction::Or, INTEGER_SUM_LOGIC_DELAY, req::ALU, Cap::write_d_read_ts),
			(SLL, arithmetic::sll, MipsFunction::SLL, INTEGER_SHIFT_LUI_DELAY, req::ALU, Cap::write_d_read_t),
			(SLT, arithmetic::slt, MipsFunction::SLT, INTEGER_SUM_LOGIC_DELAY, req::ALU, Cap::write_d_read_ts),
			(SLTU, arithmetic::sltu, MipsFunction::SLTU, INTEGER_SUM_LOGIC_DELAY, req::ALU, Cap::write_d_read_ts),
			(SRA, arithmetic::sra, MipsFunction::SRA, INTEGER_SHIFT_LUI_DELAY, req::ALU, Cap::write_d_read_t),
			(SRL, arithmetic::srl, MipsFunction::SRL, INTEGER_SHIFT_LUI_DELAY, req::ALU, Cap::write_d_read_t),
			(SUBU, arithmetic::subu, MipsFunction::SubU, INTEGER_SUM_LOGIC_DELAY, req::ALU, Cap::write_d_read_ts),
			(SYNC, nop, MipsFunction::Sync, INTEGER_SHIFT_LUI_DELAY, req::SYNC, Cap::no_req),
		]),
		(MipsOpcode::Cache, "CACHE", CacheFunction::decode, [
			(BFH, nop, CacheFunction::BFH, INTEGER_LOAD_STORE_DELAY, req::LS, Cap::no_req),
			(BHINBT, nop, CacheFunction::BHINBT, INTEGER_LOAD_STORE_DELAY, req::LS, Cap::no_req),
			(BXLBT, nop, CacheFunction::BXLBT, INTEGER_LOAD_STORE_DELAY, req::LS, Cap::no_req),
			(BXSBT, nop, CacheFunction::BXSBT, INTEGER_LOAD_STORE_DELAY, req::LS, Cap::no_req),
			(DHIN, nop, CacheFunction::DHIN, INTEGER_LOAD_STORE_DELAY, req::LS, Cap::no_req),
			(DHWBIN, nop, CacheFunction::DHWBIN, INTEGER_LOAD_STORE_DELAY, req::LS, Cap::no_req),
			(DHWOIN, nop, CacheFunction::DHWOIN, INTEGER_LOAD_STORE_DELAY, req::LS, Cap::no_req),
			(DXIN, nop, CacheFunction::DXIN, INTEGER_LOAD_STORE_DELAY, req::LS, Cap::no_req),
			(DXLDT, nop, CacheFunction::DXLDT, INTEGER_LOAD_STORE_DELAY, req::LS, Cap::no_req),
			(DXLTG, nop, CacheFunction::DXLTG, INTEGER_LOAD_STORE_DELAY, req::LS, Cap::no_req),
			(DXSDT, nop, CacheFunction::DXSDT, INTEGER_LOAD_STORE_DELAY, req::LS, Cap::no_req),
			(DXSTG, nop, CacheFunction::DXSTG, INTEGER_LOAD_STORE_DELAY, req::LS, Cap::no_req),
			(DXWBIN, nop, CacheFunction::DXWBIN, INTEGER_LOAD_STORE_DELAY, req::LS, Cap::no_req),
			(IFL, nop, CacheFunction::IFL, INTEGER_LOAD_STORE_DELAY, req::LS, Cap::no_req),
			(IHIN, nop, CacheFunction::IHIN, INTEGER_LOAD_STORE_DELAY, req::LS, Cap::no_req),
			(IXIN, nop, CacheFunction::IXIN, INTEGER_LOAD_STORE_DELAY, req::LS, Cap::no_req),
			(IXLDT, nop, CacheFunction::IXLDT, INTEGER_LOAD_STORE_DELAY, req::LS, Cap::no_req),
			(IXLTG, cop0::cache::ixltg, CacheFunction::IXLTG, INTEGER_LOAD_STORE_DELAY, req::LS, Cap::no_req),
			(IXSDT, nop, CacheFunction::IXSDT, INTEGER_LOAD_STORE_DELAY, req::LS, Cap::no_req),
			(IXSTG, nop, CacheFunction::IXSTG, INTEGER_LOAD_STORE_DELAY, req::LS, Cap::no_req),
		]),
		(MipsOpcode::Cop0, "COP0", Cop0Function::decode, [
			(MFBPC, cop0::mfc0, Cop0Function::MFBPC, INTEGER_LOAD_STORE_DELAY, req::COP0, Cap::write_t),
			(MFC0, cop0::mfc0, Cop0Function::MFC0, INTEGER_LOAD_STORE_DELAY, req::COP0, Cap::write_t_read_d),
			(MTBPC, cop0::mtc0, Cop0Function::MTBPC, INTEGER_LOAD_STORE_DELAY, req::COP0, Cap::read_t),
			(MTC0, cop0::mtc0, Cop0Function::MTC0, INTEGER_LOAD_STORE_DELAY, req::COP0, Cap::read_td),
			(TLBWI, cop0::tlbwi, Cop0Function::TlbWI, INTEGER_LOAD_STORE_DELAY, req::COP0, Cap::no_req),
			(TLBWR, cop0::tlbwr, Cop0Function::TlbWR, INTEGER_LOAD_STORE_DELAY, req::COP0, Cap::no_req),
		]),
		(MipsOpcode::Cop1, "COP1", Cop1Function::decode, [
			// N/A
		]),
		(MipsOpcode::RegImm, "REGIMM", RegImmFunction::decode, [
			(BGEZ, branch::bgez, RegImmFunction::BGEZ, INTEGER_BRANCH_JUMP_DELAY, req::BRANCH, Cap::branch_read_s),
			(BLTZ, branch::bltz, RegImmFunction::BLTZ, INTEGER_BRANCH_JUMP_DELAY, req::BRANCH, Cap::branch_read_s),
		]),
	],
	[
		(ADDI, arithmetic::addi, MipsOpcode::AddI, INTEGER_SUM_LOGIC_DELAY, req::ALU, Cap::write_t_read_s),
		(ADDIU, arithmetic::addiu, MipsOpcode::AddIU, INTEGER_SUM_LOGIC_DELAY, req::ALU, Cap::write_t_read_s),
		(ANDI, arithmetic::andi, MipsOpcode::AndI, INTEGER_SUM_LOGIC_DELAY, req::ALU, Cap::write_t_read_s),
		(BEQ, branch::beq, MipsOpcode::BEq, INTEGER_BRANCH_JUMP_DELAY, req::BRANCH, Cap::branch_compare),
		(BEQL, branch::beql, MipsOpcode::BEqL, INTEGER_BRANCH_JUMP_DELAY, req::BRANCH, Cap::branch_compare),
		(BGTZ, branch::bgtz, MipsOpcode::BGTZ, INTEGER_BRANCH_JUMP_DELAY, req::BRANCH, Cap::branch_compare),
		(BLEZ, branch::blez, MipsOpcode::BLEZ, INTEGER_BRANCH_JUMP_DELAY, req::BRANCH, Cap::branch_compare),
		(BNE, branch::bne, MipsOpcode::BNE, INTEGER_BRANCH_JUMP_DELAY, req::BRANCH, Cap::branch_compare),
		(BNEL, branch::bnel, MipsOpcode::BNEL, INTEGER_BRANCH_JUMP_DELAY, req::BRANCH, Cap::branch_compare),
		(J, branch::j, MipsOpcode::J, INTEGER_BRANCH_JUMP_DELAY, req::BRANCH, Cap::jump),
		(JAL, branch::jal, MipsOpcode::JaL, INTEGER_BRANCH_JUMP_DELAY, req::BRANCH, Cap::jump_link),
		(LB, load::lb, MipsOpcode::LB, INTEGER_LOAD_STORE_DELAY, req::LS, Cap::write_t_read_s),
		(LBU, load::lbu, MipsOpcode::LBU, INTEGER_LOAD_STORE_DELAY, req::LS, Cap::write_t_read_s),
		(LD, load::ld, MipsOpcode::LD, INTEGER_LOAD_STORE_DELAY, req::LS, Cap::write_t_read_s),
		(LHU, load::lhu, MipsOpcode::LHU, INTEGER_LOAD_STORE_DELAY, req::LS, Cap::write_t_read_s),
		(LUI, load::lui, MipsOpcode::LUI, INTEGER_SHIFT_LUI_DELAY, req::LS, Cap::write_t),
		(LW, load::lw, MipsOpcode::LW, INTEGER_LOAD_STORE_DELAY, req::LS, Cap::write_t_read_s),
		(ORI, arithmetic::ori, MipsOpcode::OrI, INTEGER_SUM_LOGIC_DELAY, req::ALU, Cap::write_t_read_s),
		(SB, store::sb, MipsOpcode::SB, INTEGER_LOAD_STORE_DELAY, req::LS, Cap::read_ts),
		(SD, store::sd, MipsOpcode::SD, INTEGER_LOAD_STORE_DELAY, req::LS, Cap::read_ts),
		(SLTI, arithmetic::slti, MipsOpcode::SLTI, INTEGER_SUM_LOGIC_DELAY, req::ALU, Cap::write_t_read_s),
		(SLTIU, arithmetic::sltiu, MipsOpcode::SLTIU, INTEGER_SUM_LOGIC_DELAY, req::ALU, Cap::write_t_read_s),
		(SW, store::sw, MipsOpcode::SW, INTEGER_LOAD_STORE_DELAY, req::LS, Cap::read_ts),
		(SWC1, cop1::swc1, MipsOpcode::SWC1, FLOAT_MFC1_DELAY, req::COP1_MOVE, Cap::read_s),
	],
]);

pub fn nop(_cpu: &mut EECore, _data: &OpCode) {
	// No Op.
	trace!("NOP FIRED");
}
