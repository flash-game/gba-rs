use fantasy_util::bit::usize::BitUtil;

use crate::cpu::reg::Register;

pub struct DataProcess {}

impl DataProcess {
    pub fn add(_set_flag: bool, op: u32, reg: &mut Register) {
        let (_operand1, _rd) = base(op, reg);
        // TODO
    }

    pub fn teq(op: u32, reg: &mut Register) {
        let (_operand1, _rd) = base(op, reg);
        // TODO
    }

    pub fn tst(op: u32, reg: &mut Register) {
        let (_operand1, _rd) = base(op, reg);
        // TODO
    }

    pub fn cmp(op: u32, reg: &mut Register) {
        let (_operand1, _rd) = base(op, reg);
        // TODO
    }

    pub fn cmn(op: u32, reg: &mut Register) {
        let (_operand1, _rd) = base(op, reg);
        // TODO
    }

    fn base_logical(instruct: u32, reg: &mut Register) {
        let _operand2 = if instruct.get_bit_bool(25) {
            // 立即数
            let imm = instruct & 0b1111_1111;
            let rotate = instruct.extract(8, 4);
            imm.rotate_right(rotate * 2)
        } else {
            let rm = (instruct & 0b1111) as u8;
            let rm_val = reg.reg_val(rm);
            let shift_type = instruct.extract(5, 2) as u8;
            let shift_amount = if instruct.get_bit_bool(4) {
                let rs = instruct.extract(8, 4) as u8;
                reg.reg_val(rs) & 0b11111
            } else {
                instruct.extract(7, 5)
            };
            match shift_type {
                0b00 => rm_val << shift_amount,
                0b01 => rm_val >> shift_amount,
                0b10 => ((rm_val as i32) >> shift_amount) as u32,
                0b11 => rm_val.rotate_right(shift_amount),
                _ => unreachable!(),
            }
        };
    }
}

fn base(op: u32, reg: &mut Register) -> (u32, u8) {
    let rn = op.extract(16, 4) as u8;
    let operand1 = reg.reg_val(rn);
    let rd = op.extract(12, 4) as u8;
    (operand1, rd)
}

struct InternalBase {
    operand1: u32,
    operand2: u32,
    rd: u8,
}
