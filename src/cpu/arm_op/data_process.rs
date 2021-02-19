use std::sync::{Arc, Mutex};

use crate::cpu::arm_op::instruct_execute::InstructExecute;
use crate::cpu::reg::Register;

struct DataProcess {}

impl InstructExecute for DataProcess {
    fn execute(reg: &mut Register) {
        let i = op.get_bit_bool(25);
        let opcode_type: OpcodeType = (op.extract(21, 4) as u8).into();
        let s = op.get_bit_bool(20);
        let rn = op.extract(16, 4) as u8;
        let rd = op.extract(12, 4) as u8;
        let operand2 = op.extract(0, 12);
        if i {
            // 立即数
        } else {

            // 寄存器移位
        }
        let operand1 = reg.reg_val(rn);
        // resutls ( 0:result value   1:flag_C   2:flag_V )
        let results: (u32, bool, bool) = match opcode_type {
            OpcodeType::AND => {
                (1, true, true)
            }
            OpcodeType::EOR => {
                (1, true, true)
            }
            OpcodeType::SUB => {
                // logic
                (1, true, true)
            }
            OpcodeType::RSB => {
                // logic
                (1, true, true)
            }
            OpcodeType::ADD => {
                // logic
                (1, true, true)
            }
            OpcodeType::ADC => {
                // logic
                (1, true, true)
            }
            OpcodeType::SBC => {
                // logic
                (1, true, true)
            }
            OpcodeType::RSC => {
                // logic
                (1, true, true)
            }
            OpcodeType::TST => {
                (1, true, true)
            }
            OpcodeType::TEQ => {
                (1, true, true)
            }
            OpcodeType::CMP => {
                // logic
                (1, true, true)
            }
            OpcodeType::CMN => {
                // logic
                (1, true, true)
            }
            OpcodeType::ORR => {
                (1, true, true)
            }
            OpcodeType::MOV => {
                (1, true, true)
            }
            OpcodeType::BIC => {
                (1, true, true)
            }
            OpcodeType::MVN => {
                (1, true, true)
            }
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