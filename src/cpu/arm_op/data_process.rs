use std::sync::{Arc, Mutex};

use fantasy_util::bit::usize::BitUtil;

use crate::cpu::arm_op::instruct_execute::InstructExecute;
use crate::cpu::arm_op_table::ArmOpType;
use crate::cpu::reg::Register;

struct DataProcess {}

impl DataProcess {
    fn execute(op: u32, reg: &mut Register, op_type: ArmOpType) {
        let opcode_type: OpcodeType = (op.extract(21, 4) as u8).into();
        let s = op.get_bit_bool(20);

        let operand1 = reg.reg_val(rn);
        // resutls ( 0:result value   1:flag_C   2:flag_V )
        let results: (u32, bool, bool) = match opcode_type {
            OpcodeType::AND => {}
            OpcodeType::EOR => {}
            OpcodeType::SUB => {}
            OpcodeType::RSB => {}
            OpcodeType::ADD => {}
            OpcodeType::ADC => {}
            OpcodeType::SBC => {}
            OpcodeType::RSC => {}

            OpcodeType::TST => {}
            OpcodeType::TEQ => {}
            OpcodeType::CMP => {}
            OpcodeType::CMN => {}

            OpcodeType::ORR => {}
            OpcodeType::MOV => {}
            OpcodeType::BIC => {}
            OpcodeType::MVN => {}
        };
        if s { // If S bit is '1' , set condition codes
            if rd != 15 {
                let new_n = (results.0 as i32) < 0;
                // TODO flag_c,flag_v
                reg.cspr.set_flag_nzcv(new_n, results.0 == 0, results.1, results.2);
            } else {
                reg.cspr.set_val(reg.spsr())
                // self.reg[reg::CPSR] = self.reg[reg::SPSR];
                // self.reg.update_bank();
            }
        } else if !s && rd == 15 {
            reg.set_reg(rd, results.0)
        }
    }

    fn add(set_cond: bool) {}

    fn base_info(op: u32, reg: &mut Register) {
        let rn = op.extract(16, 4) as u8;
        let rd = op.extract(12, 4) as u8;
        if op.get_bit_bool(25) { // 立即数
            let imm = op & 0b1111_1111;
            let rotate = op.extract(8, 4);
            // TODO
        } else {
            let rm = op.extract(0, 4) as u8;
            let rm_val = reg.reg_val(rm);
            let bit5 = op.get_bit_bool(4);
            let shift_amount = if bit5 {
                let rs = op.extract(8, 4) as u8;
                reg.reg_val(rs) as u8 & 0b00011111
            } else {
                op.extract(7, 5) as u8
            };
            // 寄存器移位
        }
    }
}

enum OpcodeType {
    AND,
    EOR,
    SUB,
    RSB,
    ADD,
    ADC,
    SBC,
    RSC,
    TST,
    TEQ,
    CMP,
    CMN,
    ORR,
    MOV,
    BIC,
    MVN,
}

impl From<u8> for OpcodeType {
    fn from(opcode: u8) -> Self {
        match opcode {
            0b0000 => OpcodeType::AND,
            0b0001 => OpcodeType::EOR,
            0b0010 => OpcodeType::SUB,
            0b0011 => OpcodeType::RSB,
            0b0100 => OpcodeType::ADD,
            0b0101 => OpcodeType::ADC,
            0b0110 => OpcodeType::SBC,
            0b0111 => OpcodeType::RSC,
            0b1000 => OpcodeType::TST,
            0b1001 => OpcodeType::TEQ,
            0b1010 => OpcodeType::CMP,
            0b1011 => OpcodeType::CMN,
            0b1100 => OpcodeType::ORR,
            0b1101 => OpcodeType::MOV,
            0b1110 => OpcodeType::BIC,
            0b1111 => OpcodeType::MVN,
            n => panic!(format!("Unknow opcode 0x{:X}", n))
        }
    }
}

enum ShiftType {
    LogicalLeft,
    LogicalRight,
    ArithmeticRight,
    RotateRight,
}