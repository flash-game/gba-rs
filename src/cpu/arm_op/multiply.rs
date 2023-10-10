use fantasy_util::bit::usize::BitUtil;

use crate::cpu::reg::Register;

pub struct Multiply {}

impl Multiply {
    ///
    ///
    pub fn execute(set_flag: bool, accumulate: bool, instruct: u32, reg: &mut Register) {
        let rm = instruct.extract(0, 4) as u8;
        let rs = instruct.extract(8, 4) as u8;
        let rn = instruct.extract(12, 4) as u8;
        let rd = instruct.extract(16, 4) as u8;
        let mut mul_result = reg.reg_val(rm).wrapping_mul(reg.reg_val(rs));
        if accumulate {
            mul_result = mul_result.wrapping_add(reg.reg_val(rn));
        }
        reg.set_reg(rd, mul_result);
        if set_flag {
            let new_n = (mul_result & 0x8000_0000) != 0;
            reg.cpsr.set_flag_nzcv(new_n, mul_result == 0, false, reg.cpsr.flag_v());
        }
    }
}

#[test]
fn multiply_test() {}