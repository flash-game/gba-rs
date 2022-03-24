use fantasy_util::bit::usize::BitUtil;

use crate::cpu::reg::Register;
use crate::util::{combine64, split64};

pub struct MultiplyLong {}

impl MultiplyLong {
    /// set_flag 是否设置cond code
    /// accumulate 是否乘加
    /// signed 是否是有符号
    pub fn execute(set_flag: bool, accumulate: bool, signed: bool, instruct: u32, reg: &mut Register) {
        let rdhi = instruct.extract(16, 4) as u8;
        let rdlo = instruct.extract(12, 4) as u8;
        let rn = instruct.extract(8, 4) as u8;
        let rm = instruct.extract(0, 4) as u8;
        let rn_val = reg.reg_val(rn);
        let rm_val = reg.reg_val(rm);
        let result = if !signed {
            let prod = (rn_val as u64) * (rm_val as u64);
            prod.wrapping_add(if !accumulate {
                0u64
            } else {
                combine64(reg.reg_val(rdhi), reg.reg_val(rdlo))
            })
        } else {
            let prod = (rn_val as i64) * (rm_val as i64);
            prod.wrapping_add(if !accumulate {
                0i64
            } else {
                combine64(reg.reg_val(rdhi), reg.reg_val(rdlo)) as i64
            }) as u64
        };
        let (rdhi_val, rdlo_val) = split64(result);
        reg.set_reg(rdhi, rdhi_val);
        reg.set_reg(rdlo, rdlo_val);
        if set_flag {
            let new_n = (result & 0x8000_0000) != 0;
            reg.cpsr.set_flag_nzcv(new_n, result == 0, false, false);
        }
    }
}
