use super::tcg::{TCGLabel, TCGOp, TCGOpcode, TCGv};
use std::cell::RefCell;
use std::rc::Rc;

macro_rules! get_rs1_addr {
    ($inst:expr) => {
        ($inst >> 15) & 0x1f
    };
}

macro_rules! get_rs2_addr {
    ($inst:expr) => {
        ($inst >> 20) & 0x1f
    };
}

#[allow(unused_macros)]
macro_rules! get_rs3_addr {
    ($inst:expr) => {
        ($inst >> 27) & 0x1f
    };
}

macro_rules! get_rd_addr {
    ($inst:expr) => {
        ($inst >> 7) & 0x1f
    };
}

#[allow(unused_macros)]
macro_rules! get_imm12 {
    ($inst:expr) => {
        ($inst >> 20) as u64
    };
}

macro_rules! get_sb_field {
    ($inst:expr) => {
        ((($inst as u64 >> 7) & 0x01) << 11)
            | ((($inst as u64 >> 8) & 0x0f) << 1)
            | ((($inst as u64 >> 25) & 0x3f) << 5)
            | ((($inst as u64 >> 31) & 0x01) << 12) as u64
    };
}

pub struct TranslateRiscv;

impl TranslateRiscv {
    pub fn translate_jalr(inst: &u32) -> Vec<TCGOp> {
        let rs1_addr: usize = get_rs1_addr!(*inst) as usize;
        let imm_const: u64 = (*inst as u64) >> 20 & 0xfff;
        let rd_addr: usize = get_rd_addr!(*inst) as usize;

        let rs1 = Box::new(TCGv::new_reg(rs1_addr as u64));
        let imm = Box::new(TCGv::new_imm(imm_const));
        let rd = Box::new(TCGv::new_reg(rd_addr as u64));

        // let tcg_inst = TCGOp::new_3op(TCGOpcode::JMP, *rd, *rs1, *imm));
        let tcg_inst = TCGOp::new_3op(TCGOpcode::JMP, *rd, *rs1, *imm);

        vec![tcg_inst]
    }

    pub fn translate_lui(inst: &u32) -> Vec<TCGOp> {
        let imm_const: u64 = (*inst as u64) & !0xfff;
        let rd_addr: usize = get_rd_addr!(*inst) as usize;

        let rs1 = Box::new(TCGv::new_reg(0));
        let imm = Box::new(TCGv::new_imm(imm_const));
        let rd = Box::new(TCGv::new_reg(rd_addr as u64));

        let tcg_inst = TCGOp::new_3op(TCGOpcode::ADD, *rd, *rs1, *imm);

        vec![tcg_inst]
    }

    fn translate_rrr(op: TCGOpcode, inst: &u32) -> Vec<TCGOp> {
        let rs1_addr: usize = get_rs1_addr!(*inst) as usize;
        let rs2_addr: usize = get_rs2_addr!(*inst) as usize;
        let rd_addr: usize = get_rd_addr!(*inst) as usize;

        let rs1 = Box::new(TCGv::new_reg(rs1_addr as u64));
        let rs2 = Box::new(TCGv::new_reg(rs2_addr as u64));
        let rd = Box::new(TCGv::new_reg(rd_addr as u64));

        let tcg_inst = TCGOp::new_3op(op, *rd, *rs1, *rs2);

        vec![tcg_inst]
    }

    fn translate_rri(op: TCGOpcode, inst: &u32) -> Vec<TCGOp> {
        let rs1_addr: usize = get_rs1_addr!(*inst) as usize;
        let imm_const: u64 = (*inst >> 20) as u64;
        let rd_addr: usize = get_rd_addr!(*inst) as usize;

        let rs1 = Box::new(TCGv::new_reg(rs1_addr as u64));
        let imm = Box::new(TCGv::new_imm(imm_const));
        let rd = Box::new(TCGv::new_reg(rd_addr as u64));

        let tcg_inst = TCGOp::new_3op(op, *rd, *rs1, *imm);

        vec![tcg_inst]
    }

    fn translate_branch(op: TCGOpcode, inst: &u32) -> Vec<TCGOp> {
        let rs1_addr: usize = get_rs1_addr!(*inst) as usize;
        let rs2_addr: usize = get_rs2_addr!(*inst) as usize;
        let target: u64 = get_sb_field!(*inst);

        let rs1 = Box::new(TCGv::new_reg(rs1_addr as u64));
        let rs2 = Box::new(TCGv::new_reg(rs2_addr as u64));
        let addr = Box::new(TCGv::new_imm(target));

        let label = Rc::new(RefCell::new(TCGLabel::new()));

        let tcg_inst = TCGOp::new_4op(op, *rs1, *rs2, *addr, Rc::clone(&label));
        let tcg_true_tb = TCGOp::new_goto_tb(TCGv::new_imm(4));
        let tcg_set_label = TCGOp::new_label(Rc::clone(&label));
        let tcg_false_tb = TCGOp::new_goto_tb(TCGv::new_imm(target));

        vec![tcg_inst, tcg_true_tb, tcg_set_label, tcg_false_tb]
    }

    pub fn translate_add(inst: &u32) -> Vec<TCGOp> {
        Self::translate_rrr(TCGOpcode::ADD, inst)
    }
    pub fn translate_sub(inst: &u32) -> Vec<TCGOp> {
        Self::translate_rrr(TCGOpcode::SUB, inst)
    }
    pub fn translate_and(inst: &u32) -> Vec<TCGOp> {
        Self::translate_rrr(TCGOpcode::AND, inst)
    }
    pub fn translate_or(inst: &u32) -> Vec<TCGOp> {
        Self::translate_rrr(TCGOpcode::OR, inst)
    }
    pub fn translate_xor(inst: &u32) -> Vec<TCGOp> {
        Self::translate_rrr(TCGOpcode::XOR, inst)
    }

    pub fn translate_addi(inst: &u32) -> Vec<TCGOp> {
        Self::translate_rri(TCGOpcode::ADD, inst)
    }
    pub fn translate_andi(inst: &u32) -> Vec<TCGOp> {
        Self::translate_rri(TCGOpcode::AND, inst)
    }
    pub fn translate_ori(inst: &u32) -> Vec<TCGOp> {
        Self::translate_rri(TCGOpcode::OR, inst)
    }
    pub fn translate_xori(inst: &u32) -> Vec<TCGOp> {
        Self::translate_rri(TCGOpcode::XOR, inst)
    }

    pub fn translate_beq(inst: &u32) -> Vec<TCGOp> {
        Self::translate_branch(TCGOpcode::EQ, inst)
    }
}