use fantasy_util::bit::usize::BitUtil;
use crate::cpu::arm_op::barrel_shifter;

use crate::cpu::reg::{R15, Register};

pub struct DataProcess {}

impl DataProcess {

    // ----------------逻辑操作----------------

    pub fn and(set_flag: bool, op: u32, reg: &mut Register) {
        let (operand1, rd) = base(op, reg);
        let (operand2, carry_opt) = base_logical(op, reg);
        let result = operand1 & operand2;

        if rd == R15 {}
        // TODO
        if set_flag {
            set_cpsr_czn(result, carry_opt, reg);
        }
    }

    pub fn eor(set_flag: bool, op: u32, reg: &mut Register) {
        let (_operand1, _rd) = base(op, reg);
        // TODO
        if set_flag {
            //set_cpsr_czn(result, carry_opt, reg);
        }
    }

    pub fn tst(op: u32, reg: &mut Register) {
        // always set flag
        let (_operand1, _rd) = base(op, reg);
        // TODO

        //set_cpsr_czn(result, carry_opt, reg);
    }

    pub fn teq(op: u32, reg: &mut Register) {
        // always set flag
        let (_operand1, _rd) = base(op, reg);
        // TODO

        //set_cpsr_czn(result, carry_opt, reg);
    }

    pub fn orr(set_flag: bool, op: u32, reg: &mut Register) {
        let (_operand1, _rd) = base(op, reg);
        // TODO
        if set_flag {
            //set_cpsr_czn(result, carry_opt, reg);
        }
    }

    pub fn mov(set_flag: bool, op: u32, reg: &mut Register) {
        let (_operand1, _rd) = base(op, reg);
        // TODO
        if set_flag {
            //set_cpsr_czn(result, carry_opt, reg);
        }
    }

    pub fn bic(set_flag: bool, op: u32, reg: &mut Register) {
        let (_operand1, _rd) = base(op, reg);
        // TODO
        if set_flag {
            //set_cpsr_czn(result, carry_opt, reg);
        }
    }

    pub fn mvn(set_flag: bool, op: u32, reg: &mut Register) {
        let (_operand1, _rd) = base(op, reg);
        // TODO
        if set_flag {
            //set_cpsr_czn(result, carry_opt, reg);
        }
    }


    // ----------------算术运算----------------

    pub fn sub(set_flag: bool, op: u32, reg: &mut Register) {
        let (_operand1, _rd) = base(op, reg);
        // TODO
        if set_flag {
            // reg.cpsr.set_flag_v();
            // reg.cpsr.set_flag_c();
            // reg.cpsr.set_flag_z();
            // reg.cpsr.set_flag_n();
        }
    }

    pub fn rsb(set_flag: bool, op: u32, reg: &mut Register) {
        let (_operand1, _rd) = base(op, reg);
        // TODO
        if set_flag {
            // reg.cpsr.set_flag_v();
            // reg.cpsr.set_flag_c();
            // reg.cpsr.set_flag_z();
            // reg.cpsr.set_flag_n();
        }
    }

    pub fn add(set_flag: bool, op: u32, reg: &mut Register) {
        let (_operand1, _rd) = base(op, reg);
        // TODO
        if set_flag {
            // reg.cpsr.set_flag_v();
            // reg.cpsr.set_flag_c();
            // reg.cpsr.set_flag_z();
            // reg.cpsr.set_flag_n();
        }
    }

    pub fn adc(set_flag: bool, op: u32, reg: &mut Register) {
        let (_operand1, _rd) = base(op, reg);
        // TODO
        if set_flag {
            // reg.cpsr.set_flag_v();
            // reg.cpsr.set_flag_c();
            // reg.cpsr.set_flag_z();
            // reg.cpsr.set_flag_n();
        }
    }

    pub fn sbc(set_flag: bool, op: u32, reg: &mut Register) {
        let (_operand1, _rd) = base(op, reg);
        // TODO
        if set_flag {
            // reg.cpsr.set_flag_v();
            // reg.cpsr.set_flag_c();
            // reg.cpsr.set_flag_z();
            // reg.cpsr.set_flag_n();
        }
    }

    pub fn rsc(set_flag: bool, op: u32, reg: &mut Register) {
        let (_operand1, _rd) = base(op, reg);
        // TODO
        if set_flag {
            // reg.cpsr.set_flag_v();
            // reg.cpsr.set_flag_c();
            // reg.cpsr.set_flag_z();
            // reg.cpsr.set_flag_n();
        }
    }

    pub fn cmp(op: u32, reg: &mut Register) {
        // always set flag
        let (_operand1, _rd) = base(op, reg);
        // TODO

        // reg.cpsr.set_flag_v();
        // reg.cpsr.set_flag_c();
        // reg.cpsr.set_flag_z();
        // reg.cpsr.set_flag_n();
    }

    pub fn cmn(op: u32, reg: &mut Register) {
        // always set flag
        let (_operand1, _rd) = base(op, reg);
        // TODO

        // reg.cpsr.set_flag_v();
        // reg.cpsr.set_flag_c();
        // reg.cpsr.set_flag_z();
        // reg.cpsr.set_flag_n();
    }
}


fn base_logical(instruct: u32, reg: &mut Register) -> (u32, Option<bool>) {
    if instruct.get_bit_bool(25) {
        // 立即数
        let imm = instruct & 0b1111_1111;
        let rotate = instruct.extract(8, 4);
        let operand2 = imm.rotate_right(rotate * 2);
        (operand2, None)
    } else {
        let (operand2, carry) = barrel_shifter(instruct, reg);
        (operand2, Some(carry))
    }
}

fn base(op: u32, reg: &mut Register) -> (u32, u8) {
    let rn = op.extract(16, 4) as u8;
    let operand1 = reg.reg_val(rn);
    let rd = op.extract(12, 4) as u8;
    (operand1, rd)
}

fn set_cpsr_czn(result: u32, carry_opt: Option<bool>, reg: &mut Register) {
    carry_opt.map(|carry| reg.cpsr.set_flag_c(carry));
    reg.cpsr.set_flag_z(result == 0);
    reg.cpsr.set_flag_n(result >> 31 == 1);
}

struct InternalBase {
    operand1: u32,
    operand2: u32,
    rd: u8,
}
